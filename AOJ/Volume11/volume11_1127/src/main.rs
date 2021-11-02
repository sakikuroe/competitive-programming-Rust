use std::{cmp::Ordering, io};
// use proconio::input;

#[derive(Copy, Clone, PartialEq)]
struct Edge {
    source: usize,
    destination: usize,
    weight: f64,
}

impl Eq for Edge {}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
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

fn kruskal(edge_list: &Vec<Edge>, v_size: usize, number_of_connected_components: usize) -> f64 {
    let mut path_weight_sum = 0.0;

    let mut cnt = 0;

    let mut union_find = UnionFind::new(v_size);
    for &Edge {
        source,
        destination,
        weight,
    } in edge_list
    {
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

type Point3 = (f64, f64, f64);

fn destination(p1: Point3, p2: Point3) -> f64 {
    let (x1, y1, z1) = p1;
    let (x2, y2, z2) = p2;
    ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2) + (z1 - z2) * (z1 - z2)).sqrt()
}

fn solve(xyzr_list: &Vec<(f64, f64, f64, f64)>) -> f64 {
    let edge_list: Vec<Edge> = {
        let mut edge_list: Vec<Edge> = vec![];
        for i in 0..xyzr_list.len() {
            for j in i + 1..xyzr_list.len() {
                let p1 = (xyzr_list[i].0, xyzr_list[i].1, xyzr_list[i].2);
                let r1 = xyzr_list[i].3;
                let p2 = (xyzr_list[j].0, xyzr_list[j].1, xyzr_list[j].2);
                let r2 = xyzr_list[j].3;

                edge_list.push(Edge {
                    source: i,
                    destination: j,
                    weight: {
                        if destination(p1, p2) < r1 + r2 {
                            0.0
                        } else {
                            destination(p1, p2) - r1 - r2
                        }
                    },
                });
            }
        }
        edge_list.sort();
        edge_list
    };

    kruskal(&edge_list, xyzr_list.len(), xyzr_list.len())
}

fn output(ans: f64) {
    println!("{:.3}", ans);
}

fn main() {
    loop {
        // Input
            // input! {
            //     n: usize
            // };
            // if n == 0 {
            //     break;
            // }
            // input! {
            //     xyzr_list: [(f64, f64, f64, f64); n],
            // }
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let word_list: Vec<&str> = line.split_whitespace().collect();
        let n: usize = word_list[0].parse::<usize>().unwrap();
        if n == 0 {
            break;
        }
        let mut xyzr_list: Vec<(f64, f64, f64, f64)> = Vec::new();
        for _ in 0..n {
            line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let word_list: Vec<f64> = {
                let word_list: Vec<&str> = line.split_whitespace().collect();
                word_list.into_iter().map(|s| s.parse::<f64>().unwrap()).collect()
            };
            xyzr_list.push((word_list[0], word_list[1], word_list[2], word_list[3]));
        }

        // Solve
        let ans: f64 = solve(&xyzr_list);
        
        // Output
        output(ans);
    }
}
