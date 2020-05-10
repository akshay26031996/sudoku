use rand::Rng;

/// Struct `Board` is the sudoku board of size 9x9
#[derive(Clone, Copy)]
pub struct Board {
    board: [[u8; 9]; 9],
}

impl Board {
    /// Constructs a new `Board`
    ///
    /// # Examples
    /// ```
    /// mod board;
    ///
    /// let mut board = board::Board::new();
    /// ```
    pub fn new() -> Self {
        Self { board: [[0; 9]; 9] }
    }
    
    /// Returns a string of 9 rows and 9 columns
    /// represening the board. 0 represents an empty
    /// cell.
    /// # Examples
    /// ```
    /// println!("{}", board.print_board().as_str());
    /// write!(&mut input_writer, "{}\n", board.print_board().as_str()).unwrap();
    /// ```
    pub fn print_board(&self) -> String {
        let mut res = String::from("");
        for i in 0..(self.board.len()) {
            for j in 0..(self.board[i].len()) {
                res.push_str(format!("{}", self.board[i][j]).as_str());
            }
            res.push_str("\n");
        }
        res
    }
    /// This method checks if the sudoku board is filled
    /// or not.
    pub fn is_filled(&self) -> bool {
        for i in 0..(self.board.len()) {
            for j in 0..(self.board.len()) {
                if self.board[i][j] == 0 {
                    return false;
                }
            }
        }
        true
    }
    /// This method fills the sudoku grid with the **random**
    /// solution.
    /// # Examples
    /// ```
    /// let mut board = board::Board::new();
    /// board.fill_grid();
    /// ```
    pub fn fill_grid(&mut self) -> bool {
        for i in 0..81 {
            let row = i / 9;
            let column = i % 9;
            if self.board[row][column] == 0 {
                let numbers = shuffled_list();
                for n in 0..numbers.len() {
                    let number = numbers[n];
                    let is_valid_number = Board::is_valid_filler(self, number, row, column);
                    if is_valid_number {
                        self.board[row][column] = number;
                        if Board::is_filled(self) {
                            return true;
                        } else {
                            if Board::fill_grid(self) {
                                return true;
                            } else {
                                self.board[row][column] = 0;
                            }
                        }
                    }
                }
                break;
            }
        }
        if Board::is_filled(self) {
            return true;
        }
        false
    }
    /// This method loops over all the possible soltions of a given
    /// sudoku board and counts the solution.
    /// # Examples
    /// ```
    /// let mut cloned_board = self.clone();
    /// let mut count = 0;
    /// Board::count_solutions(&mut cloned_board, &mut count);
    /// ```
    fn count_solutions(board: &mut Self, count: &mut u32) -> bool {
        if *count > 1u32 {
            return true;
        }
        if board.is_filled() {
            return true;
        }
        let mut row = 10;
        let mut column = 10;
        for i in 0..81 {
            row = i / 9;
            column = i % 9;
            if board.board[row][column] == 0 {
                for number in 1..10 {
                    let is_valid_number = board.is_valid_filler(number, row, column);
                    if is_valid_number {
                        board.board[row][column] = number;
                        if board.is_filled() {
                            *count = *count + 1;
                            break;
                        } else if Board::count_solutions(board, count) {
                            return true;
                        }
                    }
                }
                break;
            }
        }
        if row!=10 && column!=10 {
            board.board[row][column] = 0;
        }
        false
    }
    /// This method generates a random problem. This method takes
    /// input as number of attempts. Higher the number of attempts
    /// more difficult the problem generated would be.
    /// # Examples
    /// ```
    /// let mut board = board::Board::new();
    /// board.generate_problem(5);
    /// ```
    pub fn generate_problem(&mut self, attempts: u32) {
        let mut remaining_attempts = attempts;
        let mut rng = rand::thread_rng();
        self.fill_grid();
        while remaining_attempts > 0 {
            let mut row = rng.gen_range(0, 9);
            let mut column = rng.gen_range(0, 9);
            while self.board[row][column] == 0 {
                row = rng.gen_range(0, 9);
                column = rng.gen_range(0, 9);
            }
            let backup = self.board[row][column];
            self.board[row][column] = 0;
            let mut cloned_board = self.clone();
            let mut count = 0;
            Board::count_solutions(&mut cloned_board, &mut count);
            if count != 1 {
                self.board[row][column] = backup;
                remaining_attempts = remaining_attempts - 1;
            }
        }
    }
    /// Give the board, row and column and the input number
    /// this method checks if the input number is valid.
    /// Valid number would be such that it is not repeated in
    /// row, column and block. For more details please refere
    /// Sudoku rules.
    pub fn is_valid_filler(&self, number: u8, row: usize, column: usize) -> bool {
        let mut is_present_row = false;
        let mut is_present_col = false;
        let mut is_present_block = false;
        for i in 0..9 {
            if i == column {
                continue;
            }
            let r = self.board[row][i];
            if r == number {
                is_present_row = true;
                break;
            }
        }
        if is_present_row {
            return false;
        }
        for i in 0..9 {
            if i == row {
                continue;
            }
            let c = self.board[i][column];
            if c == number {
                is_present_col = true;
                break;
            }
        }
        if is_present_col {
            return false;
        }
        let b_i = row / 3;
        let b_j = column / 3;
        for i in 0..3 {
            for j in 0..3 {
                let x = (b_i * 3) + i;
                let y = (b_j * 3) + j;
                if x == row && y == column {
                    continue;
                }
                let b = self.board[x][y];
                if b == number {
                    is_present_block = true;
                    break;
                }
            }
        }
        if is_present_block {
            return false;
        }
        true
    }
}

fn shuffled_list() -> [u8; 9] {
    let mut shuffled_list = [0; 9];
    let mut rng = rand::thread_rng();
    for i in 0..9 {
        let mut r = rng.gen_range(1, 10);
        let mut is_present = true;
        while is_present {
            // println!("Generated random number: {}", r);
            let mut temp = false;
            for j in 0..9 {
                if r == shuffled_list[j] {
                    temp = true;
                    break;
                }
            }
            if !temp {
                is_present = false;
            } else {
                r = rng.gen_range(1, 10);
            }
        }
        shuffled_list[i] = r;
    }
    shuffled_list
}

#[test]
fn check_valid_board() {
    let mut board = Board::new();
    board.fill_grid();
    // board.print_board();
    for i in 0..board.board.len() {
        for j in 0..board.board[i].len() {
            let number = board.board[i][j];
            assert!(
                board.is_valid_filler(number, i, j),
                "number: {}, row: {}, column: {}",
                number,
                i,
                j
            );
        }
    }
}
