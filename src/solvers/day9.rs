pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let mut count: u32 = 0;
        let mut empty = false;
        let mut gaps: Vec<(usize, usize)> = Vec::new();
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

        fn count_free_space_at(data: &Vec<Option<u32>>, i: usize) -> usize {
            let mut found: usize = 0;
            while data.get(i + found).map(|v| v.is_none()).unwrap_or(false) {
                found += 1;
            }
            found
        }

        while low < fs.len() {
            if fs[low].is_none() {
                let space = count_free_space_at(&fs, low);
                gaps.push((low, space));
                low += space;
            } else {
                low += 1;
            }
        }

        let mut high = fs.len() - 1;

        // Counts backwards...
        fn count_block_size_at(data: &Vec<Option<u32>>, i: usize) -> usize {
            let mut found: usize = 1;
            let target = data.get(i).unwrap().unwrap();
            while found <= i
                && data
                    .get(i - found)
                    .map(|v| v.map(|iv| iv == target))
                    .flatten()
                    .unwrap_or(false)
            {
                found += 1;
            }
            found
        }

        low = 0;
        'defrag: while high > low {
            if fs[high].is_some() {
                let space = count_block_size_at(&fs, high);
                for (i, (gap_index, gap_size)) in gaps.iter().enumerate() {
                    if *gap_index > high - (space - 1) {
                        break;
                    }
                    if *gap_size >= space {
                        for i in 0..space {
                            fs.swap(gap_index + i, high - i);
                        }

                        if *gap_size == space {
                            gaps.remove(i);
                        } else {
                            gaps[i].0 += space;
                            gaps[i].1 -= space;
                        }
                        high -= space;
                        continue 'defrag;
                    }
                }
                if space > high {
                    break 'defrag;
                }
                high -= space;
            } else {
                high -= 1;
            }
        }

        let acc: usize = fs
            .iter()
            .enumerate()
            .map(|(i, b)| i * b.unwrap_or(0) as usize)
            .sum();
        acc.to_string()
    }
}
