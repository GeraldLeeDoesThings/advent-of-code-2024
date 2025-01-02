use std::collections::{BTreeSet, HashMap, HashSet};

pub struct Solver {}

impl crate::Solver for Solver {
    fn solve(&self, input: &String) -> String {
        let mut computers: HashSet<String> = HashSet::new();
        let mut connections: HashMap<String, BTreeSet<String>> = HashMap::new();

        for line in input.lines() {
            let mut parts = line.split('-');
            let comp_a = parts.next().unwrap().to_string();
            let comp_b = parts.next().unwrap().to_string();
            computers.insert(comp_a.clone());
            computers.insert(comp_b.clone());

            if let Some(conns_a) = connections.get_mut(&comp_a) {
                conns_a.insert(comp_b.clone());
            } else {
                connections.insert(comp_a.clone(), BTreeSet::from([comp_b.clone()]));
            }

            if let Some(conns_b) = connections.get_mut(&comp_b) {
                conns_b.insert(comp_a);
            } else {
                connections.insert(comp_b, BTreeSet::from([comp_a]));
            }
        }

        let mut cliques: BTreeSet<BTreeSet<String>> = BTreeSet::from_iter(
            computers
                .iter()
                .map(|computer| BTreeSet::from_iter([computer.clone()])),
        );
        let mut seen_cliques: HashSet<BTreeSet<String>> = HashSet::new();
        let mut maximal_cliques: HashSet<BTreeSet<String>> = HashSet::new();

        // Fast enough clique finding algorithm
        while let Some(clique) = cliques.pop_last() {
            let mut found_any = false;
            for comp in &computers {
                let connected = connections.get(comp).unwrap();
                if clique.is_subset(connected) && !clique.contains(comp) {
                    let mut expanded = clique.clone();
                    expanded.insert(comp.clone());
                    if !seen_cliques.contains(&expanded) {
                        found_any = true;
                        seen_cliques.insert(expanded.clone());
                        cliques.insert(expanded);
                    }
                }
            }
            if !found_any {
                maximal_cliques.insert(clique);
            }
        }
        maximal_cliques
            .iter()
            .max_by_key(|clique| clique.len())
            .unwrap()
            .iter()
            .map(|computer| computer.clone())
            .collect::<Vec<String>>()
            .join(",")
    }
}
