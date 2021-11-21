use proconio::input;
struct UnionFind {
        parent: Vec<usize>,
        size: Vec<usize>,
}
impl UnionFind {
        fn new(number_of_nodes: usize) -> Self {
                let parent = (0..number_of_nodes).collect::<Vec<usize>>();
                let size = vec![1; number_of_nodes];
                UnionFind {
                        parent,
                        size,
                }
        }
        fn root(&mut self, x: usize) -> usize {
                if self.parent[x] == x {
                        x
                } else {
                        self.parent[x] = self.root(self.parent[x]);
                        self.parent[x]
                }
        }
        fn size(&mut self, x: usize) -> usize {
                let a = self.root(x);
                if self.root(a) == a {
                        self.size[a]
                } else {
                        self.size(a)
                }
        }
        fn unite(&mut self, x: usize, y: usize) {
                let x = self.root(x);
                let y = self.root(y);
                if x != y {
                        self.size[x] += self.size(y);
                        self.parent[y] = x;
                }
        }
}
fn main() {
        // Input
        input! {
            q: usize,
            tx: [(usize, usize); q],
        }

        // Solve
        let n = 1048576;

        let mut ans = vec![];
        let mut a = vec![-1; n];
        let mut uf = UnionFind::new(n);
        for &(t, x) in &tx {
                if t == 1 {
                        let mut h = x % n;
                        while a[h % n] != -1 {
                                if h == uf.root(h) {
                                        uf.unite((h + 1) % n, h);
                                        h += 1;
                                } else {
                                        h = uf.root(h);
                                }
                                h %= n;
                        }
                        a[h] = x as isize;
                } else {
                        ans.push(a[x % n]);
                }
        }

        // Output
        for a in ans {
                println!("{}", a);
        }
}
