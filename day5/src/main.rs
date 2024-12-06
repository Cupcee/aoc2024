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

/// Build a subgraph for just the given update and topologically sort it.
/// Then reorder the pages accordingly.
fn fix_update(
    graph: &DiGraph<u32, ()>,
    node_map: &HashMap<u32, NodeIndex>,
    updates_line: &[u32],
) -> Vec<u32> {
    // Identify constrained and unconstrained nodes in this update
    let constrained_nodes: HashSet<u32> = updates_line
        .iter()
        .filter(|val| node_map.contains_key(val))
        .copied()
        .collect();

    let unconstrained_nodes: Vec<u32> = updates_line
        .iter()
        .filter(|val| !node_map.contains_key(val))
        .copied()
        .collect();

    // Build a subgraph containing only the constrained nodes and the edges between them
    // We need a mapping from the old NodeIndexes to new ones in the subgraph
    let mut subgraph = DiGraph::<u32, ()>::new();
    let mut subgraph_node_map = HashMap::new();

    // Add nodes
    for &val in &constrained_nodes {
        let old_nidx = node_map[&val];
        let new_nidx = subgraph.add_node(val);
        subgraph_node_map.insert(old_nidx, new_nidx);
    }

    // Add edges
    for edge in graph.edge_references() {
        let src = edge.source();
        let dst = edge.target();
        if subgraph_node_map.contains_key(&src) && subgraph_node_map.contains_key(&dst) {
            subgraph.add_edge(subgraph_node_map[&src], subgraph_node_map[&dst], ());
        }
    }

    // Perform a toposort on the subgraph
    let order = match toposort(&subgraph, None) {
        Ok(order) => order,
        Err(err) => {
            // If we have a cycle here, we can't produce a valid order.
            // In the context of the puzzle, this shouldn't happen if input is correct.
            // For safety, just return the original line or handle gracefully.
            eprintln!("No valid ordering possible for this update: {:?}", err);
            return updates_line.to_vec();
        }
    };

    let mut corrected_constrained: Vec<u32> = order.iter().map(|&n_idx| subgraph[n_idx]).collect();

    // Append unconstrained nodes at the end (or handle differently if needed)
    corrected_constrained.extend(unconstrained_nodes);

    corrected_constrained
}

fn problem2(input: String) {
    let parts: Vec<_> = input.split("\n\n").collect();
    let (rules, updates) = (parts[0].lines(), parts[1].lines());

    let mut graph = DiGraph::<u32, ()>::new();
    let mut node_map = HashMap::new();

    // Build the global graph and map each page to its node index
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

    // Collect updates that are incorrectly ordered
    let mut incorrect_updates: Vec<Vec<u32>> = Vec::new();

    'outer: for updates_line in updates {
        let updates_line = updates_line
            .split(",")
            .map(|val| val.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        // Create a mapping of NodeIndex to position in the update line for constrained pages
        let mut input_rank_map = HashMap::new();
        for (i, &val) in updates_line.iter().enumerate() {
            if let Some(&n_idx) = node_map.get(&val) {
                input_rank_map.insert(n_idx, i);
            }
        }

        // Check if this line is correct:
        // An update is correct if for every edge (src -> dst) where both pages appear,
        // src_pos < dst_pos. If any violation is found, it's incorrect.
        for edge in graph.edge_references() {
            let src = edge.source();
            let dst = edge.target();
            if let (Some(&src_pos), Some(&dst_pos)) =
                (input_rank_map.get(&src), input_rank_map.get(&dst))
            {
                if src_pos >= dst_pos {
                    // Found a violation, so this update is incorrect.
                    // We'll fix it by building a subgraph and toposorting it.
                    let fixed_line = fix_update(&graph, &node_map, &updates_line);
                    incorrect_updates.push(fixed_line);
                    continue 'outer; // Move on to the next update after fixing this one.
                }
            }
        }

        // If we get here, no violation was found. This update is already correct.
    }

    // Summation logic similar to problem1:
    let sum = incorrect_updates.iter().fold(0, |acc, x| {
        let sz = x.len();
        let index = (sz - 1) / 2;
        let value = x[index];
        value + acc
    });

    pretty_print_answer(sum);
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
