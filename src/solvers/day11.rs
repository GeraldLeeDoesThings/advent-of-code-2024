use std::collections::HashMap;

pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let mut arrangement: Vec<usize> = input
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

        let solution_iters = Vec::from_iter(
            arrangement
                .iter()
                .map(|stone| evolve(*stone, 25, &mut cache)),
        );
        arrangement = solution_iters
            .iter()
            .map(|inner| inner.iter())
            .flatten()
            .map(|s| *s)
            .collect();

        // println!("{:#?}", arrangement);
        arrangement.len().to_string()
    }
}
