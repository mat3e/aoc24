use crate::graph::digraph::Digraph;
use crate::graph::Graph;

fn main() {
    dbg!(resolve_second(
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

fn resolve_second(input: &str) -> i32 {
    todo!()
}

fn resolve_first(input: &str) -> i32 {
    let (graph_input, pages) = input.split_once("\n\n").unwrap();
    let graph = Digraph::new(graph_input, split_by_pipes);
    pages.lines().fold(0, |acc, line| {
        let line_vec = parse_page(line);
        if line_vec
            .windows(2)
            .any(|pair| in_wrong_order(&graph, pair[0], pair[1]))
        {
            return acc;
        }
        acc + line_vec[line_vec.len() / 2].parse::<i32>().unwrap()
    })
}

fn parse_page(input: &str) -> Vec<&str> {
    input
        .split(',')
        .map(|page| page.trim())
        .collect::<Vec<&str>>()
}

fn split_by_pipes(line: &str) -> (&str, &str) {
    line.split_once('|').unwrap()
}

fn in_wrong_order(graph: &impl Graph<str>, left: &str, right: &str) -> bool {
    graph
        .neighbors_of(left)
        .filter(|neighbors| neighbors.contains(&right))
        .map(|_| false)
        .unwrap_or_else(|| {
            graph
                .neighbors_of(right)
                .filter(|neighbors| neighbors.contains(&left))
                .is_some()
        })
}

// Tried to make things "pluggable" and available for later days; am I doing Rust way for plug-ability?
mod graph {
    use std::collections::HashSet;
    use std::hash::Hash;

    // Educated guess what traits are for :D
    pub trait Graph<T: Hash + Eq + ?Sized> {
        fn has(&self, point: &T) -> bool;
        fn neighbors_of(&self, point: &T) -> Option<&HashSet<&T>>;
    }

    pub mod digraph {
        use super::*;
        use std::collections::{HashMap, HashSet};
        use std::hash::Hash;

        pub struct Digraph<'graph_lf, T: ?Sized> {
            adj: HashMap<&'graph_lf T, HashSet<&'graph_lf T>>,
        }

        impl<'graph_lf, T: Hash + Eq + ?Sized> Digraph<'graph_lf, T> {
            pub fn new(
                input: &'graph_lf str,
                transform_line: fn(&'graph_lf str) -> (&'graph_lf T, &'graph_lf T),
            ) -> Digraph<T> {
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

        impl<'graph_lf, T: Hash + Eq + ?Sized> Graph<T> for Digraph<'graph_lf, T> {
            fn has(&self, point: &T) -> bool {
                self.adj.contains_key(point)
            }

            fn neighbors_of(&self, point: &T) -> Option<&HashSet<&T>> {
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

        pub fn bfs<'a, T: Hash + Eq + ?Sized>(
            graph: &'a impl Graph<T>,
            from: &'a T,
            to: &'a T,
        ) -> Option<Vec<&'a T>> {
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
                    path_to_start.reverse();
                    return Some(path_to_start);
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
            None
        }
    }
}

mod tests {
    use super::graph::digraph::*;
    use super::pathfinding::bfs::*;
    use super::*;

    #[test]
    fn resolves_second() {
        assert_eq!(
            resolve_second(
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
                    75,97,47,61,53
                    61,13,29
                    75,29,13
                    97,13,75,29,47"
            ),
            47 + 29 + 47
        );
    }

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
                    75,97,47,61,53
                    61,13,29
                    75,29,13
                    97,13,75,29,47"
            ),
            61 + 53 + 29
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
        assert!(bfs(graph_to_test, &"47", &"53").is_some());
        assert!(bfs(graph_to_test, &"97", &"61").is_some());
        assert!(bfs(graph_to_test, &"13", &"97").is_none());
    }
}
