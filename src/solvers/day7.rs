pub struct Solver {}

fn solve_operands(target: usize, acc: usize, operands: &[usize]) -> bool {
    if operands.len() == 0 {
        return target == acc;
    }

    if acc > target {
        return false;
    }

    let num_digits = 1 + operands[0].checked_ilog10().unwrap_or(0);

    solve_operands(target, acc + operands[0], &operands[1..])
        || solve_operands(target, acc * operands[0], &operands[1..])
        || solve_operands(
            // Comment this option out for part 1 solution
            target,
            acc * 10_usize.pow(num_digits) + operands[0],
            &operands[1..],
        )
}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let mut acc: usize = 0;
        for line in input.lines() {
            let mut target_split = line.split(": ");
            let target: usize = target_split.next().unwrap().parse().unwrap();
            let operands: Vec<usize> = target_split
                .next()
                .unwrap()
                .split(" ")
                .map(|s| s.parse().unwrap())
                .collect();
            if solve_operands(target, 0, &operands) {
                acc += target;
            }
        }
        acc.to_string()
    }
}
