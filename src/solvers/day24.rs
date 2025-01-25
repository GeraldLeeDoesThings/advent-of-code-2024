use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::Display,
    mem::swap,
    rc::Rc,
};

use regex::Regex;

pub struct Solver {}

#[derive(Debug, PartialEq, Eq)]
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

    fn _backwards(&self, expected: bool) -> Vec<(bool, bool)> {
        match self {
            OperationType::XOR if expected => vec![(true, false), (false, true)],
            OperationType::XOR => vec![(true, true), (false, false)],
            OperationType::OR if expected => vec![(true, false), (false, true), (true, true)],
            OperationType::OR => vec![(false, false)],
            OperationType::AND if expected => vec![(true, true)],
            OperationType::AND => vec![(true, false), (false, true), (false, false)],
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum OperationClass {
    RawAdd,
    RawCarry,
    CombinedCarry,
    FullCarry,
    Result,
    Error,
}

struct OperationContext {
    raw_add: HashMap<usize, String>,        // xnn XOR ynn
    raw_carry: HashMap<usize, String>,      // xnn AND ynn
    combined_carry: HashMap<usize, String>, // full_carry AND raw_add
    full_carry: HashMap<usize, String>,     // raw_carry OR combined_carry
    result: HashMap<usize, String>,         // full_carry XOR raw_add
    unused: HashSet<String>,
}

impl OperationContext {
    fn new() -> OperationContext {
        OperationContext {
            raw_add: HashMap::new(),
            raw_carry: HashMap::new(),
            combined_carry: HashMap::new(),
            full_carry: HashMap::new(),
            result: HashMap::new(),
            unused: HashSet::new(),
        }
    }

    fn get_map_for_op_mut<'a>(
        &'a mut self,
        op: &Operation,
    ) -> Option<&'a mut HashMap<usize, String>> {
        if op.is_incomplete() {
            return None;
        }
        Some(match op.class.unwrap() {
            OperationClass::RawAdd => &mut self.raw_add,
            OperationClass::RawCarry => &mut self.raw_carry,
            OperationClass::CombinedCarry => &mut self.combined_carry,
            OperationClass::FullCarry => &mut self.full_carry,
            OperationClass::Result => &mut self.result,
            OperationClass::Error => unreachable!(),
        })
    }

    fn track_op(&mut self, op: &Operation) -> Option<()> {
        if op.is_incomplete() {
            if !self.unused.insert(op.result_wire.clone()) {
                return None;
            }
        } else {
            let target_bit = op.target_bit.unwrap();
            let wire = op.result_wire.clone();
            let map = self.get_map_for_op_mut(op).unwrap();
            if !map.insert(target_bit, wire).is_none() {
                // Op was already being tracked
                return None;
            }
        }
        Some(())
    }

    fn untrack_op(&mut self, op: &Operation) -> Option<()> {
        if op.is_incomplete() {
            if !self.unused.remove(&op.result_wire) {
                return None;
            }
        } else {
            let target_bit = &op.target_bit.unwrap();
            let wire = &op.result_wire;
            let map = self.get_map_for_op_mut(op).unwrap();
            if !map
                .get(target_bit)
                .is_some_and(|found_wire| found_wire == wire)
            {
                return None;
            }
            map.remove(target_bit);
        }
        Some(())
    }

    fn from_ops(ops: &HashMap<String, Rc<RefCell<Operation>>>) -> Option<OperationContext> {
        let mut context = OperationContext::new();
        for (_wire, op_ref) in ops {
            let op = op_ref.try_borrow().unwrap();
            context.track_op(&op)?;
        }
        Some(context)
    }

    #[inline(always)]
    fn _count_good_wires(&self) -> usize {
        self.raw_add.len()
            + self.raw_carry.len()
            + self.combined_carry.len()
            + self.full_carry.len()
            + self.result.len()
    }

    #[inline(always)]
    fn _count_bad_wires(&self) -> usize {
        self.unused.len()
    }
}

enum OperationValidationError {
    ClassMissing,
    ClassIsError,
    TargetBitMissing,
    BadOpType,
    BadInput(String),
    BadOutput(String),
}

