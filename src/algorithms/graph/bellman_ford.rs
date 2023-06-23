#[allow(dead_code)]
pub struct Edge {
    source: usize,
    target: usize,
    weight: i32,
}

pub fn bellman_ford(edges: &[Edge], num_vertices: usize, source: usize) -> Vec<Option<i32>> {
    let mut distance = vec![None; num_vertices];
    distance[source] = Some(0);

    for _ in 0..num_vertices - 1 {
        for edge in edges {
            if let Some(current_distance) = distance[edge.source] {
                let new_distance = current_distance + edge.weight;
                if distance[edge.target].map_or(true, |d| new_distance < d) {
                    distance[edge.target] = Some(new_distance);
                }
            }
        }
    }

    for edge in edges {
        if let Some(current_distance) = distance[edge.source] {
            let new_distance = current_distance + edge.weight;
            if distance[edge.target].map_or(true, |d| new_distance < d) {
                panic!("Negative cycle detected!");
            }
        }
    }

    distance
}

#[cfg(test)]
mod bellman_ford_tests {
    use super::*;

    #[test]
    fn test_bellman_ford() {
        let edges = vec![
            Edge {
                source: 0,
                target: 1,
                weight: 4,
            },
            Edge {
                source: 0,
                target: 2,
                weight: 2,
            },
            Edge {
                source: 1,
                target: 3,
                weight: 1,
            },
            Edge {
                source: 2,
                target: 1,
                weight: -1,
            },
            Edge {
                source: 2,
                target: 3,
                weight: 3,
            },
            Edge {
                source: 3,
                target: 4,
                weight: 2,
            },
        ];

        let num_vertices = 5;
        let source = 0;

        let distance = bellman_ford(&edges, num_vertices, source);

        assert_eq!(distance[0].unwrap(), 0);
        assert_eq!(distance[1].unwrap(), 1);
        assert_eq!(distance[2].unwrap(), 2);
        assert_eq!(distance[3].unwrap(), 2);
        assert_eq!(distance[4].unwrap(), 4);
    }
}
