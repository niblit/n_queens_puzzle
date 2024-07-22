use n_queens_puzzle::solve_n_queens_puzzle;

const BOARD_SIZE: usize = 8;

fn main() {
    println!("N Queens Puzzle");
    println!("Board size: {}", BOARD_SIZE);

    let solved_positions = solve_n_queens_puzzle(BOARD_SIZE);

    for positions in solved_positions.chunks(4) {
        println!("{}", positions.join(" | "));
    }

    println!("Total number of solutions: {}", &solved_positions.len());
}