impl Display for OperationValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationValidationError::ClassMissing => write!(f, "Class has not been assigned."),
            OperationValidationError::ClassIsError => write!(f, "Class is assigned as 'Error'."),
            OperationValidationError::TargetBitMissing => {
                write!(f, "Target bit has not been assigned.")
            }
            OperationValidationError::BadOpType => {
                write!(f, "Operation type impossible for class.")
            }
            OperationValidationError::BadInput(wire) => {
                write!(f, "Input wire '{}' is invalid.", wire)
            }
            OperationValidationError::BadOutput(wire) => {
                write!(f, "Input wire '{}' is invalid.", wire)
            }
        }
    }
}

#[derive(Debug)]
struct Operation {
    wire_a: String,
    wire_b: String,
    result_wire: String,
    wires: Rc<RefCell<HashMap<String, bool>>>,
    op_type: OperationType,
    dependants: Vec<Rc<RefCell<Operation>>>,
    unknown_inputs: usize,
    class: Option<OperationClass>,
    target_bit: Option<usize>,
}

impl Operation {
    fn forwards(&mut self) {
        {
            let mut wires = self.wires.borrow_mut();
            let a = *wires.get(&self.wire_a).unwrap();
            let b = *wires.get(&self.wire_b).unwrap();
            wires.insert(self.result_wire.clone(), self.op_type.eval(a, b));
        }
        for mut dep in self.dependants.iter().map(|dep| dep.borrow_mut()) {
            dep.unknown_inputs -= 1;
            if dep.unknown_inputs == 0 {
                dep.forwards();
            }
        }
    }

    fn _backwards(
        &self,
        expected: bool,
        wire_map: &HashMap<String, Rc<RefCell<Operation>>>,
        cache: &mut HashMap<(String, bool), HashSet<String>>,
    ) -> HashSet<String> {
        if *self
            .wires
            .try_borrow()
            .unwrap()
            .get(&self.result_wire)
            .unwrap()
            == expected
        {
            return HashSet::new();
        }

        if let Some(cached) = cache.get(&(self.result_wire.clone(), expected)) {
            return cached.clone();
        }

        let mut result = HashSet::new();
        result.insert(self.result_wire.clone());
        for (a_target, b_target) in self.op_type._backwards(expected) {
            result.extend(
                wire_map
                    .get(&self.wire_a)
                    .map(|wire| {
                        wire.try_borrow()
                            .unwrap()
                            ._backwards(a_target, wire_map, cache)
                    })
                    .iter()
                    .flatten()
                    .chain(
                        wire_map
                            .get(&self.wire_b)
                            .map(|wire| {
                                wire.try_borrow()
                                    .unwrap()
                                    ._backwards(b_target, wire_map, cache)
                            })
                            .iter()
                            .flatten(),
                    )
                    .map(|s| s.clone()),
            );
        }

        cache.insert((self.result_wire.clone(), expected), result.clone());
        result
    }

    fn is_incomplete(&self) -> bool {
        return self.target_bit.is_none() && self.class.is_none_or(|c| c == OperationClass::Error);
    }

    fn make_complete(
        &mut self,
        class: OperationClass,
        target_bit: usize,
        context: &mut OperationContext,
    ) -> Option<()> {
        context.untrack_op(self)?;
        self.class = Some(class);
        self.target_bit = Some(target_bit);
        context.track_op(self)
    }

    fn _make_incomplete(&mut self, context: &mut OperationContext) -> Option<()> {
        context.untrack_op(self)?;
        self.class = None;
        self.target_bit = None;
        context.track_op(self)
    }

    fn make_error(&mut self, context: &mut OperationContext) -> Option<()> {
        context.untrack_op(self)?;
        self.class = Some(OperationClass::Error);
        self.target_bit = None;
        context.track_op(self)
    }

