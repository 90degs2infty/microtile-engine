use crate::{
    gameplay::{
        board::{Board, ProcessesRows as BoardProcesses, TakesTile, BOARD_COLS, BOARD_ROWS},
        raster::{Active, Passive, Rasterization, RasterizationExt},
    },
    geometry::{
        grid::{ExtGrid, Grid},
        tile::{BasicTile, Dimensionee, DisplacedTile, Displacee, RotatedTile, Rotatee},
    },
};

use either::Either;

mod sealed {
    pub trait Seal {}
}

pub trait State: sealed::Seal {}

#[derive(Debug)]
pub struct TileNeeded {
    board: Board<TakesTile>,
}

impl sealed::Seal for TileNeeded {}
impl State for TileNeeded {}

impl TileNeeded {
    #[must_use]
    fn new(board: Board<TakesTile>) -> Self {
        Self { board }
    }
}

impl Default for TileNeeded {
    fn default() -> Self {
        Self::new(Board::default())
    }
}

#[derive(Debug)]
pub struct TileFloating {
    tile: DisplacedTile<RotatedTile<BasicTile>>,
    board: Board<TakesTile>,
}

impl TileFloating {
    fn new(tile: DisplacedTile<RotatedTile<BasicTile>>, board: Board<TakesTile>) -> Self {
        Self { tile, board }
    }
}

impl sealed::Seal for TileFloating {}
impl State for TileFloating {}

#[derive(Debug)]
pub struct ProcessRows {
    board: Board<BoardProcesses>,
}

impl ProcessRows {
    fn new(board: Board<BoardProcesses>) -> Self {
        Self { board }
    }
}

impl sealed::Seal for ProcessRows {}
impl State for ProcessRows {}

#[derive(Debug)]
pub struct Over {
    board: Board<TakesTile>,
}

impl Over {
    fn new(board: Board<TakesTile>) -> Self {
        Self { board }
    }
}

impl sealed::Seal for Over {}
impl State for Over {}

pub trait Observer {
    fn signal_board_changed(&self, active: Grid, passive: Grid);
}

pub struct NoopObserver;

impl Observer for NoopObserver {
    fn signal_board_changed(&self, _: Grid, _: Grid) {}
}

#[derive(Debug)]
pub enum GameError {
    ObserverFull,
    ObserverEmpty,
    InvalidMove,
}

#[derive(Debug)]
pub struct Game<S, O> {
    s: S,
    observer: Option<O>,
}

impl<O> Game<TileNeeded, O> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            s: TileNeeded::default(),
            observer: None,
        }
    }
}

pub trait SignalSource: sealed::Seal {
    fn signal_board_changed(&self);
}

impl<S, O> sealed::Seal for Game<S, O>
where
    O: Observer,
    Self: RasterizationExt<Active> + RasterizationExt<Passive>,
{
}

impl<S, O> SignalSource for Game<S, O>
where
    O: Observer,
    Self: RasterizationExt<Active> + RasterizationExt<Passive>,
{
    fn signal_board_changed(&self) {
        if let Some(o) = self.observer.as_ref() {
            o.signal_board_changed(
                <Self as RasterizationExt<Active>>::rasterize(self),
                <Self as RasterizationExt<Passive>>::rasterize(self),
            );
        }
    }
}

impl<O> sealed::Seal for Game<TileNeeded, O> where O: Observer {}
impl<O> SignalSource for Game<TileNeeded, O>
where
    O: Observer,
{
    fn signal_board_changed(&self) {
        if let Some(o) = self.observer.as_ref() {
            o.signal_board_changed(
                Grid::default(),
                <Self as RasterizationExt<Passive>>::rasterize(self),
            );
        }
    }
}

impl<O> sealed::Seal for Game<Over, O> where O: Observer {}
impl<O> SignalSource for Game<Over, O>
where
    O: Observer,
{
    fn signal_board_changed(&self) {
        if let Some(o) = self.observer.as_ref() {
            o.signal_board_changed(
                Grid::default(),
                <Self as RasterizationExt<Passive>>::rasterize(self),
            );
        }
    }
}

impl<S, O> Game<S, O>
where
    O: Observer,
    Self: SignalSource,
{
    #[must_use]
    fn new_with_signal(s: S, o: Option<O>) -> Self {
        let game = Self { s, observer: o };
        game.signal_board_changed();
        game
    }
}

impl<O> Game<TileNeeded, O>
where
    O: Observer,
{
    #[must_use]
    pub fn place_tile(self, tile: BasicTile) -> Either<Game<TileFloating, O>, Game<Over, O>> {
        let (_, height) = tile.dimensions();
        let tile = DisplacedTile::new(RotatedTile::new(tile)).displace_by(
            ((BOARD_COLS >> 1) + 1).try_into().unwrap(), // >> 1 == / 2
            (BOARD_ROWS - height + 1).try_into().unwrap(),
        );
        if self.s.board.is_position_valid(&tile) {
            Either::Left(Game::new_with_signal(
                TileFloating::new(tile, self.s.board),
                self.observer,
            ))
        } else {
            Either::Right(Game {
                s: Over::new(self.s.board),
                observer: self.observer,
            })
        }
    }
}

