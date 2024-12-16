pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let mut grid: Vec<Vec<char>> = Vec::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                while x >= grid.len() {
                    grid.push(Vec::new());
                }
                assert!(grid[x].len() == y);
                grid[x].insert(y, c);
            }
        }
        let directions = [
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
        ];
        let mut to_search: Vec<(usize, usize, char)> = Vec::new();
        let mut found: usize = 0;

        fn search(x: usize, y: usize, grid: &Vec<Vec<char>>, direction: (isize, isize), target: &str) -> bool {
            if target.len() == 0 {
                return true;
            }
            if let Some(found) = grid.get(x).map(|xs| xs.get(y)).flatten() {
                if *found == target.chars().nth(0).unwrap() {
                    if target.len() == 1 {
                        return true;
                    }
                    let nx = x.checked_add_signed(direction.0);
                    let ny = y.checked_add_signed(direction.1);
                    if nx.is_some() && ny.is_some() {
                        return search(nx.unwrap(), ny.unwrap(), grid, direction, &target[1..]);
                    }
                }
            }
            false
        }

        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                for direction in directions {
                    if search(x, y, &grid, direction, "XMAS") {
                        found += 1;
                    }
                }
            }
        }
        found.to_string()
    }
}
