use proconio::input;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(number_of_nodes: usize) -> Self {
        let parent: Vec<usize> = (0..number_of_nodes).collect::<Vec<usize>>();
        let rank: Vec<usize> = vec![0; number_of_nodes];
        UnionFind { parent, rank }
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

    fn unite(&mut self, x: usize, y: usize) {
        let x = self.root(x);
        let y = self.root(y);
        if x != y {
            if self.rank[x] < self.rank[y] {
                self.parent[x] = y;
            } else {
                self.parent[y] = x;
                if self.rank[x] == self.rank[y] {
                    self.rank[x] += 1;
                }
            }
        }
    }

    fn number_of_connected_components(&mut self) -> usize {
        let n = self.parent.len();
        let mut v = (0..n).map(|x| self.root(x)).collect::<Vec<usize>>();
        v.sort();
        v.dedup();

        v.len()
    }

    fn connected(&mut self) -> bool {
        if self.number_of_connected_components() == 1 {
            true
        } else {
            false
        }
    }
}

fn solve(n: usize, ab_list: &Vec<(usize, usize)>) -> usize {
    let mut number_of_bridges: usize = 0;

    for i in 0..ab_list.len() {
        let mut union_find: UnionFind = {
            let mut union_find: UnionFind = UnionFind::new(n);
            for (j, &(a, b)) in ab_list.iter().enumerate() {
                if i == j {
                    continue;
                }
                union_find.unite(a - 1, b - 1);
            }

            union_find
        };
        if !union_find.connected() {
            number_of_bridges += 1;
        }
    }

    number_of_bridges
}

fn output(ans: usize) {
    println!("{}", ans);
}

fn main() {
    // Input
    input! {
        n: usize, m: usize,
        ab_list: [(usize, usize); m],
    }

    // Solve
    let ans: usize = solve(n, &ab_list);

    // Output
    output(ans);
}