impl<O> Default for Game<TileNeeded, O> {
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, O> Game<T, O> {
    pub fn set_observer(&mut self, observer: O) -> Result<(), GameError> {
        if self.observer.is_some() {
            Err(GameError::ObserverFull)
        } else {
            self.observer = Some(observer);
            Ok(())
        }
    }

    pub fn clear_observer(&mut self) -> Result<O, GameError> {
        self.observer.take().ok_or(GameError::ObserverEmpty)
    }
}

impl<O> Game<TileFloating, O>
where
    O: Observer,
{
    #[must_use]
    pub fn descend_tile(self) -> Either<Game<TileFloating, O>, Game<ProcessRows, O>> {
        let candidate = self.s.tile.clone().displace_by(0, -1);

        if self.s.board.is_position_valid(&candidate) {
            Either::Left(Game::new_with_signal(
                TileFloating::new(candidate, self.s.board),
                self.observer,
            ))
        } else {
            let board = self.s.board.freeze_tile(self.s.tile).unwrap();
            Either::Right(Game::new_with_signal(
                ProcessRows::new(board),
                self.observer,
            ))
        }
    }

    pub fn tile_column(&self) -> u8 {
        (*self.s.tile.displ_x() - 1)
            .try_into()
            .expect("Column should be in range 0 to 4")
    }

    /// Tries to move the tile horizontally to `column`.
    ///
    /// If moving the tile to `column` is not valid, the tile is moved as far as possible.
    ///
    /// **Caution:** the column is counted 1-indexed here!
    ///
    /// # Panics
    ///
    /// If specified column cannot be converted to an `i32`, i.e. if
    /// `let _ : i32 = column.try_into().unwrap()` panics.
    pub fn move_tile_up_to(&mut self, column: u32) {
        let column: i32 = column.try_into().unwrap();
        let mut direction = (column - self.s.tile.displ_x()).signum();
        let mut candidate = self.s.tile.clone().displace_by(direction, 0);
        let mut changed = false;

        while direction != 0 && self.s.board.is_position_valid(&candidate) {
            self.s.tile = candidate;
            changed = true;

            direction = (column - self.s.tile.displ_x()).signum();
            candidate = self.s.tile.clone().displace_by(direction, 0);
        }
        if changed {
            self.signal_board_changed();
        }
    }

    pub fn rotate_tile(&mut self) -> Result<(), GameError> {
        let candidate = self.s.tile.clone().rotate_ccw();

        if self.s.board.is_position_valid(&candidate) {
            self.s.tile = candidate;
            self.signal_board_changed();
            Ok(())
        } else {
            Err(GameError::InvalidMove)
        }
    }
}

impl<O> Game<ProcessRows, O>
where
    O: Observer,
{
    #[must_use]
    pub fn process_row(self) -> Either<Game<ProcessRows, O>, Game<TileNeeded, O>> {
        match self.s.board.process_row() {
            Either::Left(board) => Either::Left(Game::new_with_signal(
                ProcessRows::new(board),
                self.observer,
            )),
            Either::Right(board) => {
                Either::Right(Game::new_with_signal(TileNeeded::new(board), self.observer))
            }
        }
    }
}

impl<O> Rasterization<Passive> for Game<TileNeeded, O> {
    fn rasterize_buf(&self, out: &mut Grid) {
        self.s.board.rasterize_buf(out);
    }
}

impl<O> Rasterization<Passive> for Game<TileFloating, O> {
    fn rasterize_buf(&self, out: &mut Grid) {
        self.s.board.rasterize_buf(out);
    }
}

impl<O> Rasterization<Active> for Game<TileFloating, O> {
    fn rasterize_buf(&self, out: &mut Grid) {
        *out = match ExtGrid::try_from(&self.s.tile) {
            Ok(grid) => grid.center(),
            _ => Grid::default(),
        }
    }
}

impl<O> Rasterization<Passive> for Game<ProcessRows, O> {
    fn rasterize_buf(&self, out: &mut Grid) {
        <Board<BoardProcesses> as Rasterization<Passive>>::rasterize_buf(&self.s.board, out);
    }
}

impl<O> Rasterization<Active> for Game<ProcessRows, O> {
    fn rasterize_buf(&self, out: &mut Grid) {
        <Board<BoardProcesses> as Rasterization<Active>>::rasterize_buf(&self.s.board, out);
    }
}

impl<O> Rasterization<Passive> for Game<Over, O> {
    fn rasterize_buf(&self, out: &mut Grid) {
        self.s.board.rasterize_buf(out);
    }
}
