use super::graph_core::Graph;

pub trait CycleDetection {
    fn has_cycle(&self) -> bool;
    fn is_cyclic_util(
        &self,
        node_id: usize,
        visited: &mut Vec<bool>,
        parent: Option<usize>,
    ) -> bool;
}

impl CycleDetection for Graph {
    fn has_cycle(&self) -> bool {
        let mut visited = vec![false; self.vertices];

        for v in 0..self.vertices {
            if !visited[v] && self.is_cyclic_util(v, &mut visited, None) {
                return true;
            }
        }

        false
    }

    fn is_cyclic_util(
        &self,
        node_id: usize,
        visited: &mut Vec<bool>,
        parent: Option<usize>,
    ) -> bool {
        visited[node_id] = true;

        for &adj_vertex in &self.adj_list[node_id] {
            if !visited[adj_vertex] {
                if self.is_cyclic_util(adj_vertex, visited, Some(node_id)) {
                    return true;
                }
            } else if adj_vertex != parent.unwrap() {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod cyclic_check_1_tests {
    use super::*;

    #[test]
    fn test_cyclic_is_cyclic_check() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);
        graph.add_edge(2, 3);

        assert!(graph.has_cycle());
    }

    #[test]
    fn test_cyclic_check_is_not_cyclic() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        assert!(!graph.has_cycle());
    }
}
