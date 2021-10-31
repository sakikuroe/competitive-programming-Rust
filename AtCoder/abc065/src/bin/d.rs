use std::cmp::Ordering;
use proconio::input;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

fn kruskal(edge_list: &Vec<Edge>, v_size: usize) -> usize {
    let mut path_weight_sum: usize = 0;

    let mut union_find = UnionFind::new(v_size);
    for &Edge{ source, destination, weight, } in edge_list {
        if !union_find.same(source, destination) {
            union_find.unite(source, destination);
            path_weight_sum += weight;
        }
    }

    path_weight_sum
}

fn solve(n: usize, xy_list: &mut Vec<(usize, usize)>) -> usize {
    let mut edge_list: Vec<Edge> = vec![];

    let mut nxy_list = (0..n).zip(xy_list.iter()).collect::<Vec<(usize, &(usize, usize))>>();
    nxy_list.sort_by(|&(_, (x1, _)), &(_, (x2, _))| x1.partial_cmp(x2).unwrap());
    for i in 0..n - 1 {
        let (n1, &(x1, y1)) = nxy_list[i];
        let (n2, &(x2, y2)) = nxy_list[i + 1];
        let weight = (x1 as isize - x2 as isize).abs().min((y1 as isize - y2 as isize).abs()) as usize;
        edge_list.push(Edge { source:n1, destination: n2, weight });
    }
    
    nxy_list.sort_by(|&(_, (_, y1)), &(_, (_, y2))| y1.partial_cmp(y2).unwrap());
    for i in 0..n - 1 {
        let (n1, &(x1, y1)) = nxy_list[i];
        let (n2, &(x2, y2)) = nxy_list[i + 1];
        let weight = (x1 as isize - x2 as isize).abs().min((y1 as isize - y2 as isize).abs()) as usize;
        edge_list.push(Edge { source:n1, destination: n2, weight });
    }
    
    edge_list.sort();
    kruskal(&edge_list, n)
}

fn output(ans: usize) {
    println!("{}", ans);
}

fn main() {
    // Input
    input! {
        n: usize,
        mut xy_list: [(usize, usize); n],
    }

    // Solve
    let ans: usize = solve(n, &mut xy_list);

    // Output
    output(ans);
}