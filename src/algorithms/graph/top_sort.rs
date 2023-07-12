use super::graph_core::Graph;
use std::collections::{HashMap, VecDeque};

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


// Kahn's algo
pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut in_degree: Vec<i32> = vec![0; num_courses as usize];

    for prerequisite in prerequisites.iter() {
        let course = prerequisite[0];
        let prereq = prerequisite[1];
        graph.entry(prereq).or_insert(Vec::new()).push(course);
        in_degree[course as usize] += 1;
    }

    let mut queue: VecDeque<i32> = VecDeque::new();
    for course in 0..num_courses {
        if in_degree[course as usize] == 0 {
            queue.push_back(course);
        }
    }

    let mut course_order: Vec<i32> = Vec::new();
    while let Some(course) = queue.pop_front() {
        course_order.push(course);

        if let Some(neighbors) = graph.get(&course) {
            for &neighbor in neighbors.iter() {
                in_degree[neighbor as usize] -= 1;
                if in_degree[neighbor as usize] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    if course_order.len() == num_courses as usize {
        course_order
    } else {
        Vec::new()
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

    #[test]
    fn test_top_sort_example() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];
        assert_eq!(vec![0, 1], find_order(num_courses, prerequisites));

        let num_courses = 4;
        let prerequisites = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
        assert_eq!(vec![0, 1, 2, 3], find_order(num_courses, prerequisites));

        let num_courses = 1;
        let prerequisites = Vec::new();
        assert_eq!(vec![0], find_order(num_courses, prerequisites));
    }
}
