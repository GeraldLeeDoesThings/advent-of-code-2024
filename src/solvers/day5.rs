use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let mut parsing_rules = true;
        let mut before: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut after: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut acc: usize = 0;
        for line in input.lines() {
            if line.is_empty() {
                parsing_rules = false;
            } else if parsing_rules {
                let mut parts = line.split("|");
                let first = parts.next().unwrap().parse().unwrap();
                let second = parts.next().unwrap().parse().unwrap();
                if let Some(before_set) = before.get_mut(&first) {
                    before_set.insert(second);
                } else {
                    let mut before_set = HashSet::new();
                    before_set.insert(second);
                    assert!(before.insert(first, before_set).is_none());
                }
                if let Some(after_set) = after.get_mut(&second) {
                    after_set.insert(first);
                } else {
                    let mut after_set = HashSet::new();
                    after_set.insert(first);
                    assert!(after.insert(second, after_set).is_none());
                }
            } else {
                let mut updates: Vec<usize> =
                    line.split(",").map(|num| num.parse().unwrap()).collect();
                let mut working_before: HashSet<usize> = HashSet::new();
                let mut working_after: HashSet<usize> =
                    HashSet::from_iter(updates.iter().map(|v| *v));
                let mut valid = true;
                for val in &updates {
                    working_after.remove(val);
                    let before_ok = before
                        .get(val)
                        .map(|bset| bset.is_disjoint(&working_before))
                        .unwrap_or(true);
                    let after_ok = after
                        .get(val)
                        .map(|aset| aset.is_disjoint(&working_after))
                        .unwrap_or(true);
                    if !(before_ok && after_ok) {
                        valid = false;
                        break;
                    }
                    working_before.insert(*val);
                }
                if !valid {
                    updates.sort_by(|a, b| {
                        if a == b {
                            return Ordering::Equal;
                        }
                        if let Some(before) = before.get(a) {
                            if before.contains(b) {
                                return Ordering::Greater;
                            }
                        }
                        if let Some(before) = before.get(b) {
                            if before.contains(a) {
                                return Ordering::Less;
                            }
                        }
                        Ordering::Equal
                    });
                    acc += updates[updates.len() / 2];
                }
            }
        }
        acc.to_string()
    }
}
