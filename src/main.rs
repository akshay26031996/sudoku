pub mod board;

fn main() {
    let mut board = board::Board::new();
    board.generate_problem(5);
    board.print_board();
    board.fill_grid();
    board.print_board();
}
