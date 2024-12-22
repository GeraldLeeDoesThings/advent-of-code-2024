use regex::Regex;

pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        // v1 = x1a + y1b
        // v2 = x2a + y2b
        // Some algebra later...
        // b = (x1 * v2 - x2 * v1) / (x1 * y2 - x2 * y1)
        // let re = Regex::new(r"(mul\(([0-9]+),([0-9]+)\))|(do\(\))|(don't\(\))").unwrap();
        let button_regex = Regex::new(r"Button (?:A|B): X\+([0-9]+), Y\+([0-9]+)").unwrap();
        let prize_regex = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();
        let mut acc: usize = 0;
        for chunk in input.split("\n\n") {
            let mut lines = chunk.lines();

            let button_a_parse = button_regex.captures(lines.next().unwrap()).unwrap();
            let button_b_parse = button_regex.captures(lines.next().unwrap()).unwrap();
            let prize_parse = prize_regex.captures(lines.next().unwrap()).unwrap();

            let button_a: (isize, isize) = (
                button_a_parse.get(1).unwrap().as_str().parse().unwrap(),
                button_a_parse.get(2).unwrap().as_str().parse().unwrap(),
            );
            let button_b: (isize, isize) = (
                button_b_parse.get(1).unwrap().as_str().parse().unwrap(),
                button_b_parse.get(2).unwrap().as_str().parse().unwrap(),
            );
            let prize: (isize, isize) = (
                10000000000000_isize
                    + prize_parse
                        .get(1)
                        .unwrap()
                        .as_str()
                        .parse::<isize>()
                        .unwrap(),
                10000000000000_isize
                    + prize_parse
                        .get(2)
                        .unwrap()
                        .as_str()
                        .parse::<isize>()
                        .unwrap(),
            );
            // println!("A: X+{} Y+{}", button_a.0, button_a.1);
            // println!("B: X+{} Y+{}", button_b.0, button_b.1);
            // println!("Prize: X+{} Y+{}", prize.0, prize.1);

            let b_denom = button_a.0 * button_b.1 - button_a.1 * button_b.0;
            let b_numer = button_a.0 * prize.1 - button_a.1 * prize.0;

            // a and b seem to always be linearly independant, so no need to check
            if let Some(b_candidate) = b_numer.checked_div(b_denom) {
                if let Some(a_candidate) =
                    (prize.0 - b_candidate * button_b.0).checked_div(button_a.0)
                {
                    if prize.0 == button_a.0 * a_candidate + button_b.0 * b_candidate
                        && prize.1 == button_a.1 * a_candidate + button_b.1 * b_candidate
                        && a_candidate >= 0
                        && b_candidate >= 0
                    {
                        acc += a_candidate as usize * 3 + b_candidate as usize;
                    }
                }
            }
        }
        acc.to_string()
    }
}
