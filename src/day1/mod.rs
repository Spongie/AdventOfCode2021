use crate::common;

pub fn solve_1() {
    let input: Vec<i32> = common::read_input_as_integers("input/input_1_1.txt");

    let mut x = 1;
    let mut increases = 0;
    while x < input.len() {
        if input[x - 1] < input[x] {
            increases += 1;
        }

        x += 1;
    }

    println!("Increased {} times", increases);
}

pub fn solve_2() {
    let input: Vec<i32> = common::read_input_as_integers("input/input_1_1.txt");

    let mut x = 1;
    let mut increases = 0;

    while x < input.len() {
        if x + 2 >= input.len() {
            break;
        }
        let previous_segment: i32 = input[x - 1..=x + 1].iter().sum();
        let segment: i32 = input[x..=x + 2].iter().sum();

        if segment > previous_segment {
            increases += 1;
        }

        x += 1;
    }

    println!("Increased {} times", increases);
}
