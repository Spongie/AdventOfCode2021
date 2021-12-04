use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_input_as_string_list(file: &str) -> Vec<String> {
    let mut rows = Vec::new();

    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(x) = line {
                rows.push(x);
            }
        }
    }

    return rows;
}

pub fn read_input_as_integers(file: &str) -> Vec<i32> {
    read_input_as_string_list(file)
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}
