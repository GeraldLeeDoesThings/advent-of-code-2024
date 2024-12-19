pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let mut count: u32 = 0;
        let mut empty = false;
        let mut fs: Vec<Option<u32>> = input
            .chars()
            .flat_map(|c| {
                let repts = c.to_digit(10).unwrap() as usize;
                if empty {
                    empty = false;
                    std::iter::repeat(None).take(repts)
                } else {
                    empty = true;
                    count += 1;
                    std::iter::repeat(Some(count - 1)).take(repts)
                }
            })
            .collect();
        let mut low: usize = 0;
        let mut high = fs.len() - 1;
        let mut acc: usize = 0;
        while low < fs.len() {
            while high > 0 && fs.get(high).unwrap().is_none() {
                high -= 1;
            }
            match fs.get(low).unwrap() {
                None => {
                    if high > low {
                        fs.swap(low, high)
                    } else {
                        low += 1;
                    }
                }
                &Some(v) => {
                    acc += low * v as usize;
                    low += 1;
                }
            }
        }
        println!("");
        acc.to_string()
    }
}
