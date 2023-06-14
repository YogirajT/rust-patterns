use super::graph_core::Graph;
use std::collections::{HashSet};

trait TopSort {
    fn dfs(&self, start: usize, visited: &mut Vec<bool>, result: &mut Vec<usize>);
    fn topological_sort(&self) -> Vec<usize>;
}

impl TopSort for Graph {
    fn dfs(&self, node: usize, visited: &mut Vec<bool>, result: &mut Vec<usize>) {
        visited[node] = true;

        for &neighbor in &self.adj_list[node] {
            if !visited[neighbor] {
                self.dfs(neighbor, visited, result);
            }
        }

        result.push(node);
    }
    fn topological_sort(&self) -> Vec<usize> {
        let mut visited = vec![false; self.vertices];
        let mut result = Vec::new();

        for node in 0..self.vertices {
            if !visited[node] {
                self.dfs(node, &mut visited, &mut result);
            }
        }

        result.reverse();
        result
    }
}

#[cfg(test)]
mod top_sort_tests {
    use super::*;

    #[test]
    fn test_top_sort() {
        let mut graph = Graph::new(7);
        graph.add_edge(0, 1);
        graph.add_edge(0, 3);
        graph.add_edge(1, 2);
        graph.add_edge(1, 4);
        graph.add_edge(2, 5);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);
        graph.add_edge(4, 6);
        graph.add_edge(5, 6);

        let result = graph.topological_sort();
        println!("Topological Sort: {:?}", result);
    }
}
