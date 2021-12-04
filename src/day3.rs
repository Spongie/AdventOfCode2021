use crate::common;

pub fn solve_1() {
    let input = common::read_input_as_string_list("input/input_3_1.txt");

    let bit_count = input[0].len();
    let mut gamma_bits = Vec::with_capacity(bit_count);
    let mut i = 0;
    while i < bit_count {
        gamma_bits.push(0);
        i += 1;
    }

    for line in input {
        i = 0;

        while i < line.len() {
            let value: i32 = if line
                .chars()
                .nth(i)
                .unwrap()
                .to_string()
                .parse::<i32>()
                .unwrap()
                == 0
            {
                -1
            } else {
                1
            };
            gamma_bits[i] += value;

            i += 1;
        }
    }

    let mut gamma_bit_string = String::new();

    for bit in gamma_bits {
        if bit > 0 {
            gamma_bit_string += "1"
        } else {
            gamma_bit_string += "0"
        }
    }

    let gamma = i32::from_str_radix(gamma_bit_string.as_ref(), 2).unwrap();
    let epsilon = gamma ^ i32::MAX >> (31 - gamma_bit_string.len());

    println!(
        "Gamma: {}, Epsilon: {}, Result {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

fn solve_rating(input: &[String], should_pick_ones: fn(usize, usize) -> bool) -> i32 {
    let bit_count = input[0].len();
    let mut i = 0;

    let mut numbers: Vec<String> = Vec::new();

    for number in input {
        numbers.push(number.to_string());
    }

    loop {
        let chars: Vec<char> = numbers.iter().map(|x| x.chars().nth(i).unwrap()).collect();
        let ones = chars
            .iter()
            .filter(|x| (**x) == '1')
            .collect::<Vec<&char>>()
            .len();
        let zeros = chars
            .iter()
            .filter(|x| (**x) == '0')
            .collect::<Vec<&char>>()
            .len();

        if should_pick_ones(ones, zeros) {
            numbers = numbers
                .iter()
                .filter(|x| x.chars().nth(i).unwrap() == '1')
                .map(|x| x.to_string())
                .collect();
        } else {
            numbers = numbers
                .iter()
                .filter(|x| x.chars().nth(i).unwrap() == '0')
                .map(|x| x.to_string())
                .collect();
        }

        i += 1;

        if i >= bit_count || numbers.len() == 1 {
            break;
        }
    }

    i32::from_str_radix(numbers[0].as_ref(), 2).unwrap()
}

pub fn solve_2() {
    let input = common::read_input_as_string_list("input/input_3_1.txt");

    let oxygen_rating = solve_rating(&input, |ones, zeros| ones >= zeros);
    let c02_rating = solve_rating(&input, |ones, zeros| zeros > ones);

    println!(
        "Oxygen: {}, CO2: {}, Combined: {}",
        oxygen_rating,
        c02_rating,
        oxygen_rating * c02_rating
    )
}
