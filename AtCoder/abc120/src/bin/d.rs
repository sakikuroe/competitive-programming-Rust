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
        UnionFind { parent, rank, number_of_connected_components }
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
                self.number_of_connected_components[y] += self.connected_components(x);
                self.parent[x] = y;
            } else {
                self.number_of_connected_components[x] += self.connected_components(y);
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

fn solve(n: usize, ab_list: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut union_find: UnionFind = UnionFind::new(n);

    let mut res: Vec<usize> = vec![];

    let mut confortable = 0;
    res.push(n * (n - 1) / 2 - confortable);

    for &(a, b) in ab_list {
        if !union_find.same(a, b) {
            let s = union_find.connected_components(a);
            let t = union_find.connected_components(b);
            confortable += s * t;
            union_find.unite(a, b);
        }
        res.push(n * (n - 1) / 2 - confortable);
    }

    res.reverse();
    res
}

fn output(ans_list: &Vec<usize>) {
    for &ans in ans_list {
        println!("{}", ans);
    }
}

fn main() {
    // Input
    input! {
        n: usize, m: usize,
        mut ab_list: [(usize, usize); m],
    }

    // Initialize
    ab_list.reverse();
    ab_list.pop();
    ab_list = ab_list.iter().map(|(a, b)| (a - 1, b - 1)).collect::<Vec<(usize, usize)>>();

    // Solve
    let ans_list: Vec<usize> = solve(n, &ab_list);

    // Output
    output(&ans_list);
}
