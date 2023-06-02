use std::collections::HashSet;

struct Graph {
    vertices: usize,
    adj_list: Vec<Vec<usize>>,
}

impl Graph {
    fn new(vertices: usize) -> Graph {
        Graph {
            vertices,
            adj_list: vec![Vec::new(); vertices],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_list[u].push(v);
    }

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

fn main() {
    let mut graph = Graph::new(5);
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 4);

    graph.dfs(0);
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

        assert_eq!(Paint::RED, red_v16_sport_car.paint.unwrap());
        assert_eq!(Engine::V16, red_v16_sport_car.engine.unwrap());
        assert_eq!(Body::SPORT, red_v16_sport_car.body.unwrap());
    }
}