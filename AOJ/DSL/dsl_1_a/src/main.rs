use std::io;
// use proconio::input;

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

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
}

fn solve(union_find: &mut UnionFind, q_list: &Vec<(usize, usize, usize)>) -> Vec<usize> {
    let mut res: Vec<usize> = vec![];
    for &(f, x, y) in q_list {
        if f == 0 {
            union_find.unite(x, y);
        } else {
            res.push(if union_find.same(x, y) { 1 } else { 0 });
        }
    }
    res
}

fn output(ans_list: &Vec<usize>) {
    for &x in ans_list {
        println!("{}", x);
    }
}

fn main() {
    // Input
        // input! {
        //     n: usize, q: usize,
        //     q_list: [(usize, usize, usize); q],
        // }
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let word_list: Vec<&str> = line.split_whitespace().collect();
    let n: usize = word_list[0].parse::<usize>().unwrap();
    let q: usize = word_list[1].parse::<usize>().unwrap();

    let mut q_list: Vec<(usize, usize, usize)> = Vec::new();
    for _ in 0..q {
        line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let word_list: Vec<usize> = {
            let word_list: Vec<&str> = line.split_whitespace().collect();
            word_list.into_iter().map(|s| s.parse::<usize>().unwrap()).collect()
        };
        q_list.push((word_list[0], word_list[1], word_list[2]));
    }

    // Initialize
    let mut union_find: UnionFind = UnionFind::new(n);

    // Solve
    let ans_list: Vec<usize> = solve(&mut union_find, &q_list);

    // Output
    output(&ans_list);
}
