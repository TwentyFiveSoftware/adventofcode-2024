use std::collections::{HashSet};
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    let graph = Graph::from_str(input).unwrap();

    println!("PART 1: {}", part1(&graph));
    println!("PART 2: {}", part2(&graph));
}

fn part1(graph: &Graph) -> usize {
    graph.unique_node_triplets().into_iter()
        .filter(|(node_a, node_b, node_c)|
            node_a.starts_with("t") || node_b.starts_with("t") || node_c.starts_with("t"))
        .filter(|triplet| graph.is_node_triplet_interconnected(&triplet))
        .count()
}

fn part2(graph: &Graph) -> String {
    let mut clique = graph.find_largest_clique().into_iter().collect::<Vec<String>>();
    clique.sort();
    clique.join(",")
}

struct Graph {
    connections: HashSet<(String, String)>,
}

impl Graph {
    fn nodes(&self) -> HashSet<String> {
        self.connections.iter().map(|(node, _)| node.clone()).collect()
    }

    fn find_largest_clique(&self) -> HashSet<String> {
        self.find_largest_clique_with_nodes(HashSet::new(), &self.nodes().into_iter().collect::<Vec<String>>())
    }

    fn find_largest_clique_with_nodes(&self, current_clique: HashSet<String>, remaining_nodes: &[String]) -> HashSet<String> {
        let mut largest_clique = current_clique.clone();

        for i in 0..remaining_nodes.len() {
            let node = remaining_nodes[i].clone();

            if !self.is_node_fully_connected_to_nodes(&node, &current_clique) {
                continue;
            }

            let mut larger_clique = current_clique.clone();
            larger_clique.insert(node);

            larger_clique = self.find_largest_clique_with_nodes(larger_clique, &remaining_nodes[(i + 1)..]);
            if larger_clique.len() > largest_clique.len() {
                largest_clique = larger_clique;
            }
        }

        largest_clique
    }

    fn is_node_fully_connected_to_nodes(&self, node: &str, other_nodes: &HashSet<String>) -> bool {
        other_nodes.iter().all(|other_node| self.connections.contains(&(node.to_string(), other_node.clone())))
    }

    fn unique_node_triplets(&self) -> Vec<(String, String, String)> {
        let nodes = self.nodes().into_iter().collect::<Vec<String>>();

        let mut unique_node_triplets = vec![];

        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                for k in (j + 1)..nodes.len() {
                    unique_node_triplets.push((nodes[i].clone(), nodes[j].clone(), nodes[k].clone()));
                }
            }
        }

        unique_node_triplets
    }

    fn is_node_triplet_interconnected(&self, (node_a, node_b, node_c): &(String, String, String)) -> bool {
        self.connections.contains(&(node_a.clone(), node_b.clone())) &&
            self.connections.contains(&(node_a.clone(), node_c.clone())) &&
            self.connections.contains(&(node_b.clone(), node_c.clone()))
    }
}

impl FromStr for Graph {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut connections: HashSet<(String, String)> = HashSet::new();

        s.lines()
            .filter_map(|line| {
                let [a, b] = line.split("-").collect::<Vec<&str>>()[..] else { return None; };
                Some((a.to_string(), b.to_string()))
            })
            .for_each(|(a, b)| {
                connections.insert((a.clone(), b.clone()));
                connections.insert((b, a));
            });

        Ok(Graph { connections })
    }
}
