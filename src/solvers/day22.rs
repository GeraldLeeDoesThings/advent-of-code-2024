pub struct Solver {}

fn next_secret(secret: usize) -> usize {
    let mut new_secret = secret ^ (secret << 6) % 16777216;
    new_secret = new_secret ^ (new_secret >> 5) % 16777216;
    new_secret ^ (new_secret << 11) % 16777216
}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        input
            .lines()
            .map(|line| {
                let mut secret: usize = line.parse().unwrap();
                for _ in 0..2000 {
                    secret = next_secret(secret);
                }
                secret
            })
            .sum::<usize>()
            .to_string()
    }
}
