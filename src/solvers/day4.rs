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
        let mut found: usize = 0;

        fn search(x: usize, y: usize, grid: &Vec<Vec<char>>) -> bool {
            let ul = grid[x - 1][y - 1];
            let ur = grid[x + 1][y - 1];
            let bl = grid[x - 1][y + 1];
            let br = grid[x + 1][y + 1];
            let down_diagonal_ok = (ul == 'M' && br == 'S') || (ul == 'S' && br == 'M');
            let up_diagonal_ok = (bl == 'M' && ur == 'S') || (bl == 'S' && ur == 'M');
            down_diagonal_ok && up_diagonal_ok && grid[x][y] == 'A'
        }

        for x in 1..grid.len() - 1 {
            for y in 1..grid[0].len() - 1 {
                if search(x, y, &grid) {
                    found += 1;
                }
            }
        }
        found.to_string()
    }
}
