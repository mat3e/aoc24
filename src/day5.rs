use crate::graph::digraph::Digraph;

fn main() {
    dbg!(resolve_first(
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
    ));
}

fn resolve_first(input: &str) -> i32 {
    let (graph_input, pages) = input.split_once("\n\n").unwrap();
    let graph = &Digraph::new(graph_input, split_by_pipes);
    todo!()
}

fn split_by_pipes(line: &str) -> (&str, &str) {
    line.split_once('|').unwrap()
}

// Tried to make things "pluggable" and available for later days; am I doing Rust way for plug-ability?
mod graph {
    use std::collections::HashSet;
    use std::hash::Hash;

    // Educated guess what traits are for :D
    pub trait Graph<T: Hash + Eq> {
        fn has(&self, point: &T) -> bool;
        fn neighbors_of(&self, point: &T) -> Option<&HashSet<T>>;
    }

    pub mod digraph {
        use super::*;
        use std::collections::{HashMap, HashSet};
        use std::hash::Hash;

        pub struct Digraph<T> {
            adj: HashMap<T, HashSet<T>>,
        }

        impl<T: Hash + Eq> Digraph<T> {
            pub fn new(input: &str, transform_line: fn(&str) -> (T, T)) -> Digraph<T> {
                input.lines().fold(
                    Digraph {
                        adj: HashMap::new(),
                    },
                    |mut acc, line| {
                        let (from, to) = transform_line(line.trim());
                        acc.adj.entry(from).or_insert(HashSet::new()).insert(to);
                        acc
                    },
                )
            }
        }

        impl<T: Hash + Eq> Graph<T> for Digraph<T> {
            fn has(&self, point: &T) -> bool {
                self.adj.contains_key(point)
            }

            fn neighbors_of(&self, point: &T) -> Option<&HashSet<T>> {
                self.adj.get(point)
            }
        }
    }
}

mod pathfinding {
    pub mod bfs {
        use crate::graph::Graph;
        use std::collections::{HashMap, HashSet, LinkedList};
        use std::hash::Hash;

        pub fn bfs<T: Hash + Eq + Clone>(graph: &impl Graph<T>, from: &T, to: &T) -> Vec<T> {
            let mut queue = LinkedList::<&T>::new();
            queue.push_back(from);
            let mut visited = HashSet::<&T>::new();
            visited.insert(from);
            let mut connected_points = HashMap::<&T, &T>::new();
            while queue.front().is_some() {
                let current_point = queue.pop_front().unwrap();
                if current_point == to {
                    let mut path_to_start: Vec<&T> = vec![];
                    let mut current_path_point = current_point;
                    while current_path_point != from {
                        path_to_start.push(current_path_point);
                        current_path_point = connected_points.get(current_path_point).unwrap()
                    }
                    return path_to_start.into_iter().cloned().rev().collect();
                }
                if let Some(neighbors) = graph.neighbors_of(current_point) {
                    for neighbor in neighbors {
                        if visited.contains(neighbor) {
                            continue;
                        }
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                        connected_points.insert(neighbor, current_point);
                    }
                }
            }
            vec![]
        }
    }
}

mod tests {
    use super::graph::digraph::*;
    use super::pathfinding::bfs::*;
    use super::*;

    #[test]
    fn resolves_first() {
        assert_eq!(
            resolve_first(
                "47|53
                    97|13
                    97|61
                    97|47
                    75|29
                    61|13
                    75|53
                    29|13
                    97|29
                    53|29
                    61|53
                    97|53
                    61|29
                    47|13
                    75|47
                    97|75
                    47|61
                    75|61
                    47|29
                    75|13
                    53|13

                    75,47,61,53,29
                    97,61,53,29,13
                    75,29,13
                    75,97,47,61,53
                    61,13,29
                    97,13,75,29,47"
            ),
            143
        );
    }

    #[test]
    fn has_path() {
        // I started to pay respect to the type system; hope I follow the principles
        let graph_to_test = &Digraph::new(
            "47|53
                97|13
                97|61
                97|47",
            split_by_pipes,
        );
        assert!(!bfs(graph_to_test, &"47", &"53").is_empty());
        assert!(!bfs(graph_to_test, &"97", &"61").is_empty());
        assert!(bfs(graph_to_test, &"13", &"97").is_empty());
    }
}
