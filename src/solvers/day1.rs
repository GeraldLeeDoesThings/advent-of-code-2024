use std::collections::HashMap;

pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let mut cleaned = input.replace("  ", " ");
        while cleaned.contains("  ") {
            cleaned = cleaned.replace("  ", " ")
        }
        let mut count_a: HashMap<usize, usize> = HashMap::new();
        let mut count_b: HashMap<usize, usize> = HashMap::new();
        for line in cleaned.lines() {
            let nums: Vec<&str> = line.split(" ").collect();
            if nums.len() != 2 {
                break;
            }
            let a = nums[0].parse().unwrap();
            let b = nums[1].parse().unwrap();
            count_a.insert(a, count_a.get(&a).unwrap_or(&0) + 1);
            count_b.insert(b, count_b.get(&b).unwrap_or(&0) + 1);
        }
        let mut similarity: usize = 0;
        for (a, count) in &count_a {
            similarity += a * count * count_b.get(a).unwrap_or(&0);
        }
        similarity.to_string()
    }
}