    fn logical_validate(
        &self,
        ops: &HashMap<String, Rc<RefCell<Operation>>>,
    ) -> Result<(), OperationValidationError> {
        if self.class.is_none() {
            return Err(OperationValidationError::ClassMissing);
        }
        if self.target_bit.is_none() {
            return Err(OperationValidationError::TargetBitMissing);
        }
        let target_bit = self.target_bit.unwrap();
        match self.class.unwrap() {
            OperationClass::RawAdd => {
                if self.op_type != OperationType::XOR {
                    return Err(OperationValidationError::BadOpType);
                }
                let x_target = format!("x{:02}", target_bit);
                let y_target = format!("y{:02}", target_bit);
                for wire in [&self.wire_a, &self.wire_b] {
                    if ops.get(wire).is_some() {
                        return Err(OperationValidationError::BadInput(wire.clone()));
                    }
                    if wire != &x_target && wire != &y_target {
                        return Err(OperationValidationError::BadInput(wire.clone()));
                    }
                }
                Ok(())
            }
            OperationClass::RawCarry => {
                if self.op_type != OperationType::AND {
                    return Err(OperationValidationError::BadOpType);
                }
                let x_target = format!("x{:02}", target_bit);
                let y_target = format!("y{:02}", target_bit);
                for wire in [&self.wire_a, &self.wire_b] {
                    if ops.get(wire).is_some() {
                        return Err(OperationValidationError::BadInput(wire.clone()));
                    }
                    if wire != &x_target && wire != &y_target {
                        return Err(OperationValidationError::BadInput(wire.clone()));
                    }
                }
                Ok(())
            }
            OperationClass::CombinedCarry => {
                if self.op_type != OperationType::AND {
                    return Err(OperationValidationError::BadOpType);
                }
                for (wire, maybe_op) in [&self.wire_a, &self.wire_b]
                    .iter()
                    .map(|&wire| (wire, ops.get(wire)))
                {
                    if maybe_op.is_none() {
                        return Err(OperationValidationError::BadInput(wire.clone()));
                    }
                    let op = maybe_op.unwrap().try_borrow().unwrap();
                    if op.target_bit.is_none_or(|tb| tb != target_bit) {
                        return Err(OperationValidationError::BadInput(wire.clone()));
                    }
                    if op.class.is_none_or(|class| {
                        class != OperationClass::FullCarry && class != OperationClass::RawAdd
                    }) {
                        return Err(OperationValidationError::BadInput(wire.clone()));
                    }
                }
                Ok(())
            }
            OperationClass::FullCarry => {
                let prev_bit = target_bit - 1;
                if prev_bit == 0 {
                    if self.op_type != OperationType::AND {
                        return Err(OperationValidationError::BadOpType);
                    }
                    for wire in [&self.wire_a, &self.wire_b] {
                        match wire.as_str() {
                            "x00" | "y00" => (),
                            _ => return Err(OperationValidationError::BadInput(wire.clone())),
                        }
                    }
                } else {
                    if self.op_type != OperationType::OR {
                        return Err(OperationValidationError::BadOpType);
                    }
                    for (wire, maybe_op) in [&self.wire_a, &self.wire_b]
                        .iter()
                        .map(|&wire| (wire, ops.get(wire)))
                    {
                        if maybe_op.is_none() {
                            return Err(OperationValidationError::BadInput(wire.clone()));
                        }
                        let op = maybe_op.unwrap().try_borrow().unwrap();
                        if op.target_bit.is_none_or(|tb| tb != prev_bit) {
                            return Err(OperationValidationError::BadInput(wire.clone()));
                        }
                        if op.class.is_none_or(|class| {
                            class != OperationClass::CombinedCarry
                                && class != OperationClass::RawCarry
                        }) {
                            return Err(OperationValidationError::BadInput(wire.clone()));
                        }
                    }
                }
                Ok(())
            }
            OperationClass::Result => {
                let target_wire = format!("z{:02}", target_bit);
                if self.result_wire == target_wire {
                    Ok(())
                } else {
                    Err(OperationValidationError::BadOutput(target_wire))
                }
            }
            OperationClass::Error => Err(OperationValidationError::ClassIsError),
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:?}) {} {} ({:?}) -> {}",
            self.class.unwrap(),
            self.wire_a,
            self.wire_b,
            self.op_type,
            self.result_wire
        )
    }
}

