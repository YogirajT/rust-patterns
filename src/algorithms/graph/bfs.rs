use std::collections::VecDeque;
use std::collections::HashSet;

use super::graph_core::Graph;

trait Bfs {
    fn bfs(&self, start: usize);
}

impl Bfs for Graph {
    fn bfs(&self, start: usize) {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(start);
        queue.push_back(start);

        while let Some(vertex) = queue.pop_front() {
            for &neighbour in &self.adj_list[vertex] {
                if !visited.contains(&neighbour) {
                    visited.insert(neighbour);
                    queue.push_back(neighbour);
                }
            }
        }
    }
}

#[cfg(test)]
mod bfs_tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);

        graph.bfs(0);
    }
}
