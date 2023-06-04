pub struct Graph {
    vertices: usize,
    pub adj_list: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(vertices: usize) -> Graph {
        Graph {
            vertices,
            adj_list: vec![Vec::new(); vertices],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_list[u].push(v);
        self.adj_list[v].push(u);
    }
}
