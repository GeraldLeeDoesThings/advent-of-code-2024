use std::{
    cmp::min,
    collections::{BinaryHeap, HashMap},
};

pub struct Solver {}

#[derive(PartialEq, Eq)]
struct EvolvePair {
    steps_needed: usize,
    val: usize,
    weight: usize,
}

impl PartialOrd for EvolvePair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EvolvePair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.steps_needed.cmp(&other.steps_needed) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => match self.val.cmp(&other.val) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => self.weight.cmp(&other.weight),
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            },
            std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
        }
    }
}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let arrangement: Vec<usize> = input
            .split(" ")
            .map(|num_str| num_str.parse().unwrap())
            .collect();
        let mut cache: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

        fn evolve(
            stone: usize,
            steps: usize,
            cache: &mut HashMap<(usize, usize), Vec<usize>>,
        ) -> Vec<usize> {
            if let Some(result) = cache.get(&(stone, steps)) {
                return result.clone();
            }

            if steps == 0 {
                return Vec::from([stone]);
            }

            for step in steps - 1..=0 {
                let mut partials: Vec<usize> = Vec::new();
                if let Some(partial_result) = cache.get(&(stone, steps)) {
                    partials.extend(partial_result);
                }
                if partials.len() > 0 {
                    let mut result: Vec<usize> = Vec::new();
                    for partial in partials {
                        let expanded = evolve(partial, steps - step, cache);
                        result.extend(expanded);
                    }
                    cache.insert((stone, steps), result.clone());
                    return result;
                }
            }

            if stone == 0 {
                let result = evolve(1, steps - 1, cache);
                cache.insert((stone, steps), result.clone());
                result
            } else {
                let num_digits = stone.ilog10() + 1;
                if num_digits % 2 == 0 {
                    let top_digits = stone / 10_usize.pow(num_digits / 2);
                    let bottom_digits = stone - top_digits * 10_usize.pow(num_digits / 2);
                    let result = Vec::from_iter(
                        evolve(top_digits, steps - 1, cache)
                            .iter()
                            .map(|s| *s)
                            .chain(evolve(bottom_digits, steps - 1, cache).iter().map(|s| *s)),
                    );
                    cache.insert((stone, steps), result.clone());
                    result
                } else {
                    let result = evolve(stone * 2024, steps - 1, cache);
                    cache.insert((stone, steps), result.clone());
                    result
                }
            }
        }

        let mut compute_queue = BinaryHeap::from_iter(arrangement.iter().map(|s| EvolvePair {
            steps_needed: 75,
            val: *s,
            weight: 1,
        }));
        let mut acc = 0;
        let limit = 25;

        while let Some(to_comp) = compute_queue.pop() {
            let target_steps = min(to_comp.steps_needed, limit);
            let partial = evolve(to_comp.val, target_steps, &mut cache);
            // println!("{} {} {}", to_comp.steps_needed, to_comp.val, to_comp.weight);
            if target_steps == to_comp.steps_needed {
                acc += to_comp.weight * partial.len();
                // println!("{:#?}", partial);
            } else {
                let mut count: HashMap<usize, usize> = HashMap::new();
                for s in partial {
                    let current = count.get(&s).unwrap_or(&0);
                    count.insert(s, current + 1);
                }
                for (val, weight) in count {
                    compute_queue.push(EvolvePair {
                        steps_needed: to_comp.steps_needed - target_steps,
                        val: val,
                        weight: to_comp.weight * weight,
                    });
                }
            }
        }

        // println!("{:#?}", arrangement);
        acc.to_string()
    }
}
