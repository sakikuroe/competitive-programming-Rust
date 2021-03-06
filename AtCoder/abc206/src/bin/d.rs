use proconio::input;

struct UnionFind {
        parent: Vec<usize>,
        rank: Vec<usize>,
        number_of_connected_components: Vec<usize>,
}
impl UnionFind {
        fn new(number_of_nodes: usize) -> Self {
                let parent: Vec<usize> = (0..number_of_nodes).collect::<Vec<usize>>();
                let rank: Vec<usize> = vec![0; number_of_nodes];
                let number_of_connected_components: Vec<usize> = vec![1; number_of_nodes];
                UnionFind {
                        parent,
                        rank,
                        number_of_connected_components,
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
                        self.number_of_connected_components[a]
                } else {
                        self.connected_components(a)
                }
        }
        fn unite(&mut self, x: usize, y: usize) {
                let x = self.root(x);
                let y = self.root(y);
                if x != y {
                        if self.rank[x] < self.rank[y] {
                                self.number_of_connected_components[y] +=
                                        self.connected_components(x);
                                self.parent[x] = y;
                        } else {
                                self.number_of_connected_components[x] +=
                                        self.connected_components(y);
                                self.parent[y] = x;
                                if self.rank[x] == self.rank[y] {
                                        self.rank[x] += 1;
                                }
                        }
                }
        }
}

fn main() {
        // Input
        input! {
                n: usize,
                a_list: [usize; n],
        }

        // Initialize
        let mut unionfind = UnionFind::new(200000);

        // Solve
        for i in 0..n {
                let a = a_list[i] - 1;
                let b = a_list[n - 1 - i] - 1;
                if a != b {
                        unionfind.unite(a, b)
                }
        }

        let mut ans = 0;

        for i in 0..200000 {
                if unionfind.is_root(i) {
                        ans += unionfind.connected_components(i) - 1;
                }
        }

        // Output
        println!("{}", ans);
}