fn reset_dependency_counts(ops: &mut HashMap<String, Rc<RefCell<Operation>>>) {
    for op_cell in ops.values() {
        let mut op = op_cell.borrow_mut();
        op.unknown_inputs = 0;
        if let Some(_) = ops.get(&op.wire_a) {
            op.unknown_inputs += 1;
        }

        if let Some(_) = ops.get(&op.wire_b) {
            op.unknown_inputs += 1;
        }
    }
}

fn exec_run(
    wire_map: Rc<RefCell<HashMap<String, bool>>>,
    ops: &mut HashMap<String, Rc<RefCell<Operation>>>,
    x: usize,
    y: usize,
) -> bool {
    for i in 0..=44 {
        let x_wire = format!("x{:02}", i);
        let y_wire = format!("y{:02}", i);
        wire_map.borrow_mut().insert(x_wire, ((x >> i) & 1) == 1);
        wire_map.borrow_mut().insert(y_wire, ((y >> i) & 1) == 1);
    }

    reset_dependency_counts(ops);
    let initial_wires: Vec<String> = Vec::from_iter(ops.iter().filter_map(|(wire, op)| {
        if op.borrow_mut().unknown_inputs == 0 {
            Some(wire.clone())
        } else {
            None
        }
    }));

    for op_cell in initial_wires.iter().filter_map(|wire| ops.get(wire)) {
        let mut op = op_cell.borrow_mut();
        assert!(
            op.unknown_inputs == 0,
            "Bad op: {} {} ({:?}) -> {}",
            op.wire_a,
            op.wire_b,
            op.op_type,
            op.result_wire
        );
        op.forwards();
    }

    for op_cell in ops.values() {
        let op = op_cell.try_borrow().unwrap();
        assert!(
            op.unknown_inputs == 0,
            "Bad op: {} {} ({:?}) -> {}",
            op.wire_a,
            op.wire_b,
            op.op_type,
            op.result_wire
        );
    }

    let mut acc: usize = 0;
    let mut acc_x: usize = 0;
    let mut acc_y: usize = 0;
    for (wire, powered) in wire_map.try_borrow().unwrap().iter() {
        if wire.starts_with('z') {
            let bit_shift: usize = wire[1..].parse().unwrap();
            acc += (*powered as usize) << bit_shift;
        }
        if wire.starts_with('x') {
            let bit_shift: usize = wire[1..].parse().unwrap();
            acc_x += (*powered as usize) << bit_shift;
        }
        if wire.starts_with('y') {
            let bit_shift: usize = wire[1..].parse().unwrap();
            acc_y += (*powered as usize) << bit_shift;
        }
    }
    acc == acc_x + acc_y
}

