use regex::Regex;

pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
        let mut acc: usize = 0;
        for mat in re.captures_iter(input) {
            acc += mat.get(1).unwrap().as_str().parse::<usize>().unwrap() * mat.get(2).unwrap().as_str().parse::<usize>().unwrap();
        }
        acc.to_string()
    }
}
