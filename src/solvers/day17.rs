pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let mut maybe_reg_a: Option<usize> = None;
        let mut maybe_reg_b: Option<usize> = None;
        let mut maybe_reg_c: Option<usize> = None;
        let mut program: Vec<usize> = Vec::new();

        for line in input.lines() {
            if line.starts_with("Register A: ") {
                maybe_reg_a = Some(line[12..].parse().unwrap());
            }
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

        let mut reg_a = maybe_reg_a.unwrap();
        let mut reg_b = maybe_reg_b.unwrap();
        let mut reg_c = maybe_reg_c.unwrap();
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

        let mut exec_ptr: usize = 0;
        let mut output: Vec<char> = Vec::new();

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
                    output.push(
                        char::from_digit(
                            (expand_combo(operand, reg_a, reg_b, reg_c) % 8) as u32,
                            8,
                        )
                        .unwrap(),
                    );
                    output.push(',');
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
        let _ = output.pop();
        output.iter().collect()
    }
}
