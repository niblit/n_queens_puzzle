use std::collections::HashSet;
pub fn solve_n_queens_puzzle(n: usize) -> Vec<String> {
    let placement: Vec<usize> = (0..n).collect();
    let placement_permutations = permutations(&placement);
    let mut solution_strings = Vec::new();

    for current_try in placement_permutations {
        let mut positions: HashSet<BoardCoordinates> = HashSet::new();
        for &index in &placement {
            if let Some(coordinates) =
                BoardCoordinates::new(current_try[index] as isize, index as isize, n)
            {
                positions.insert(coordinates);
            }
        }
        if is_valid_solution(&positions) {
            let mut solution_string = positions.iter().map(|coordinate| {coordinate.to_notation()}).collect::<Vec<String>>();
            solution_string.sort_unstable();
            let solution_string = solution_string.join(" ");

            solution_strings.push(solution_string);
        }
    }
    solution_strings
}

#[derive(Hash, PartialEq, Eq)]
pub struct BoardCoordinates {
    row: usize,
    col: usize,
    n: usize
}

impl BoardCoordinates {
    pub fn new(row: isize, col: isize, n: usize) -> Option<Self> {
        if row < 0 || col < 0 {
            return None;
        }
        let row = row as usize;
        let col = col as usize;
        if (0..n).contains(&row) && (0..n).contains(&col) {
            return Some(Self { row, col , n});
        }
        None
    }

    pub fn to_notation(&self) -> String {
        if self.n == 8 {
            let cols = ["a", "b", "c", "d", "e", "f", "g", "h"];

            format!("{}{}", cols[self.col], self.row + 1)
        }
        else {
            format!("{}{}", self.col + 1, self.row + 1)
        }
    }
}

pub fn is_valid_solution(position: &HashSet<BoardCoordinates>) -> bool {
    for queen in position {
        let queen_moves = generate_destinations(queen);
        for queen_move in queen_moves {
            if position.contains(&queen_move) {
                return false;
            }
        }
    }
    true
}

fn generate_destinations(start: &BoardCoordinates) -> Vec<BoardCoordinates> {
    const DIRECTIONS: [[isize; 2]; 8] = [
        [-1, 0],
        [-1, 1],
        [0, 1],
        [1, 1],
        [1, 0],
        [1, -1],
        [0, -1],
        [-1, -1],
    ];
    let mut result = Vec::new();

    for direction in DIRECTIONS {
        for multiplier in 1..start.n {
            let multiplier = multiplier as isize;
            if let Some(coordinates) = BoardCoordinates::new(
                start.row as isize + direction[0] * multiplier,
                start.col as isize + direction[1] * multiplier,
                start.n
            ) {
                result.push(coordinates)
            } else {
                break;
            }
        }
    }

    result
}

pub fn permutations<T: Clone>(elements: &[T]) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    permute(
        elements,
        0,
        &mut Vec::new(),
        &mut HashSet::new(),
        &mut result,
    );
    result
}

fn permute<T: Clone>(
    elements: &[T],
    index: usize,
    current_perm: &mut Vec<T>,
    used_indices: &mut HashSet<usize>,
    result: &mut Vec<Vec<T>>,
) {
    if index == elements.len() {
        result.push(current_perm.clone());
        return;
    }

    for i in 0..elements.len() {
        if !used_indices.contains(&i) {
            current_perm.push(elements[i].clone());
            used_indices.insert(i);
            permute(elements, index + 1, current_perm, used_indices, result);
            current_perm.pop();
            used_indices.remove(&i);
        }
    }
}
