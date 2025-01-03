use std::{collections::HashMap, rc::Rc, sync::Mutex};

pub struct Solver {}

enum OperationType {
    XOR,
    OR,
    AND,
}

impl OperationType {
    fn eval(&self, a: bool, b: bool) -> bool {
        match self {
            OperationType::XOR => a ^ b,
            OperationType::OR => a || b,
            OperationType::AND => a && b,
        }
    }
}

impl From<&str> for OperationType {
    fn from(value: &str) -> Self {
        match value {
            "XOR" => Self::XOR,
            "OR" => Self::OR,
            "AND" => Self::AND,
            _ => unreachable!(),
        }
    }
}

struct Operation {
    wire_a: String,
    wire_b: String,
    result_wire: String,
    wires: Rc<Mutex<HashMap<String, bool>>>,
    op_type: OperationType,
    dependants: Mutex<Vec<Rc<Operation>>>,
}

impl<'a> Drop for Operation {
    fn drop(&mut self) {
        let mut wires = self.wires.lock().unwrap();
        let a = *wires.get(&self.wire_a).unwrap();
        let b = *wires.get(&self.wire_b).unwrap();
        wires.insert(self.result_wire.clone(), self.op_type.eval(a, b));
    }
}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        // TODO: Implement
        let wire_mutex: Rc<Mutex<HashMap<String, bool>>> = Rc::new(Mutex::new(HashMap::new()));
        let mut ops: HashMap<String, Rc<Operation>> = HashMap::new();
        {
            let mut wires = wire_mutex.lock().unwrap();
            for line in input.lines() {
                let tokens: Vec<&str> = line.split(" ").collect();
                match tokens.len() {
                    2 => {
                        wires.insert(
                            tokens[0].strip_suffix(':').unwrap().to_string(),
                            tokens[1].parse::<u8>().unwrap() == 1,
                        );
                    }
                    5 => {
                        assert!(tokens[3] == "->");
                        let wire_a = tokens[0].to_string();
                        let wire_b = tokens[2].to_string();
                        let outout_wire = tokens[4].to_string();
                        ops.insert(
                            outout_wire.clone(),
                            Rc::new(Operation {
                                wire_a: wire_a,
                                wire_b: wire_b,
                                result_wire: outout_wire,
                                wires: wire_mutex.clone(),
                                op_type: tokens[1].into(),
                                dependants: Mutex::new(Vec::new()),
                            }),
                        );
                    }
                    _ => (),
                }
            }
        }

        for op in ops.values() {
            if let Some(dep_a) = ops.get(&op.wire_a) {
                dep_a.dependants.lock().unwrap().push(op.clone());
            }

            if let Some(dep_b) = ops.get(&op.wire_b) {
                dep_b.dependants.lock().unwrap().push(op.clone());
            }
        }

        // This is where the magic happens
        ops.clear();

        let mut acc: usize = 0;
        for (wire, powered) in wire_mutex.lock().unwrap().iter() {
            if wire.starts_with('z') {
                let bit_shift: usize = wire[1..].parse().unwrap();
                acc += (*powered as usize) << bit_shift;
            }
        }

        acc.to_string()
    }
}
