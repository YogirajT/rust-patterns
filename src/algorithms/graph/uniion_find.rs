pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        let mut parent = vec![0; size];
        let rank = vec![0; size];

        for i in 0..size {
            parent[i] = i;
        }

        UnionFind { parent, rank }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
    }
}


mod union_find_tests {
    use super::*;


    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::new(5);

        uf.union(0, 2);
        uf.union(1, 3);
        uf.union(3, 4);

        assert_eq!(uf.find(0), 2);
        assert_eq!(uf.find(1), 3);
        assert_eq!(uf.find(2), 2);
        assert_eq!(uf.find(3), 3);
        assert_eq!(uf.find(4), 3);
    }

    #[test]
    fn test_union_find_disjoint_sets() {
        let mut uf = UnionFind::new(5);

        for i in 0..5 {
            assert_eq!(uf.find(i), i);
        }
    }

    #[test]
    fn test_union_find_single_union() {
        let mut uf = UnionFind::new(5);

        uf.union(0, 1);

        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.find(1), 0);
        assert_eq!(uf.find(2), 2);
        assert_eq!(uf.find(3), 3);
        assert_eq!(uf.find(4), 4);
    }
}
