use std::cmp::Ordering;
use proconio::input;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge {
    source: usize,
    destination: usize,
    weight: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(number_of_nodes: usize) -> Self {
        let parent: Vec<usize> = (0..number_of_nodes).collect::<Vec<usize>>();
        let rank: Vec<usize> = vec![0; number_of_nodes];
        UnionFind {
            parent,
            rank,
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

fn kruskal(edge_list: &Vec<Edge>, v_size: usize, number_of_connected_components: usize) -> usize {
    let mut path_weight_sum: usize = 0;

    let mut cnt = 0;

    let mut union_find = UnionFind::new(v_size);
    for &Edge{ source, destination, weight, } in edge_list {
        if cnt == number_of_connected_components {
            break;
        }
        if !union_find.same(source, destination) {
            union_find.unite(source, destination);
            path_weight_sum += weight;
            cnt += 1;
        }
    }

    path_weight_sum
}

fn solve(n: usize, k: usize, abc_list: &Vec<(usize, usize, usize)>) -> usize {
    let edge_list: Vec<Edge> = {
        let mut edge_list: Vec<Edge> = vec![];
        for &(s, t, w) in abc_list {
            edge_list.push(Edge{ source: s, destination: t, weight: w, });
        }
        edge_list.sort();
        edge_list
    };

    kruskal(&edge_list, n, n - k)
}

fn output(ans: usize) {
    println!("{}", ans);
}

fn main() {
    // Input
    input! {
        n: usize, m: usize, k: usize,
        abc_list: [(usize, usize, usize); m],
    }

    // Initialize
    let abc_list = abc_list.iter().map(|&(a, b, c)| (a - 1, b - 1, c)).collect::<Vec<(usize, usize, usize)>>();

    // Solve
    let ans: usize = solve(n, k , &abc_list);

    // Output
    output(ans);
}