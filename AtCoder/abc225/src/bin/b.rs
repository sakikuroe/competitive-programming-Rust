use proconio::input;

#[derive(Debug)]
struct Graph {
        edges: Vec<Vec<usize>>,
        size: usize,
}
impl Graph {
        fn new(size: usize) -> Self {
                Graph {
                        edges: vec![vec![]; size],
                        size,
                }
        }
        fn add_edge(&mut self, src: usize, dst: usize) {
                self.edges[src].push(dst);
        }
}

fn main() {
        // Input
        input! {
            n: usize,
            ab_list: [(usize, usize); n-1],
        }

        // Initialize
        let mut g = Graph::new(n);
        for (a, b) in ab_list.clone() {
                g.add_edge(a - 1, b - 1);
                g.add_edge(b - 1, a - 1);
        }

        // Solve
        let mut ans = false;
        for i in 0..n {
                if g.edges[i].len() == n - 1 {
                        ans = true;
                }
        }

        // Output
        if ans {
                println!("Yes");
        } else {
                println!("No");
        }
}
