use cargo_snippet::snippet;

#[snippet(name = "unionfind")]
struct UnionFind {
        parent: Vec<usize>,
        rank: Vec<usize>,
        size: Vec<usize>,
}
#[snippet(name = "unionfind")]
impl UnionFind {
        fn new(number_of_nodes: usize) -> Self {
                let parent  = (0..number_of_nodes).collect::<Vec<usize>>();
                let rank = vec![0; number_of_nodes];
                let size  = vec![1; number_of_nodes];
                UnionFind {
                        parent,
                        rank,
                        size,
                }
        }

        fn is_root(&mut self, x: usize) -> bool {
                self.parent[x] == x
        }

        fn root(&mut self, x: usize) -> usize {
                if self.is_root(x) {
                        x
                } else {
                        self.parent[x] = self.root(self.parent[x]);
                        self.parent[x]
                }
        }

        fn connected_components(&mut self, x: usize) -> usize {
                let a = self.root(x);
                if self.is_root(a) {
                        self.size[a]
                } else {
                        self.connected_components(a)
                }
        }

        fn unite(&mut self, x: usize, y: usize) {
                let x = self.root(x);
                let y = self.root(y);
                if x != y {
                        if self.rank[x] < self.rank[y] {
                                self.size[y] +=
                                        self.connected_components(x);
                                self.parent[x] = y;
                        } else {
                                self.size[x] +=
                                        self.connected_components(y);
                                self.parent[y] = x;
                                if self.rank[x] == self.rank[y] {
                                        self.rank[x] += 1;
                                }
                        }
                }
        }

        fn same(&mut self, x: usize, y: usize) -> bool {
                self.root(x) == self.root(y)
        }
}

fn main() {
        println!("Hello, world!");
}
