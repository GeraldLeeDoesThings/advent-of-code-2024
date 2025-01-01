use std::collections::{HashMap, HashSet};

pub struct Solver {}

fn is_optimal(path: &Vec<(usize, usize)>) -> bool {
    if path.is_empty() {
        return true;
    }

    // Assume that duplicate points are not present

    let first = path.first().unwrap();
    let last = path.last().unwrap();
    let taxicab_dist = first.0.abs_diff(last.0) + first.1.abs_diff(last.1);
    path.len() - 1 == taxicab_dist
}

type CoordMap = HashMap<char, (usize, usize)>;
type PathMap = HashMap<(char, char), HashSet<Vec<char>>>;

fn cache_paths<const A: usize, const B: usize>(
    map: &[[Option<char>; A]; B],
    coord_map: &mut CoordMap,
    paths: &mut PathMap,
) {
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c.is_none() {
                continue;
            }
            let start = c.unwrap();
            paths.insert((start, start), HashSet::from([Vec::new()]));

            assert!(coord_map.insert(start, (x, y)).is_none());
            let mut explore_list: HashSet<Vec<(usize, usize)>> =
                HashSet::from([Vec::from([(x, y)])]);
            while !explore_list.is_empty() {
                explore_list = HashSet::from_iter(
                    explore_list
                        .iter()
                        .flat_map(|path| {
                            let last = path.last().unwrap();
                            [(-1, 0), (0, -1), (1, 0), (0, 1)]
                                .iter()
                                .filter_map(|(dx, dy)| {
                                    if let (Some(nx), Some(ny)) = (
                                        last.0.checked_add_signed(*dx),
                                        last.1.checked_add_signed(*dy),
                                    ) {
                                        let mut extended = path.clone();
                                        extended.push((nx, ny));
                                        Some(extended)
                                    } else {
                                        None
                                    }
                                })
                        })
                        .filter(|path| {
                            let (x, y) = path.last().unwrap();
                            map.get(*y)
                                .is_some_and(|row| row.get(*x).map(|o| *o).flatten().is_some())
                                && is_optimal(path)
                        }),
                );

                for list in &explore_list {
                    let last = list.last().unwrap();
                    let end = map[last.1][last.0].unwrap();
                    if paths.get(&(start, end)).is_none() {
                        paths.insert((start, end), HashSet::new());
                    }
                    paths.get_mut(&(start, end)).unwrap().insert(
                        list.windows(2)
                            .map(|window| {
                                let a = window[0];
                                let b = window[1];
                                (b.0 as isize - a.0 as isize, b.1 as isize - a.1 as isize)
                            })
                            .map(|d| match d {
                                (-1, 0) => '<',
                                (0, -1) => '^',
                                (1, 0) => '>',
                                (0, 1) => 'v',
                                _ => unreachable!(),
                            })
                            .collect(),
                    );
                }
            }
        }
    }
}

type SolveCache = HashMap<Vec<char>, HashMap<char, HashSet<Vec<char>>>>;

fn solve<'a>(
    init: char,
    seq: &'a [char],
    path_map: &PathMap,
    cache: &mut SolveCache,
) -> HashSet<Vec<char>> {
    if let Some(cached) = cache
        .get(&seq.to_vec())
        .map(|cmap| cmap.get(&init))
        .flatten()
    {
        return cached.clone();
    }

    let paths: HashSet<Vec<char>> = HashSet::from([Vec::new()]);
    let prev = init;

    if seq.len() == 1 {
        let next = seq[0];
        if prev != next {
            let extensions = path_map.get(&(prev, next)).unwrap();
            HashSet::from_iter(paths.iter().flat_map(|path| {
                extensions.iter().map(|ext| {
                    let mut extended = path.clone();
                    extended.extend(ext);
                    extended.push('A');
                    extended
                })
            }))
        } else {
            HashSet::from_iter(paths.iter().map(|path| {
                let mut extended = path.clone();
                extended.push('A');
                extended
            }))
        }
    } else {
        let split = seq.len() / 2;
        let split_init = seq[split - 1];
        let second_half = solve(split_init, &seq[split..], path_map, cache);
        let result =
            HashSet::from_iter(solve(init, &seq[..split], path_map, cache).iter().flat_map(
                |first_half| {
                    second_half.iter().map(|second| {
                        let mut first = first_half.clone();
                        first.extend(second.iter());
                        first
                    })
                },
            ));

        if let Some(init_map) = cache.get_mut(seq) {
            init_map.insert(init, result.clone());
        } else {
            let mut init_map = HashMap::new();
            init_map.insert(init, result.clone());
            cache.insert(seq.to_vec(), init_map);
        }

        result
    }
}

