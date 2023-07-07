use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    id: usize,
    distance: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra(graph: &[Vec<(usize, i32)>], source: usize) -> Vec<i32> {
    let mut distances = vec![i32::max_value(); graph.len()];
    let mut heap = BinaryHeap::new();

    distances[source] = 0;
    heap.push(Node {
        id: source,
        distance: 0,
    });

    while let Some(Node { id, distance }) = heap.pop() {
        if distance > distances[id] {
            continue;
        }

        for &(neighbor, weight) in &graph[id] {
            let new_distance = distance + weight;
            if new_distance < distances[neighbor] {
                distances[neighbor] = new_distance;
                heap.push(Node {
                    id: neighbor,
                    distance: new_distance,
                });
            }
        }
    }

    distances
}

#[cfg(test)]
mod dijkstras_tests {
    use super::*;

    #[test]
    fn test_dijkstras() {
        let graph = vec![
            vec![(1, 7), (2, 9), (5, 14)],
            vec![(0, 7), (2, 10), (3, 15)],
            vec![(0, 9), (1, 10), (3, 11), (5, 2)],
            vec![(1, 15), (2, 11), (4, 6)],
            vec![(3, 6), (5, 9)],
            vec![(0, 14), (2, 2), (4, 9)],
        ];

        let source = 0;
        let distances = dijkstra(&graph, source);

        let expected_distances = vec![0, 7, 9, 20, 20, 11];

        assert_eq!(distances, expected_distances);
    }
}
