use petgraph::algo::toposort;
use petgraph::prelude::*;
use shared::*;
use std::collections::{HashMap, HashSet};
use std::fs;

fn problem1(input: String) {
    let parts: Vec<_> = input.split("\n\n").collect();
    let (rules, updates) = (parts[0].lines(), parts[1].lines());

    let mut graph = DiGraph::<u32, ()>::new();
    let mut node_map = HashMap::new();

    // Build the graph and ensure uniqueness of nodes
    for rule in rules {
        let indices = rule
            .split("|")
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let src_id = *node_map
            .entry(indices[0])
            .or_insert_with(|| graph.add_node(indices[0]));
        let dst_id = *node_map
            .entry(indices[1])
            .or_insert_with(|| graph.add_node(indices[1]));
        graph.add_edge(src_id, dst_id, ());
    }

    let mut correct_updates: Vec<Vec<u32>> = Vec::new();

    'outer: for updates_line in updates {
        let updates_line = updates_line
            .split(",")
            .map(|val| val.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        // Map each value in the input line to its NodeIndex (if it exists in the graph)
        let mut input_rank_map = HashMap::new();
        for (i, &val) in updates_line.iter().enumerate() {
            if let Some(&n_idx) = node_map.get(&val) {
                input_rank_map.insert(n_idx, i);
            }
        }

        // Check constraints: for each edge, if both src and dst are present in input_rank_map,
        // ensure that input_rank_map[src] < input_rank_map[dst].
        for edge in graph.edge_references() {
            let src = edge.source();
            let dst = edge.target();
            if let (Some(&src_pos), Some(&dst_pos)) =
                (input_rank_map.get(&src), input_rank_map.get(&dst))
            {
                if src_pos >= dst_pos {
                    // The given updates_line violates the ordering constraint
                    continue 'outer; // Skip adding this updates_line to correct_updates
                }
            }
        }

        // If we get here, this updates_line respects all constraints
        correct_updates.push(updates_line);
    }

    // Perform the final summation as described:
    let sum = correct_updates.iter().fold(0, |acc, x| {
        let sz = x.len();
        let index = (sz - 1) / 2;
        let value = x[index];
        value + acc
    });

    pretty_print_answer(sum);
}

fn problem2(input: String) {
    let parts: Vec<_> = input.split("\n\n").collect();
    let (rules, updates) = (parts[0].lines(), parts[1].lines());

    // Parse all rules into a structure that's easy to query per-update.
    // We'll store them in a HashMap<u32, Vec<u32>> representing adjacency:
    // For a page X, we'll keep a list of all Y such that X|Y is a rule.
    let mut adjacency: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut pages_in_rules = HashSet::new();

    for rule in rules {
        let parts: Vec<_> = rule.split("|").collect();
        let src = parts[0].parse::<u32>().unwrap();
        let dst = parts[1].parse::<u32>().unwrap();
        adjacency.entry(src).or_default().push(dst);
        pages_in_rules.insert(src);
        pages_in_rules.insert(dst);
    }

    let mut incorrect_updates: Vec<Vec<u32>> = Vec::new();

    'outer: for updates_line in updates {
        let update_pages = updates_line
            .split(",")
            .map(|val| val.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        // Check correctness:
        // For each rule X|Y that involves pages in this update, verify order.
        // Build a map from page -> index in update_pages for quick lookup.
        let mut position_map = HashMap::new();
        for (i, &page) in update_pages.iter().enumerate() {
            position_map.insert(page, i);
        }

        let mut is_correct = true;
        for (&src, targets) in &adjacency {
            if let Some(&src_pos) = position_map.get(&src) {
                for &dst in targets {
                    if let Some(&dst_pos) = position_map.get(&dst) {
                        // If this rule applies (both src and dst in this update),
                        // then check ordering constraint.
                        if src_pos >= dst_pos {
                            // Violation found
                            is_correct = false;
                            break;
                        }
                    }
                }
            }
            if !is_correct {
                break;
            }
        }

        if is_correct {
            // Update is already correct, do nothing
            continue 'outer;
        }

        // If we reach here, the update is incorrect and needs to be fixed.
        // We'll build a subgraph for just this update and sort it.
        let corrected_order = fix_update(&update_pages, &adjacency);
        incorrect_updates.push(corrected_order);
    }

    // Sum the middle page number of all corrected updates
    let sum = incorrect_updates.iter().fold(0, |acc, x| {
        let sz = x.len();
        let index = (sz - 1) / 2;
        let value = x[index];
        acc + value
    });

    pretty_print_answer(sum);
}

/// Build a minimal subgraph from the given adjacency (rules) and run topological sort.
/// This function returns a corrected ordering of the pages.
fn fix_update(update_pages: &[u32], adjacency: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    // Separate constrained (appear in adjacency) and unconstrained (no rules or not in adjacency)
    let constrained_nodes: Vec<u32> = update_pages
        .iter()
        .filter(|&&p| adjacency.contains_key(&p) || adjacency.values().any(|v| v.contains(&p)))
        .copied()
        .collect();

    let unconstrained_nodes: Vec<u32> = update_pages
        .iter()
        .filter(|&&p| !constrained_nodes.contains(&p))
        .copied()
        .collect();

    // Build a subgraph for just the constrained nodes
    let mut subgraph = DiGraph::<u32, ()>::new();
    let mut node_map = HashMap::new();

    // Add nodes
    for &page in &constrained_nodes {
        let idx = subgraph.add_node(page);
        node_map.insert(page, idx);
    }

    // Add edges
    for &page in &constrained_nodes {
        if let Some(targets) = adjacency.get(&page) {
            for &t in targets {
                if node_map.contains_key(&t) {
                    // Both page and t are in this update, so add an edge
                    let src_idx = node_map[&page];
                    let dst_idx = node_map[&t];
                    subgraph.add_edge(src_idx, dst_idx, ());
                }
            }
        }
    }

    // Topologically sort the subgraph
    let order = match toposort(&subgraph, None) {
        Ok(order) => order,
        Err(_) => {
            // If there's a cycle here, we can't fix it. For puzzle logic, we assume no such case.
            // Just return the original line for safety.
            return update_pages.to_vec();
        }
    };

    let mut corrected_constrained: Vec<u32> = order.iter().map(|&i| subgraph[i]).collect();

    // Append unconstrained nodes at the end (or handle differently as per puzzle logic)
    corrected_constrained.extend(unconstrained_nodes);

    corrected_constrained
}

fn main() {
    let args = Args::argparse();
    let input = fs::read_to_string(args.input).unwrap();

    match args.problem {
        1 => problem1(input),
        2 => problem2(input),
        _ => panic!("Not implemented"),
    }
}
