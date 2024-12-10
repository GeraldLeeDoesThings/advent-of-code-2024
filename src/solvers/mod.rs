use crate::Solver;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn get_solver(day: u8) -> Option<Box<dyn Solver>> {
    match day {
        1 => Some(Box::new(day1::Solver {})),
        2 => Some(Box::new(day2::Solver {})),
        3 => Some(Box::new(day3::Solver {})),
        4 => Some(Box::new(day4::Solver {})),
        5 => Some(Box::new(day5::Solver {})),
        6 => Some(Box::new(day6::Solver {})),
        7 => Some(Box::new(day7::Solver {})),
        8 => Some(Box::new(day8::Solver {})),
        9 => Some(Box::new(day9::Solver {})),
        10 => Some(Box::new(day10::Solver {})),
        11 => Some(Box::new(day11::Solver {})),
        12 => Some(Box::new(day12::Solver {})),
        13 => Some(Box::new(day13::Solver {})),
        14 => Some(Box::new(day14::Solver {})),
        15 => Some(Box::new(day15::Solver {})),
        16 => Some(Box::new(day16::Solver {})),
        17 => Some(Box::new(day17::Solver {})),
        18 => Some(Box::new(day18::Solver {})),
        19 => Some(Box::new(day19::Solver {})),
        20 => Some(Box::new(day20::Solver {})),
        21 => Some(Box::new(day21::Solver {})),
        22 => Some(Box::new(day22::Solver {})),
        23 => Some(Box::new(day23::Solver {})),
        24 => Some(Box::new(day24::Solver {})),
        25 => Some(Box::new(day25::Solver {})),
        _ => None,
    }
}
