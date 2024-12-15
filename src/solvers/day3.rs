use regex::Regex;

pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let re = Regex::new(r"(mul\(([0-9]+),([0-9]+)\))|(do\(\))|(don't\(\))").unwrap();
        let mut acc: usize = 0;
        let mut enabled = true;
        for mat in re.captures_iter(input) {
            let full = mat.get(0).unwrap().as_str();
            if full.starts_with("mul") && enabled {
                acc += mat.get(2).unwrap().as_str().parse::<usize>().unwrap() * mat.get(3).unwrap().as_str().parse::<usize>().unwrap();
            }
            else if full.starts_with("don't") {
                enabled = false;
            }
            else if full.starts_with("do") {
                enabled = true;
            }
        }
        acc.to_string()
    }
}