fn _compute_dependencies(
    wire: &String,
    ops: &HashMap<String, Rc<RefCell<Operation>>>,
    deps: &mut HashMap<String, HashSet<String>>,
) -> HashSet<String> {
    if let Some(known) = deps.get(wire) {
        return known.clone();
    }

    if let Some(op) = ops.get(wire).map(|op| op.try_borrow().unwrap()) {
        let mut result: HashSet<String> = HashSet::from_iter(
            _compute_dependencies(&op.wire_a, ops, deps)
                .union(&_compute_dependencies(&op.wire_b, ops, deps))
                .map(|s| s.clone()),
        );
        result.insert(op.wire_a.clone());
        result.insert(op.wire_b.clone());

        deps.insert(wire.clone(), result.clone());
        result
    } else {
        deps.insert(wire.clone(), HashSet::new());
        HashSet::new()
    }
}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let wire_map: Rc<RefCell<HashMap<String, bool>>> = Rc::new(RefCell::new(HashMap::new()));
        let mut ops: HashMap<String, Rc<RefCell<Operation>>> = HashMap::new();
        {
            let mut wires = wire_map.borrow_mut();
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
                            Rc::new(RefCell::new(Operation {
                                wire_a: wire_a,
                                wire_b: wire_b,
                                result_wire: outout_wire,
                                wires: wire_map.clone(),
                                op_type: tokens[1].into(),
                                dependants: Vec::new(),
                                unknown_inputs: 0,
                                class: None,
                                target_bit: None,
                            })),
                        );
                    }
                    _ => (),
                }
            }
        }

        let mut context: OperationContext = OperationContext::from_ops(&mut ops).unwrap();

        for op_cell in ops.values() {
            let op = op_cell.borrow_mut();
            if let Some(dep_a) = ops.get(&op.wire_a) {
                dep_a.borrow_mut().dependants.push(op_cell.clone());
            }

            if let Some(dep_b) = ops.get(&op.wire_b) {
                dep_b.borrow_mut().dependants.push(op_cell.clone());
            }
        }

        reset_dependency_counts(&mut ops);

        let mut to_process: Vec<Rc<RefCell<Operation>>> = Vec::from_iter(
            ops.iter()
                .filter(|(_wire, op)| (*op).borrow_mut().unknown_inputs == 0)
                .map(|(_wire, op)| op.clone()),
        );

        let initial_wire_regex = Regex::new(r"(?:x|y)(\d{2})").unwrap();
        while let Some(op_ref) = to_process.pop() {
            let mut op = op_ref.borrow_mut();
            assert!(op.unknown_inputs == 0);
            assert!(op.class.is_none());
            if initial_wire_regex.is_match_at(&op.wire_a, 0)
                && initial_wire_regex.is_match_at(&op.wire_b, 0)
            {
                let a_input = &op.wire_a[0..=0];
                let b_input = &op.wire_b[0..=0];
                assert!(a_input != b_input);
                let a_bit_index: usize = op.wire_a[1..=2].parse().unwrap();
                let b_bit_index: usize = op.wire_b[1..=2].parse().unwrap();
                assert!(a_bit_index == b_bit_index);
                match op.op_type {
                    OperationType::XOR => {
                        if a_bit_index == 0 {
                            op.make_complete(OperationClass::Result, a_bit_index, &mut context);
                        } else {
                            op.make_complete(OperationClass::RawAdd, a_bit_index, &mut context);
                        }
                    }
                    OperationType::OR => unreachable!("Impossible logic gate"),
                    OperationType::AND => {
                        if a_bit_index == 0 {
                            op.make_complete(
                                OperationClass::FullCarry,
                                a_bit_index + 1,
                                &mut context,
                            );
                        } else {
                            op.make_complete(OperationClass::RawCarry, a_bit_index, &mut context);
                        }
                    }
                }
            } else {
                let a = ops.get(&op.wire_a).unwrap().try_borrow().unwrap();
                let b = ops.get(&op.wire_b).unwrap().try_borrow().unwrap();
                let a_class = a.class.unwrap();
                let b_class = b.class.unwrap();
                let mut counts: HashMap<OperationClass, usize> = HashMap::new();
                counts.insert(a_class, *counts.get(&a_class).unwrap_or(&0) + 1);
                counts.insert(b_class, *counts.get(&b_class).unwrap_or(&0) + 1);

                if counts.get(&OperationClass::RawAdd).is_some_and(|&c| c == 1)
                    && counts
                        .get(&OperationClass::FullCarry)
                        .is_some_and(|&c| c == 1)
                {
                    if a.target_bit == b.target_bit && a.target_bit.is_some() {
                        if op.op_type == OperationType::XOR {
                            op.make_complete(
                                OperationClass::Result,
                                a.target_bit.unwrap(),
                                &mut context,
                            );
                        } else if op.op_type == OperationType::AND {
                            op.make_complete(
                                OperationClass::CombinedCarry,
                                a.target_bit.unwrap(),
                                &mut context,
                            );
                        } else {
                            op.make_error(&mut context);
                        }
                    } else {
                        op.make_error(&mut context);
                    }
                } else if counts
                    .get(&OperationClass::CombinedCarry)
                    .is_some_and(|&c| c == 1)
                    && counts
                        .get(&OperationClass::RawCarry)
                        .is_some_and(|&c| c == 1)
                {
                    if a.target_bit == b.target_bit {
                        if op.op_type == OperationType::OR {
                            op.make_complete(
                                OperationClass::FullCarry,
                                a.target_bit.unwrap() + 1,
                                &mut context,
                            )
                            .unwrap();
                        } else {
                            op.make_error(&mut context);
                        }
                    } else {
                        op.make_error(&mut context);
                    }
                } else {
                    op.make_error(&mut context);
                }
            }
            if op.class.unwrap() != OperationClass::Error {
                for dep in &mut op.dependants {
                    dep.borrow_mut().unknown_inputs -= 1;
                    if dep.try_borrow().unwrap().unknown_inputs == 0 {
                        to_process.push(dep.clone());
                    }
                }
            }
        }

        let mut swapped_wires: Vec<(String, String)> = Vec::new();

        fn swap_wires(
            ops: &mut HashMap<String, Rc<RefCell<Operation>>>,
            pair: &(String, String),
            context: &mut OperationContext,
            swaps: &mut Vec<(String, String)>,
        ) {
            let (new_second_wire, first_rc) = ops.remove_entry(&pair.0).unwrap();
            let (new_first_wire, second_rc) = ops.remove_entry(&pair.1).unwrap();
            let mut first = first_rc.borrow_mut();
            let mut second = second_rc.borrow_mut();
            context.untrack_op(&first);
            context.untrack_op(&second);
            swap(&mut first.result_wire, &mut second.result_wire);
            swap(&mut first.dependants, &mut second.dependants);
            // swap(&mut first.class, &mut second.class);
            // swap(&mut first.target_bit, &mut second.target_bit);
            context.track_op(&first);
            context.track_op(&second);
            drop(first);
            drop(second);
            assert!(ops.insert(new_first_wire, first_rc).is_none());
            assert!(ops.insert(new_second_wire, second_rc).is_none());
            swaps.push(pair.clone());
        }

        let mut bad_raw_adds: Vec<String> = Vec::new();

        for i in 0..=44 {
            // Solve for bad results
            if i == 0 {
                assert!(context.result.get(&i).is_some());
                assert!(context.full_carry.get(&(i + 1)).is_some()); // Redundant
            } else {
                let radd = ops
                    .get(context.raw_add.get(&i).unwrap())
                    .clone()
                    .unwrap()
                    .try_borrow()
                    .unwrap();
                let target_out = format!("z{:02}", i);
                let mut needed_swap: Option<(String, String)> = None;
                for mut dep in radd.dependants.iter().map(|dep_ref| dep_ref.borrow_mut()) {
                    if dep.op_type == OperationType::XOR && dep.is_incomplete() {
                        dep.make_complete(OperationClass::Result, i, &mut context);
                        assert!(context.result.get(&i).is_some());
                        if dep.result_wire != target_out {
                            needed_swap = Some((dep.result_wire.clone(), target_out.clone()));
                        }
                    }
                }
                if let Some(result_wire) = context.result.get(&i) {
                    let result = ops.get(result_wire).unwrap().borrow_mut();
                    if result.result_wire != target_out && needed_swap.is_none() {
                        needed_swap = Some((result.result_wire.clone(), target_out.clone()));
                    }
                } else {
                    bad_raw_adds.push(radd.result_wire.clone());
                }
                if let Some(swap) = needed_swap {
                    drop(radd);
                    swap_wires(&mut ops, &swap, &mut context, &mut swapped_wires);
                }
            }
        }

        for bad_radd in bad_raw_adds {
            let radd = ops.get(&bad_radd).unwrap().try_borrow().unwrap();
            let target_bit = radd.target_bit.unwrap();
            let target_out = format!("z{:02}", target_bit);
            let best_candidate = context
                .unused
                .iter()
                .max_by_key(|&candidate_wire| {
                    let candidate = ops.get(candidate_wire).unwrap().try_borrow().unwrap();
                    let mut score = 0;
                    if candidate.result_wire == target_out {
                        score += 1;
                    }
                    if candidate.op_type != OperationType::XOR {
                        score = 0;
                    } else {
                        score += 1;
                    }
                    score
                })
                .unwrap();
            let mut best = ops.get(best_candidate).unwrap().borrow_mut();
            let mut needed_swaps: Vec<(String, String)> = Vec::new();
            if best.result_wire != target_out {
                assert!(context.unused.contains(&target_out));
                needed_swaps.push((best_candidate.clone(), target_out));
            }
            if ops.get(&best.wire_a).unwrap().try_borrow().unwrap().op_type != OperationType::OR {
                needed_swaps.push((best.wire_a.clone(), radd.result_wire.clone()));
            }
            if ops.get(&best.wire_b).unwrap().try_borrow().unwrap().op_type != OperationType::OR {
                assert!(needed_swaps.len() < 2);
                needed_swaps.push((best.wire_b.clone(), radd.result_wire.clone()));
            }
            best.make_complete(OperationClass::Result, target_bit, &mut context);
            drop(radd);
            drop(best);
            for swap in needed_swaps {
                swap_wires(&mut ops, &swap, &mut context, &mut swapped_wires);
            }
        }

        for i in 0..=44 {
            if i == 0 {
                assert!(context.result.get(&i).is_some());
                assert!(context.full_carry.get(&(i + 1)).is_some()); // Redundant
            } else {
                let radd = ops
                    .get(context.raw_add.get(&i).unwrap())
                    .unwrap()
                    .try_borrow()
                    .unwrap();
                let mut ccarry = context.combined_carry.get(&i).map(|s| s.clone());
                if ccarry.is_none() {
                    for dep_ref in &radd.dependants {
                        let mut dep = dep_ref.borrow_mut();
                        if dep.op_type == OperationType::AND {
                            assert!(ccarry.is_none());
                            if dep.is_incomplete() {
                                dep.make_complete(OperationClass::CombinedCarry, i, &mut context);
                                ccarry = Some(dep.result_wire.clone());
                            }
                        }
                    }
                }
            }
        }

        for i in 0..=44 {
            if i == 0 {
            } else {
                if context.full_carry.get(&(i + 1)).is_some() {
                    continue;
                }
                let rcarry = ops
                    .get(context.raw_carry.get(&i).unwrap())
                    .unwrap()
                    .try_borrow()
                    .unwrap();
                let ccarry = ops
                    .get(context.combined_carry.get(&i).unwrap())
                    .unwrap()
                    .try_borrow()
                    .unwrap();
                let rcarry_deps: Vec<String> = rcarry
                    .dependants
                    .iter()
                    .map(|dep_ref| dep_ref.try_borrow().unwrap().result_wire.clone())
                    .collect();
                let ccarry_deps: Vec<String> = ccarry
                    .dependants
                    .iter()
                    .map(|dep_ref| dep_ref.try_borrow().unwrap().result_wire.clone())
                    .collect();
                assert!(rcarry_deps == ccarry_deps);
                assert!(rcarry_deps.len() == 1);
                let mut fcarry = ops.get(&rcarry_deps[0]).unwrap().borrow_mut();
                assert!(fcarry.is_incomplete());
                fcarry.make_complete(OperationClass::FullCarry, i + 1, &mut context);
            }
        }

        for (_wire, op_cell) in &ops {
            let op = op_cell.try_borrow().unwrap();
            let val = op.logical_validate(&ops);
            assert!(val.is_ok(), "{}", val.unwrap_err());
        }

        // Validate
        for x_shift in 0..=44 {
            for y_shift in 0..=44 {
                for extra_x_shift in 0..=44 {
                    assert!(exec_run(
                        wire_map.clone(),
                        &mut ops,
                        1 << x_shift + 1 << extra_x_shift,
                        1 << y_shift
                    ));
                    assert!(exec_run(
                        wire_map.clone(),
                        &mut ops,
                        !(1 << x_shift + 1 << extra_x_shift),
                        !(1 << y_shift)
                    ));
                }
            }
        }

        let mut sorted_swaps: Vec<String> = swapped_wires
            .drain(..)
            .flat_map(|swap| [swap.0, swap.1])
            .collect();
        sorted_swaps.sort();
        sorted_swaps.join(",")
    }
}
