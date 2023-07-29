#![allow(dead_code)]
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

type Vertex = u32;
type Weight = u32;
type Edge = (Vertex, Vertex, Weight);

fn prim_algorithm(edges: &[Edge], num_vertices: usize) -> Vec<Edge> {
    let mut ajd_list: HashMap<Vertex, Vec<(Vertex, Weight)>> = HashMap::new();
    for &(src, dest, weight) in edges {
        ajd_list.entry(src).or_default().push((dest, weight));
        ajd_list.entry(dest).or_default().push((src, weight));
    }

    let mut heap = BinaryHeap::new();

    let mut visited = vec![false; num_vertices];

    visited[0] = true;

    for &(dest, weight) in &ajd_list[&0] {
        heap.push(Reverse((weight, 0, dest)));
    }

    let mut mst = Vec::new();

    while let Some(Reverse((weight, src, dest))) = heap.pop() {
        if visited[dest as usize] {
            continue;
        }

        visited[dest as usize] = true;
        mst.push((src, dest, weight));

        for &(next_dest, next_w) in &ajd_list[&dest] {
            if !visited[next_dest as usize] {
                heap.push(Reverse((next_w, dest, next_dest)));
            }
        }
    }

    mst
}

#[cfg(test)]
mod prim_tests {
    use super::*;

    #[test]
    fn test_prim() {
        let edges = [(0, 1, 5), (0, 2, 2), (0, 3, 3), (1, 2, 4), (2, 3, 1)];

        let answers: [(u32, u32, u32); 3] = [(0, 2, 2), (2, 3, 1), (2, 1, 4)];

        let num_vertices = 4;

        let mst = prim_algorithm(&edges, num_vertices);

        for n in 0..3 {
            let mst_node = mst[n];
            let answer = answers[n];
            assert_eq!(answer, mst_node);
        }
    }
}
