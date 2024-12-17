use std::collections::HashMap;

pub struct Solver {}

struct Rule {
    first: usize,
    second: usize,
}

impl Rule {
    fn satisfied(&self, update_indicies: &HashMap<usize, usize>) -> bool {
        let first_index = update_indicies.get(&self.first);
        let second_index = update_indicies.get(&self.second);
        if first_index.is_some() && second_index.is_some() {
            first_index.unwrap() < second_index.unwrap()
        }
        else {
            true
        }
    }
}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        // TODO: Implement
        let mut parsing_rules = true;
        let mut rules: Vec<Rule> = Vec::new();
        let mut acc: usize = 0;
        for line in input.lines() {
            if line.is_empty() {
                parsing_rules = false;
            }
            else if parsing_rules {
                let mut parts = line.split("|");
                rules.push(
                    Rule {
                        first: parts.next().unwrap().parse().unwrap(),
                        second: parts.next().unwrap().parse().unwrap(),
                    }
                );
            }
            else {
                let updates: Vec<usize> = line.split(",").map(|num| num.parse().unwrap()).collect();
                let indicies: HashMap<usize, usize> = HashMap::from_iter(
                    updates
                        .iter()
                        .enumerate()
                        .map(|(i, v)| (*v, i))
                );
                if rules.iter().all(|rule| rule.satisfied(&indicies)) {
                    acc += updates[updates.len() / 2]
                }
            }
        }
        acc.to_string()
    }
}
