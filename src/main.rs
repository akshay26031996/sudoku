// use std::{
//     fs::File,
//     io::{BufWriter, Write},
// };
mod board;

fn main() {
    // let input_file = File::create("/home/akshay/workspace/python/input.txt").unwrap();
    // let mut input_writer = BufWriter::new(&input_file);
    // let output_file = File::create("/home/akshay/workspace/python/output.txt").unwrap();
    // let mut output_writer = BufWriter::new(&output_file);
    // for _i in 0..10 {
    let mut board = board::Board::new();
    board.generate_problem(5);
    // write!(&mut input_writer, "{}\n", board.print_board().as_str()).unwrap();
    print!("{}", board.print_board().as_str());
    board.fill_grid();
    // write!(&mut output_writer, "{}\n", board.print_board().as_str()).unwrap();
    println!("#########");
    print!("{}", board.print_board().as_str());
    //}
}
