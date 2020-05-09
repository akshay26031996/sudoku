pub mod board;

fn main() {
    let mut board = board::Board::new();
    board.fill_grid();
    board.print_board();
}