fn path_to_transitions(path: &Vec<char>) -> HashMap<(char, char), usize> {
    let mut result: HashMap<(char, char), usize> = HashMap::new();
    let mut prev = 'A';
    for c in path {
        let next = *c;
        result.insert((prev, next), result.get(&(prev, next)).unwrap_or(&0) + 1);
        prev = next;
    }
    result.insert((prev, 'A'), result.get(&(prev, 'A')).unwrap_or(&0) + 1);
    result
}

type TransitionCache = HashMap<(char, char, usize), usize>;

fn solve_transition(
    transition: (char, char),
    depth: usize,
    path_map: &PathMap,
    cache: &mut TransitionCache,
) -> usize {
    if let Some(cached) = cache.get(&(transition.0, transition.1, depth)) {
        return *cached;
    }

    if depth == 0 {
        // Manual input
        return 1;
    }

    let result = path_map
        .get(&transition)
        .unwrap()
        .iter()
        .map(|path| {
            let transitions = path_to_transitions(path);
            let mut num_transitions: usize = 0;
            for (transition, count) in transitions {
                num_transitions += count * solve_transition(transition, depth - 1, path_map, cache);
            }
            num_transitions
        })
        .min()
        .unwrap();

    cache.insert((transition.0, transition.1, depth), result);
    result
}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let numeric: [[Option<char>; 3]; 4] = [
            [Some('7'), Some('8'), Some('9')],
            [Some('4'), Some('5'), Some('6')],
            [Some('1'), Some('2'), Some('3')],
            [None, Some('0'), Some('A')],
        ];

        let mut numeric_paths: PathMap = HashMap::new();
        let mut number_to_coord: CoordMap = HashMap::new();

        cache_paths(&numeric, &mut number_to_coord, &mut numeric_paths);

        let directional: [[Option<char>; 3]; 2] = [
            [None, Some('^'), Some('A')],
            [Some('<'), Some('v'), Some('>')],
        ];
        let mut direction_paths: PathMap = HashMap::new();
        let mut direction_to_coord: CoordMap = HashMap::new();

        cache_paths(&directional, &mut direction_to_coord, &mut direction_paths);

        let mut transition_cache: TransitionCache = TransitionCache::new();
        let mut acc: usize = 0;
        let depth = 25;
        for line in input.lines() {
            let chars: Vec<char> = line.chars().collect();
            let paths = solve('A', &chars[..], &numeric_paths, &mut HashMap::new());
            let min_transitions_needed: usize = paths
                .iter()
                .map(|path| {
                    let transitions = path_to_transitions(path);
                    transitions
                        .iter()
                        .map(|(transition, count)| {
                            count
                                * solve_transition(
                                    *transition,
                                    depth,
                                    &direction_paths,
                                    &mut transition_cache,
                                )
                        })
                        .sum::<usize>()
                })
                .min()
                .unwrap();

            let min_inputs = min_transitions_needed - 1; // Last transition (last -> A) is a dummy transition
            acc += line[..line.len() - 1].parse::<usize>().unwrap() * min_inputs;
        }

        acc.to_string()
    }
}
