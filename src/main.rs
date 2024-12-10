use std::{
    cmp::max,
    collections::HashMap,
    env::args,
    fs::{self, read_to_string},
    io,
    num::ParseIntError,
    path::PathBuf,
};

pub struct DayInputPair {
    input: String,
    day: u8,
}

#[derive(Debug)]
pub enum ParseDayInputPairError {
    IoError(io::Error),
    DayParseError(ParseIntError),
    OtherError(String),
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

fn parse_args() -> Result<DayInputPair, ParseDayInputPairError> {
    // TODO: Allow the test dir to be passed as an argument
    let paths = fs::read_dir("./tests")?;
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

    let target_day = args()
        .nth(1)
        .map(|day_string| day_string.parse::<u8>())
        .transpose()?
        .unwrap_or(max_day);
    if !day_dir_map.contains_key(&target_day) {
        return Err(format!("Could not find test file for day {target_day}").into());
    }

    Ok(DayInputPair {
        input: read_to_string(day_dir_map.get(&target_day).unwrap())?,
        day: target_day,
    })
}

fn main() {
    let maybe_input_pair = parse_args();
    if let Err(parse_err) = maybe_input_pair {
        println!("Encountered error while parsing input: {:#?}", parse_err);
        return;
    }
    let input_pair = maybe_input_pair.unwrap();
    println!("Day: {}\n{}", input_pair.day, input_pair.input);
}
