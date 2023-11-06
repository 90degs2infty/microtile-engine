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
    let game = Game::default();

    // Tile 1 - Line
    println!("Tile 1 - Line");
    let tile = BasicTile::Line;
    let active = [
        [true, true, false, false, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];
    let passive = [
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];

    let mut game = place_tile_continue(game, tile)?;
    game.rotate_tile();
    game.move_tile_up_to(1);
    let game = push_tile_down(game, 3)?;
    check_snapshots(&game, &active, &passive);
    let game = process_rows(game, 5)?;

    // Tile 2 - Line
    println!("Tile 2 - Line");
    let tile = BasicTile::Line;
    let active = [
        [true, true, false, true, true],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];
    let passive = [
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];

    let mut game = place_tile_continue(game, tile)?;
    game.rotate_tile();
    game.move_tile_up_to(5);
    let game = push_tile_down(game, 3)?;
    check_snapshots(&game, &active, &passive);
    let game = process_rows(game, 5)?;

    // Tile 3 - Diagonal
    println!("Tile 3 - Diagonal");
    let tile = BasicTile::Diagonal;
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

    let game = place_tile_continue(game, tile)?;
    let game = push_tile_down(game, 3)?;
    check_snapshots(&game, &active, &passive);
    let game = process_rows(game, 6)?;

    // Tile 4 - Square
    println!("Tile 4 - Square");
    let tile = BasicTile::Square;
    let active = [
        [false, false, true, true, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];
    let passive = [
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];

    let game = place_tile_continue(game, tile)?;
    let game = descend_tile_no_processing(game)?;
    let game = descend_tile_no_processing(game)?;
    let game = descend_tile_no_processing(game)?;
    let mut game = descend_tile_no_processing(game)?;
    game.move_tile_up_to(5);
    let game = push_tile_down(game, 0)?;
    check_snapshots(&game, &active, &passive);
    let game = process_rows(game, 5)?;

    // Tile 5 - Line
    println!("Tile 5 - Line");
    let tile = BasicTile::Line;
    let active = [
        [true, true, true, true, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];
    let passive = [
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];

    let mut game = place_tile_continue(game, tile)?;
    game.rotate_tile();
    game.move_tile_up_to(0);
    let game = push_tile_down(game, 3)?;
    check_snapshots(&game, &active, &passive);
    let game = process_rows(game, 5)?;

    // Tile 6 - Line
    println!("Tile 6 - Line");
    let tile = BasicTile::Line;
    let active = [
        [true, true, true, true, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];
    let passive = [
        [false; BOARD_COLS],
        [true, true, false, false, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];

    let mut game = place_tile_continue(game, tile)?;
    game.rotate_tile();
    game.move_tile_up_to(0);
    let game = push_tile_down(game, 2)?;
    check_snapshots(&game, &active, &passive);
    let game = process_rows(game, 5)?;

    // Tile 7 - Line
    println!("Tile 7 - Line");
    let tile = BasicTile::Line;
    let active = [
        [true, true, true, true, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];
    let passive = [
        [false; BOARD_COLS],
        [true, true, true, true, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];

    let mut game = place_tile_continue(game, tile)?;
    game.rotate_tile();
    game.move_tile_up_to(4);
    let game = push_tile_down(game, 2)?;
    check_snapshots(&game, &active, &passive);
    let game = process_rows(game, 5)?;

    // Tile 8 - Line
    println!("Tile 8 - Line");
    let tile = BasicTile::Line;
    let active = [
        [true, true, true, true, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];
    let passive = [
        [false; BOARD_COLS],
        [true, true, true, true, false],
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false; BOARD_COLS],
    ];

    let game = place_tile_continue(game, tile)?;
    let game = push_tile_down(game, 1)?;
    check_snapshots(&game, &active, &passive);
    let game = process_rows(game, 5)?;

    // Tile 9 - Square
    println!("Tile 9 - Square");
    let tile = BasicTile::Square;
    let active = [
        [true, true, true, true, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];
    let passive = [
        [false; BOARD_COLS],
        [true, true, true, true, false],
        [false, true, true, false, false],
        [false, false, true, false, false],
        [false; BOARD_COLS],
    ];

    let mut game = place_tile_continue(game, tile)?;
    game.move_tile_up_to(2);
    let mut game = descend_tile_no_processing(game)?;
    game.move_tile_up_to(5);
    let game = push_tile_down(game, 1)?;
    check_snapshots(&game, &active, &passive);
    let game = process_rows(game, 5)?;

    // Tile 10 - Square
    println!("Tile 10 - Square");
    let tile = BasicTile::Square;
    let active = [
        [true, true, true, true, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];
    let passive = [
        [false; BOARD_COLS],
        [true, true, true, true, false],
        [true, true, true, false, false],
        [false, false, true, false, false],
        [false; BOARD_COLS],
    ];

    let mut game = place_tile_continue(game, tile)?;
    game.move_tile_up_to(1);
    let game = push_tile_down(game, 2)?;
    check_snapshots(&game, &active, &passive);
    let game = process_rows(game, 5)?;

    // Tile 11 - Square
    println!("Tile 11 - Square");
    let tile = BasicTile::Square;
    let active = [
        [true, true, true, true, true],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];
    let passive = [
        [false; BOARD_COLS],
        [true, true, true, true, false],
        [true, true, true, false, false],
        [false, false, true, false, false],
        [false; BOARD_COLS],
    ];

    let mut game = place_tile_continue(game, tile)?;
    game.move_tile_up_to(5);
    let game = push_tile_down(game, 4)?;
    check_snapshots(&game, &active, &passive);
    let game = process_rows(game, 6)?;

    // Tile 12 - Diagonal
    println!("Tile 12 - Diagonal");
    let tile = BasicTile::Diagonal;
    let active = [
        [true, true, true, true, true],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];
    let passive = [
        [false; BOARD_COLS],
        [true, true, true, true, false],
        [false, false, true, false, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];

    let mut game = place_tile_continue(game, tile)?;
    game.rotate_tile();
    game.move_tile_up_to(5);
    let game = push_tile_down(game, 3)?;
    check_snapshots(&game, &active, &passive);
    let game = process_rows(game, 6)?;

    // Tile 13 - Line
    println!("Tile 13 - Line");
    let tile = BasicTile::Line;
    let active = [
        [true, true, true, true, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];
    let passive = [
        [false; BOARD_COLS],
        [false, false, true, true, true],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];

    let mut game = place_tile_continue(game, tile)?;
    game.rotate_tile();
    game.rotate_tile();
    game.rotate_tile();
    game.move_tile_up_to(5);
    let game = push_tile_down(game, 2)?;
    check_snapshots(&game, &active, &passive);
    let game = process_rows(game, 5)?;

    // Tile 14 - Square
    println!("Tile 14 - Square");
    let tile = BasicTile::Square;
    let active = [
        [true, true, true, true, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];
    let passive = [
        [false; BOARD_COLS],
        [true, false, true, true, true],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];

    let game = place_tile_continue(game, tile)?;
    let mut game = descend_tile_no_processing(game)?;
    game.move_tile_up_to(1);
    let game = push_tile_down(game, 2)?;
    check_snapshots(&game, &active, &passive);
    let game = process_rows(game, 5)?;

    // Tile 15 - Diagonal
    println!("Tile 15 - Diagonal");
    let tile = BasicTile::Diagonal;
    let active = [
        [true, true, true, true, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];
    let passive = [
        [false; BOARD_COLS],
        [true, true, true, true, true],
        [true, false, false, false, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];

    let mut game = place_tile_continue(game, tile)?;
    game.move_tile_up_to(2);
    game.rotate_tile();
    let game = push_tile_down(game, 2)?;
    check_snapshots(&game, &active, &passive);
    let game = process_rows(game, 6)?;

    // Tile 16 - Line
    println!("Tile 16 - Line");
    let tile = BasicTile::Line;
    let active = [
        [true, true, true, true, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];
    let passive = [
        [false; BOARD_COLS],
        [true, true, true, false, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];

    let mut game = place_tile_continue(game, tile)?;
    game.move_tile_up_to(1);
    game.rotate_tile();
    game.move_tile_up_to(2);
    game.rotate_tile();
    game.move_tile_up_to(3);
    let game = push_tile_down(game, 2)?;
    check_snapshots(&game, &active, &passive);
    let game = process_rows(game, 5)?;

    // Tile 17 - Square
    println!("Tile 17 - Square");
    let tile = BasicTile::Square;
    let active = [
        [true, true, true, true, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];
    let passive = [
        [false; BOARD_COLS],
        [true, true, true, true, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];

    let mut game = place_tile_continue(game, tile)?;
    game.move_tile_up_to(4);
    let game = push_tile_down(game, 3)?;
    check_snapshots(&game, &active, &passive);
    let game = process_rows(game, 5)?;

    // Tile 18 - Line
    println!("Tile 18 - Line");
    let tile = BasicTile::Line;
    let active = [
        [true, true, true, true, true],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];
    let passive = [
        [false; BOARD_COLS],
        [true, true, true, true, true],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];

    let mut game = place_tile_continue(game, tile)?;
    game.rotate_tile();
    game.rotate_tile();
    game.rotate_tile();
    game.rotate_tile();
    game.move_tile_up_to(5);
    let game = push_tile_down(game, 3)?;
    check_snapshots(&game, &active, &passive);
    let game = process_rows(game, 7)?;

    // Tile 19 - Line
    println!("Tile 19 - Line");
    let tile = BasicTile::Line;
    let active = [
        [false, false, true, false, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];
    let passive = [
        [false; BOARD_COLS],
        [false, false, true, false, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];

    let game = place_tile_continue(game, tile)?;
    let game = push_tile_down(game, 3)?;
    check_snapshots(&game, &active, &passive);
    let game = process_rows(game, 5)?;

    // Tile 20 - Line
    println!("Tile 20 - Line");
    let tile = BasicTile::Line;
    let active = [
        [false, false, true, false, false],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
        [false; BOARD_COLS],
    ];
    let passive = [
        [false; BOARD_COLS],
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false, false, true, false, false],
        [false; BOARD_COLS],
    ];

    let game = place_tile_continue(game, tile)?;
    let game = push_tile_down(game, 1)?;
    check_snapshots(&game, &active, &passive);
    let game = process_rows(game, 5)?;

    // Tile 21 - Line
    println!("Tile 21 - Line");
    let tile = BasicTile::Line;

    let _ = place_tile_over(game, tile)?;

    Ok(())
}
