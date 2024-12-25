pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let mut maybe_reg_b: Option<usize> = None;
        let mut maybe_reg_c: Option<usize> = None;
        let mut program: Vec<usize> = Vec::new();

        for line in input.lines() {
            if line.starts_with("Register B: ") {
                maybe_reg_b = Some(line[12..].parse().unwrap());
            }
            if line.starts_with("Register C: ") {
                maybe_reg_c = Some(line[12..].parse().unwrap());
            }
            if line.starts_with("Program: ") {
                program.extend(line[9..].split(',').map(|s| s.parse::<usize>().unwrap()));
            }
        }

        let mut reg_a;
        let reg_b = maybe_reg_b.unwrap();
        let reg_c = maybe_reg_c.unwrap();
        // println!("A: {}, B: {}, C: {}, Program: {:?}", reg_a, reg_b, reg_c, program);

        fn expand_combo(operand: usize, reg_a: usize, reg_b: usize, reg_c: usize) -> usize {
            match operand {
                0 | 1 | 2 | 3 => operand,
                4 => reg_a,
                5 => reg_b,
                6 => reg_c,
                7 => panic!("Reserved"),
                _ => panic!("Unknown combo operator"),
            }
        }

        /*
        start:
            b = a % 8
            b = b ^ 1
            c = a >> b
            b = b ^ c
            b = b ^ 4
            a = a >> 3
            output b
            if a != 0 goto start

        steps:
            b = a % 8
            b = (a % 8) ^ 1
            c = a >> ((a % 8) ^ 1)
            b = ((a % 8) ^ 1) ^ c
            b = ((a % 8) ^ 1) ^ c ^ 4
            a = a >> 3

            b = ((a % 8) ^ 1) ^ (a >> ((a % 8) ^ 1)) ^ 4
            0 = ((a % 8) ^ 1) ^ (a >> ((a % 8) ^ 1)) ^ 4  mod 8
            4 = ((a % 8) ^ 1) ^ (a >> ((a % 8) ^ 1))      mod 8
            (a % 8) ^ 1 ^ 4 = (a >> ((a % 8) ^ 1))        mod 8
            a ^ 1 ^ 4 = AHHHH


        */

        fn run_trial(
            init_reg_a: usize,
            init_reg_b: usize,
            init_reg_c: usize,
            program: &Vec<usize>,
            target: &[usize],
        ) -> bool {
            let mut reg_a = init_reg_a;
            let mut reg_b = init_reg_b;
            let mut reg_c = init_reg_c;

            let mut exec_ptr: usize = 0;
            let mut output_count: usize = 0;
            let mut ok = false;

            while let (Some(command), Some(operand)) = (
                program.get(exec_ptr).map(|c| *c),
                program.get(exec_ptr + 1).map(|c| *c),
            ) {
                match command {
                    0 => {
                        // adv
                        reg_a = reg_a >> expand_combo(operand, reg_a, reg_b, reg_c);
                        exec_ptr += 2;
                    }
                    1 => {
                        // bxl
                        reg_b = reg_b ^ operand;
                        exec_ptr += 2;
                    }
                    2 => {
                        // bst
                        reg_b = expand_combo(operand, reg_a, reg_b, reg_c) % 8;
                        exec_ptr += 2;
                    }
                    3 => {
                        // jnz
                        if reg_a != 0 {
                            exec_ptr = operand;
                        } else {
                            exec_ptr += 2;
                        }
                    }
                    4 => {
                        // bxc
                        reg_b = reg_b ^ reg_c;
                        exec_ptr += 2;
                    }
                    5 => {
                        // out
                        if target
                            .get(output_count)
                            .is_some_and(|v| *v == expand_combo(operand, reg_a, reg_b, reg_c) % 8)
                        {
                            output_count += 1;
                            if output_count == target.len() {
                                ok = true;
                            }
                        } else {
                            return false;
                        }
                        exec_ptr += 2;
                    }
                    6 => {
                        // bdv
                        reg_b = reg_a >> expand_combo(operand, reg_a, reg_b, reg_c);
                        exec_ptr += 2;
                    }
                    7 => {
                        // cdv
                        reg_c = reg_a >> expand_combo(operand, reg_a, reg_b, reg_c);
                        exec_ptr += 2;
                    }
                    _ => panic!("Unkown opcode"),
                }
            }

            ok
        }

        let mut candidates: Vec<usize> = Vec::from([0]);

        // This only works because the program right shifts a
        // by 3 every loop, and loops nicely
        for offset in 1..=program.len() {
            let mut new_candidates: Vec<usize> = Vec::new();
            for candidate in &candidates {
                for reg_a_candidate in 0..(1 << 3) {
                    reg_a = (candidate << 3) + reg_a_candidate;
                    if run_trial(
                        reg_a,
                        reg_b,
                        reg_c,
                        &program,
                        &program[(program.len() - offset)..],
                    ) {
                        new_candidates.push(reg_a);
                    }
                }
            }
            candidates = new_candidates;
        }

        candidates.iter().min().unwrap().to_string()
    }
}
