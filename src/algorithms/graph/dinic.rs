#![allow(dead_code)]
use std::cmp;


// Dinics's algorithm for maximum flow

#[derive(Clone)]
struct Edge {
    to: usize,
    cap: i32,
    rev: usize,
}

struct Graph {
    edges: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(n: usize) -> Graph {
        let edges = vec![Vec::new(); n];
        Graph { edges }
    }

    fn add_edge(&mut self, from: usize, to: usize, cap: i32) {
        let forward = Edge {
            to,
            cap,
            rev: self.edges[to].len(),
        };
        let backward = Edge {
            to: from,
            cap: 0,
            rev: self.edges[from].len(),
        };
        self.edges[from].push(forward);
        self.edges[to].push(backward);
    }

    fn dfs(&mut self, v: usize, t: usize, f: i32, visited: &mut [bool]) -> i32 {
        if v == t {
            return f;
        }
        visited[v] = true;
        for i in 0..self.edges[v].len() {
            let e = self.edges[v][i].clone();
            if !visited[e.to] && e.cap > 0 {
                let d = self.dfs(e.to, t, cmp::min(f, e.cap), visited);
                if d > 0 {
                    self.edges[v][i].cap -= d;
                    self.edges[e.to][e.rev].cap += d;
                    return d;
                }
            }
        }
        0
    }

    fn max_flow(&mut self, s: usize, t: usize) -> i32 {
        let mut flow = 0;
        loop {
            let mut visited = vec![false; self.edges.len()];
            let f = self.dfs(s, t, std::i32::MAX, &mut visited);
            if f == 0 {
                return flow;
            }
            flow += f;
        }
    }
}

#[cfg(test)]
mod algo_tests {
    use super::Graph;


    #[test]
    fn dinics_test() {
        let n = 4;
        let s = 0;
        let t = n - 1;
        let mut graph = Graph::new(n);
        let edges = vec![(0, 1, 2), (0, 2, 3), (1, 2, 1), (1, 3, 1), (2, 3, 2)];
        for (u, v, c) in edges {
            graph.add_edge(u, v, c);
        }
        let max_flow = graph.max_flow(s, t);

        assert_eq!(3, max_flow);
    }
}
