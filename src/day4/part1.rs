use std::ops::AddAssign;
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use crate::utils::read_input;

const WORD: &str = "XMAS";

pub fn to_usize_coords(coords: &(BigInt, BigInt)) -> (usize, usize) {
    (coords.0.to_usize().unwrap(), coords.1.to_usize().unwrap())
}

pub fn out_of_bounds(board: &Vec<Vec<char>>, coords: &(BigInt, BigInt)) -> bool {
    !((BigInt::ZERO..BigInt::from(board.len())).contains(&coords.0) &&
        (BigInt::ZERO..BigInt::from(board[0].len())).contains(&coords.1))
}

pub fn travel_through_neighbors(
    board: &Vec<Vec<char>>,
    coords: &(BigInt, BigInt),
    direction: &(BigInt, BigInt),
    progression: usize
) -> bool {
    if progression > WORD.len() - 1 {
        return true;
    }
    if out_of_bounds(board, coords) {
        return false;
    }
    let usized_coords: (usize, usize) = to_usize_coords(coords);
    if board[usized_coords.0][usized_coords.1] == WORD.chars().nth(progression).unwrap() {
        return travel_through_neighbors(
            &board,
            &(&coords.0 + &direction.0, &coords.1 + &direction.1),
            direction,
            progression + 1
        );
    }
    false
}

pub fn start_analysis(board: &Vec<Vec<char>>, i: usize, j: usize) -> BigInt {
    let coords: &(BigInt, BigInt) = &(BigInt::from(i), BigInt::from(j));
    let one: BigInt = BigInt::from(1);

    let mut count: BigInt = BigInt::ZERO;

    let permutations: Vec<(BigInt, BigInt)> = vec![
        (&coords.0 + &one, &coords.1 + &one),
        (&coords.0 + &one, &coords.1 - &one),
        (&coords.0 - &one, &coords.1 + &one),
        (&coords.0 - &one, &coords.1 - &one),
        (&coords.0 + &one, coords.1.clone()),
        (&coords.0 - &one, coords.1.clone()),
        (coords.0.clone(), &coords.1 + &one),
        (coords.0.clone(), &coords.1 - &one),
    ]
        .into_iter()
        .filter(|x| {!out_of_bounds(&board, x)})
        .collect();

    for perm in permutations {
        let usize_coords = to_usize_coords(&perm);
        if board[usize_coords.0][usize_coords.1] == WORD.chars().nth(1).unwrap() {
            let result: bool = travel_through_neighbors(
                &board,
                &perm,
                &(&perm.0 - &coords.0, &perm.1 - &coords.1),
                1
            );
            if result {
                count.add_assign(1)
            }
        }
    }
    count
}

pub fn compute() {
    let content: String = read_input(4);
    let mut board: Vec<Vec<char>> = Vec::with_capacity(content.lines().count());
    for line in content.lines() {
        let mut row: Vec<char> = Vec::with_capacity(line.len());
        for content_char in line.chars() {
            row.push(content_char);
        }
        board.push(row);
    }
    let mut sum: BigInt = BigInt::ZERO;

    for (i, line) in content.lines().enumerate() {
        for (j, content_char) in line.chars().enumerate() {
            if content_char == WORD.chars().nth(0).unwrap() {
                sum += start_analysis(&board, i, j);
            }
        }
    }

    println!("{}", sum);
}