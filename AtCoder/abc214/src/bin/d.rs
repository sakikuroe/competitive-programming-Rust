use proconio::input;

struct UnionFind {
        parent: Vec<usize>,
        rank: Vec<usize>,
        size: Vec<usize>,
}
impl UnionFind {
        fn new(number_of_nodes: usize) -> Self {
                let parent = (0..number_of_nodes).collect::<Vec<usize>>();
                let rank = vec![0; number_of_nodes];
                let size = vec![1; number_of_nodes];
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
        fn size(&mut self, x: usize) -> usize {
                let a = self.root(x);
                if self.is_root(a) {
                        self.size[a]
                } else {
                        self.size(a)
                }
        }
        fn unite(&mut self, x: usize, y: usize) {
                let x = self.root(x);
                let y = self.root(y);
                if x != y {
                        if self.rank[x] < self.rank[y] {
                                self.size[y] += self.size(x);
                                self.parent[x] = y;
                        } else {
                                self.size[x] += self.size(y);
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
                mut uvw: [(usize, usize, usize); n - 1],
        }

        // Initialize
        // 0-basedに直す
        let mut uvw = uvw
                .into_iter()
                .map(|(u, v, w)| (u - 1, v - 1, w))
                .collect::<Vec<(usize, usize, usize)>>();
        // wの値に依ってソート
        uvw.sort_by_key(|a| a.2);

        // Solve
        let mut ans = 0;
        let mut uf = UnionFind::new(n);
        for (u, v, w) in uvw {
                ans += uf.size(u) * uf.size(v) * w;
                uf.unite(u, v);
        }

        // Output
        println!("{}", ans);
}
