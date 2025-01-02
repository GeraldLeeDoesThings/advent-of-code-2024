use std::collections::{HashMap, HashSet};

pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let mut computers: HashSet<String> = HashSet::new();
        let mut connections: HashMap<String, HashSet<String>> = HashMap::new();

        for line in input.lines() {
            let mut parts = line.split('-');
            let comp_a = parts.next().unwrap().to_string();
            let comp_b = parts.next().unwrap().to_string();
            computers.insert(comp_a.clone());
            computers.insert(comp_b.clone());

            if let Some(conns_a) = connections.get_mut(&comp_a) {
                conns_a.insert(comp_b.clone());
            } else {
                connections.insert(comp_a.clone(), HashSet::from([comp_b.clone()]));
            }

            if let Some(conns_b) = connections.get_mut(&comp_b) {
                conns_b.insert(comp_a);
            } else {
                connections.insert(comp_b, HashSet::from([comp_a]));
            }
        }

        let mut triplets: HashSet<[String; 3]> = HashSet::new();

        for comp_a in &computers {
            let a_conns = connections.get(comp_a).unwrap();
            for comp_b in a_conns {
                let b_conns = connections.get(comp_b).unwrap();
                for mutual in a_conns.intersection(b_conns) {
                    let mut triplet = [comp_a.clone(), comp_b.clone(), mutual.clone()];
                    triplet.sort();
                    triplets.insert(triplet);
                }
            }
        }
        triplets
            .iter()
            .filter(|triplet| triplet.iter().any(|comp| comp.starts_with('t')))
            .count()
            .to_string()
    }
}
