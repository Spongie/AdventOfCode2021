use std::collections::HashSet;

struct BingoSquare {
    number: i32,
    hit: bool,
}

impl BingoSquare {
    fn new(number: i32) -> BingoSquare {
        BingoSquare {
            number: number,
            hit: false,
        }
    }
}

struct BingoBoard {
    numbers: Vec<Vec<BingoSquare>>,
}

impl BingoBoard {
    fn new() -> BingoBoard {
        BingoBoard {
            numbers: Vec::new(),
        }
    }
}

fn parse_bingo_numbers(line: &str) -> Vec<i32> {
    line.split(",")
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn parse_bingo_board(lines: Vec<&str>) -> BingoBoard {
    let mut board = BingoBoard::new();

    for line in lines {
        let row = line
            .split(" ")
            .into_iter()
            .filter_map(|x| x.trim().parse::<i32>().ok())
            .map(|x| BingoSquare::new(x))
            .collect::<Vec<BingoSquare>>();

        board.numbers.push(row);
    }

    board
}

fn has_winner(boards: &Vec<BingoBoard>) -> Vec<usize> {
    let mut index = 0;
    let mut winner_indexes = Vec::new();

    for board in boards {
        let mut is_winner = false;

        for row in &board.numbers {
            if row.iter().filter(|x| x.hit).count() == row.len() {
                winner_indexes.push(index);
                is_winner = true;
                break;
            }
        }

        if is_winner {
            index += 1;
            continue;
        }

        let mut column = 0;

        while column < board.numbers.len() {
            if board.numbers.iter().filter(|z| z[column].hit).count() == board.numbers.len() {
                winner_indexes.push(index);
                break;
            }
            column += 1;
        }
        index += 1;
    }
    winner_indexes
}

pub fn solve_1() {
    let input = include_str!("../input/input4.txt")
        .lines()
        .collect::<Vec<&str>>();

    let numbers = parse_bingo_numbers(input[0]);

    let mut index = 1;
    let mut indexes = Vec::new();

    for line in input.iter().skip(1) {
        if line.trim() == "" {
            indexes.push(index + 1);
        }

        index += 1;
    }

    let mut boards = Vec::new();

    for index in indexes {
        let board = parse_bingo_board(input.iter().skip(index).take(5).map(|x| *x).collect());
        boards.push(board);
    }

    for number in numbers {
        for board in boards.iter_mut() {
            for row in board.numbers.iter_mut() {
                for square in row.iter_mut() {
                    if square.number == number {
                        square.hit = true
                    }
                }
            }
        }

        let winner = has_winner(&boards);

        if winner.len() > 0 {
            let x = winner[0];

            let unmarked_value: i32 = boards[x]
                .numbers
                .iter()
                .flat_map(|x| x)
                .filter(|z| !z.hit)
                .map(|x| x.number)
                .sum();

            println!(
                "Number: {}, Sum: {}, Score is {}",
                number,
                unmarked_value,
                unmarked_value * number
            );

            return;
        }
    }
}

struct LastWinner {
    board_index: usize,
    number: i32,
}

pub fn solve_2() {
    let input = include_str!("../input/input4.txt")
        .lines()
        .collect::<Vec<&str>>();

    let numbers = parse_bingo_numbers(input[0]);

    let mut index = 1;
    let mut indexes = Vec::new();

    for line in input.iter().skip(1) {
        if line.trim() == "" {
            indexes.push(index + 1);
        }

        index += 1;
    }

    let mut boards = Vec::new();

    for index in indexes {
        let board = parse_bingo_board(input.iter().skip(index).take(5).map(|x| *x).collect());
        boards.push(board);
    }

    let mut winner_indexes = Vec::new();

    for number in numbers {
        for board in boards.iter_mut() {
            for row in board.numbers.iter_mut() {
                for square in row.iter_mut() {
                    if square.number == number {
                        square.hit = true
                    }
                }
            }
        }

        let winners = has_winner(&boards);

        for winner in winners {
            if !winner_indexes.contains(&winner) {
                winner_indexes.push(winner);
                println!("Adding winner index {}", winner);
            }
        }

        println!(
            "After: {} there are {} winners",
            number,
            winner_indexes.len()
        );

        if winner_indexes.len() == boards.len() {
            let unmarked_value: i32 = boards[winner_indexes[winner_indexes.len() - 1]]
                .numbers
                .iter()
                .flat_map(|x| x)
                .filter(|z| !z.hit)
                .map(|x| x.number)
                .sum();

            println!(
                "Number: {}, Sum: {}, Score is {}",
                number,
                unmarked_value,
                unmarked_value * number
            );

            return;
        }
    }
}
