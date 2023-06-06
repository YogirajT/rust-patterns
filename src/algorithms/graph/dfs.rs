use std::collections::HashSet;
use super::graph_core::Graph;

trait Dfs {
    fn dfs_recursive(&self, vertex: usize, visited: &mut HashSet<usize>);
    fn dfs(&self, start: usize);
}

impl Dfs for Graph {
    fn dfs(&self, start: usize) {
        let mut visited = HashSet::new();
        self.dfs_recursive(start, &mut visited);
    }

    fn dfs_recursive(&self, vertex: usize, visited: &mut HashSet<usize>) {
        visited.insert(vertex);
        for &neighbour in &self.adj_list[vertex] {
            if !visited.contains(&neighbour) {
                self.dfs_recursive(neighbour, visited);
            }
        }
    }
}

#[cfg(test)]
mod dfs_tests {
    use super::*;

    #[test]
    fn test_dfs() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);

        graph.dfs(0);
    }
}
