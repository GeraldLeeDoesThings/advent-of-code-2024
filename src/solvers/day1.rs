use std::iter::zip;

pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let mut cleaned = input.replace("  ", " ");
        while cleaned.contains("  ") {
            cleaned = cleaned.replace("  ", " ")
        }
        let mut list_a: Vec<usize> = Vec::new();
        let mut list_b: Vec<usize> = Vec::new();
        for line in cleaned.lines() {
            let nums: Vec<&str> = line.split(" ").collect();
            if nums.len() != 2 {
                break;
            }
            list_a.push(nums[0].parse().unwrap());
            list_b.push(nums[1].parse().unwrap());
        }
        list_a.sort();
        list_b.sort();
        let mut diff: usize = 0;
        for (a, b) in zip(list_a, list_b) {
            diff += a.abs_diff(b);
        }
        diff.to_string()
    }
}
