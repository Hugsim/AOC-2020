use petgraph::stable_graph::*;

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

use crate::util::*;

#[derive(Debug, Eq, PartialEq)]
struct Bag {
    colour: String,
    contents: Vec<(String, i64)>,
}

pub fn solve() -> (Option<i64>, Option<i64>) {
    let contents = read_file_to_vec::<Bag>("src/days/input/7");

    let mut g = StableGraph::<String, i64>::new();
    let mut nodes = HashMap::new();

    for Bag { colour, .. } in &contents {
        let n = g.add_node(colour.to_string());
        nodes.insert(colour.to_string(), n);
    }
    for Bag {
        contents,
        colour: fromColour,
    } in &contents
    {
        for (toColour, amount) in contents {
            if !nodes.contains_key(toColour) {
                let n = g.add_node(toColour.to_string());
                nodes.insert(toColour.to_string(), n);
            }
            g.add_edge(
                *nodes.get(toColour).unwrap(),
                *nodes.get(fromColour).unwrap(),
                *amount,
            );
        }
    }

    let start_node = *nodes.get("shinygold").unwrap();
    let reachable_nodes = reachable_nodes(&g, start_node);
    let sol1 = reachable_nodes.len();

    let sol2 = num_bags(&g, start_node);

    (Some(sol1 as i64 - 1), Some(sol2 - 1))
}

fn num_bags(graph: &StableGraph<String, i64>, from_node: NodeIndex) -> i64 {
    let mut sum = 1;
    for node in graph.neighbors_directed(from_node, petgraph::Incoming) {
        let weight = graph[graph.find_edge(node, from_node).unwrap()];
        let next_weight = num_bags(graph, node);
        sum += weight * next_weight;
    }
    sum
}

fn reachable_nodes(graph: &StableGraph<String, i64>, from_node: NodeIndex) -> HashSet<NodeIndex> {
    let mut res = HashSet::new();
    res.insert(from_node);
    for node in graph.neighbors(from_node) {
        res = &res | &reachable_nodes(graph, node);
    }
    res
}

impl FromStr for Bag {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace('.', "");
        let contents: Vec<Vec<String>> = s
            .split(" bags contain ")
            .map(|s| s.split(", ").map(|s| s.to_string()).collect())
            .collect();

        let colour: String = contents[0][0].chars().filter(|c| c != &' ').collect();
        let contents = &contents[1];
        let contents: Vec<(String, i64)> = contents
            .iter()
            .filter(|s| s != &"no other bags")
            .map(|s| {
                (
                    s[2..(s.len() - 4)].chars().filter(|c| c != &' ').collect(),
                    s.chars().collect::<Vec<char>>()[0].to_digit(10).unwrap() as i64,
                )
            })
            .collect();
        Ok(Bag { colour, contents })
    }
}
