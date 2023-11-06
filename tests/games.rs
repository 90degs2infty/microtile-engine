use anyhow::{bail, Ok, Result};
use either::Either;
use microtile_engine::{
    gameplay::{Game, Over, ProcessRows, TileFloating, TileNeeded},
    geometry::{
        board::{BOARD_COLS, BOARD_ROWS},
        tile::BasicTile,
    },
    rendering::{Active, Passive, Rendering},
};

fn place_tile_continue(game: Game<TileNeeded>, tile: BasicTile) -> Result<Game<TileFloating>> {
    match game.place_tile(tile) {
        Either::Left(game) => Ok(game),
        Either::Right(_) => bail!("Game should not have ended by placing this tile"),
    }
}

fn place_tile_over(game: Game<TileNeeded>, tile: BasicTile) -> Result<Game<Over>> {
    match game.place_tile(tile) {
        Either::Left(_) => bail!("Game should have ended by placing this tile"),
        Either::Right(game) => Ok(game),
    }
}

fn descend_tile_no_processing(game: Game<TileFloating>) -> Result<Game<TileFloating>> {
    match game.descend_tile() {
        Either::Left(game) => Ok(game),
        Either::Right(_) => bail!("Game entered `ProcessRows` state too fast"),
    }
}

fn descend_tile_processing(game: Game<TileFloating>) -> Result<Game<ProcessRows>> {
    match game.descend_tile() {
        Either::Left(_) => bail!("Game did not recognize `ProcessRows` state"),
        Either::Right(game) => Ok(game),
    }
}

fn push_tile_down(mut game: Game<TileFloating>, num_steps: usize) -> Result<Game<ProcessRows>> {
    for _ in 0..num_steps {
        game = descend_tile_no_processing(game)?;
    }
    descend_tile_processing(game)
}

fn process_rows(mut game: Game<ProcessRows>, num_iter: usize) -> Result<Game<TileNeeded>> {
    for _ in 0..num_iter {
        game = match game.process_row() {
            Either::Left(game) => game,
            Either::Right(_) => bail!("Game did not process all rows"),
        }
    }

    match game.process_row() {
        Either::Left(_) => bail!("Game did not leave `ProcessRows` state"),
        Either::Right(game) => Ok(game),
    }
}

fn check_snapshot<T>(game: &Game<ProcessRows>, expected: &[[bool; BOARD_COLS]; BOARD_ROWS])
where
    Game<ProcessRows>: Rendering<BOARD_ROWS, BOARD_COLS, T>,
{
    let mut render_buf = [[false; 5]; 5];

    <Game<ProcessRows> as Rendering<BOARD_COLS, BOARD_ROWS, T>>::render_buf(&game, &mut render_buf);

    assert_eq!(render_buf, *expected);
}

fn check_snapshots(
    game: &Game<ProcessRows>,
    active: &[[bool; BOARD_COLS]; BOARD_ROWS],
    passive: &[[bool; BOARD_COLS]; BOARD_ROWS],
) {
    check_snapshot::<Active>(&game, &active);
    check_snapshot::<Passive>(&game, &passive);
}

#[test]
fn game_one() -> Result<()> {
    let mut tiles: Vec<BasicTile> = vec![
        BasicTile::Line,
        BasicTile::Line,
        BasicTile::Diagonal,
        BasicTile::Square,
        BasicTile::Line,
        BasicTile::Line,
        BasicTile::Line,
        BasicTile::Line,
        BasicTile::Square,
        BasicTile::Square,
        BasicTile::Square,
        BasicTile::Diagonal,
        BasicTile::Line,
        BasicTile::Square,
        BasicTile::Line,
        BasicTile::Line,
        BasicTile::Line,
        BasicTile::Line,
    ];
    tiles.reverse();

    let mut snapshots = vec![
        [
            [true, true, false, false, false],
            [false; BOARD_COLS],
            [false; BOARD_COLS],
            [false; BOARD_COLS],
            [false; BOARD_COLS],
        ],
        [
            [true, true, false, true, true],
            [false; BOARD_COLS],
            [false; BOARD_COLS],
            [false; BOARD_COLS],
            [false; BOARD_COLS],
        ],
    ];
    snapshots.reverse();

    let game = Game::default();

    let mut game = place_tile_continue(game, tiles.pop().unwrap())?;

    game.rotate_tile();
    game.move_tile_up_to(1);
    let game = push_tile_down(game, 3)?;

    check_snapshot::<Active>(&game, &snapshots.pop().unwrap());
    let game = process_rows(game, 5)?;

    // Tile 2
    let mut game = place_tile_continue(game, tiles.pop().unwrap())?;
    game.rotate_tile();
    game.move_tile_up_to(5);
    let game = push_tile_down(game, 3)?;
    check_snapshot::<Active>(&game, &snapshots.pop().unwrap());
    let game = process_rows(game, 5)?;

    // Tile diagonal
    let game = place_tile_continue(game, BasicTile::Diagonal)?;
    let game = push_tile_down(game, 3)?;

    let active = [
        [true; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];

    let passive = [
        [false; BOARD_COLS],
        [false, false, false, true, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];
    check_snapshots(&game, &active, &passive);
    let game = process_rows(game, 6)?;

    Ok(())
}
