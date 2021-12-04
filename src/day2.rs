use crate::common;

struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

impl Position {
    fn new() -> Position {
        Position { x: 0, y: 0, aim: 0 }
    }
}

struct Command {
    command_type: String,
    amount: i32,
}

impl Command {
    fn from_line(line: &str) -> Command {
        let data: Vec<&str> = line.split(" ").collect();

        assert_eq!(2, data.len());

        Command {
            command_type: data[0].to_string(),
            amount: data[1].parse::<i32>().unwrap(),
        }
    }
}

pub fn solve_1() {
    let input: Vec<Command> = common::read_input_as_string_list("input/input_2_1.txt")
        .into_iter()
        .map(|x| Command::from_line(&x))
        .collect();

    let mut position = Position::new();

    for command in input {
        match command.command_type.as_str() {
            "forward" => position.x += command.amount,
            "up" => position.y -= command.amount,
            "down" => position.y += command.amount,
            _ => (),
        };

        if position.y < 0 {
            position.y = 0;
        }
    }

    println!("Final position is x:{} y:{}", position.x, position.y);
    println!("Result is: {}", position.x * position.y);
}

pub fn solve_2() {
    let input: Vec<Command> = common::read_input_as_string_list("input/input_2_1.txt")
        .into_iter()
        .map(|x| Command::from_line(&x))
        .collect();

    let mut position = Position::new();

    for command in input {
        match command.command_type.as_str() {
            "forward" => {
                position.x += command.amount;
                position.y += command.amount * position.aim;
            }
            "up" => position.aim -= command.amount,
            "down" => position.aim += command.amount,
            _ => (),
        };

        if position.y < 0 {
            position.y = 0;
        }
    }

    println!("Final position is x:{} y:{}", position.x, position.y);
    println!("Result is: {}", position.x * position.y);
}
