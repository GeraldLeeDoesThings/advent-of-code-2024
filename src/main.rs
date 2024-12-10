use std::{
    cmp::max,
    collections::HashMap,
    fmt::Display,
    fs::{self, read_to_string},
    io,
    num::ParseIntError,
    path::PathBuf,
};

use clap::Parser;

mod solvers;

struct DayInputPair {
    input: String,
    day: u8,
}

#[derive(Parser)]
struct CliArgs {
    #[arg(short, long)]
    day: Option<u8>,
    #[arg(short, long)]
    test_dir: Option<PathBuf>,
}

#[derive(Debug)]
enum ParseDayInputPairError {
    IoError(io::Error),
    DayParseError(ParseIntError),
    OtherError(String),
}

impl Display for ParseDayInputPairError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseDayInputPairError::IoError(error) => writeln!(f, "IoError: {error}"),
            ParseDayInputPairError::DayParseError(parse_int_error) => {
                writeln!(f, "DayParseError: {parse_int_error}")
            }
            ParseDayInputPairError::OtherError(error_msg) => writeln!(f, "OtherError: {error_msg}"),
        }
    }
}

impl From<io::Error> for ParseDayInputPairError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<ParseIntError> for ParseDayInputPairError {
    fn from(value: ParseIntError) -> Self {
        Self::DayParseError(value)
    }
}

impl From<&str> for ParseDayInputPairError {
    fn from(s: &str) -> Self {
        Self::OtherError(s.to_string())
    }
}

impl From<String> for ParseDayInputPairError {
    fn from(s: String) -> Self {
        Self::OtherError(s)
    }
}

pub trait Solver {
    fn solve(&self, input: &String) -> String;
}

fn parse_args(args: &CliArgs) -> Result<DayInputPair, ParseDayInputPairError> {
    // TODO: Allow the test dir to be passed as an argument
    let paths = fs::read_dir(args.test_dir.clone().unwrap_or("./tests".into()))?;
    let mut max_day: u8 = 0;
    let mut day_dir_map: HashMap<u8, PathBuf> = HashMap::new();

    for maybe_path in paths {
        let path = maybe_path?;
        let child_type = path.file_type()?;
        let path_os_string = path.file_name();
        let path_str = path_os_string
            .to_str()
            .ok_or("Failed to convert OS String to str")?;
        let file_extension_dot_index = path_str.find('.').unwrap_or(path_str.len());
        if let Ok(day) = path_str[0..file_extension_dot_index].parse::<u8>() {
            if child_type.is_file() {
                day_dir_map.insert(day, path.path());
                max_day = max(max_day, day);
            }
        }
    }

    let target_day = args.day.unwrap_or(max_day);
    if !day_dir_map.contains_key(&target_day) {
        return Err(format!("Could not find test file for day {target_day}").into());
    }

    Ok(DayInputPair {
        input: read_to_string(day_dir_map.get(&target_day).unwrap())?,
        day: target_day,
    })
}

fn main() {
    let maybe_input_pair = parse_args(&CliArgs::parse());
    if let Err(parse_err) = maybe_input_pair {
        println!("Encountered error while parsing input: {}", parse_err);
        return;
    }
    let input_pair = maybe_input_pair.unwrap();
    println!("Day: {}", input_pair.day);
    let maybe_solver = solvers::get_solver(input_pair.day);
    if maybe_solver.is_none() {
        println!("Could not find solver for day {}", input_pair.day);
    }
    let solver = maybe_solver.unwrap();
    println!("Result:\n{}", solver.solve(&input_pair.input));
}
