use anyhow::{bail, Ok, Result};
use either::Either;
use microtile_engine::{
    gameplay::{Game, ProcessRows, TileFloating, TileNeeded},
    geometry::{board::BOARD_COLS, tile::BasicTile},
    rendering::Rendering,
};

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

    let mut snapshots = vec![[
        [false; BOARD_COLS],
        [true, true, false, false, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ]];
    snapshots.reverse();

    let mut render_buf = [[false; 5]; 5];

    let game = Game::default();

    let mut game = match game.place_tile(tiles.pop().unwrap()) {
        Either::Left(game) => game,
        Either::Right(_) => bail!("Game should not have ended after first tile!"),
    };

    game.rotate_tile();

    game.render_buf(&mut render_buf);
    let expected = snapshots.pop().unwrap();

    assert_eq!(render_buf, expected);

    let game = descend_tile_no_processing(game)?;
    let game = descend_tile_no_processing(game)?;
    let game = descend_tile_no_processing(game)?;
    let game = descend_tile_processing(game)?;

    let _game = process_rows(game, 5)?;

    Ok(())
}
