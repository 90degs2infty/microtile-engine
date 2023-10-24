use microtile_engine::{Board, DisplacedTile, Line, Render, RotatedTile, Rotatee};

fn print_board<const M: usize, const N: usize>(board: &Board<M, N>) {
    println!("x-----x");
    for row in (0..5).rev() {
        print!("|");

        for col in 0..5 {
            if board[row][col] {
                print!("o");
            } else {
                print!(" ");
            }
        }
        println!("|");
    }
    println!("x-----x");
}

fn main() {
    let mut board: Board<5, 5> = [[false; 5]; 5];

    let tile = DisplacedTile::new(RotatedTile::new(Line {}));
    tile.render(&mut board);

    print_board(&board);

    let tile = tile.displace_by(2, 1).rotate();
    tile.render(&mut board);
    print_board(&board);
}
