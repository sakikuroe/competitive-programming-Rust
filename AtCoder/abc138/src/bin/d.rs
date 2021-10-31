use proconio::input;
use std::usize::MAX;
const INF: usize = MAX;

#[derive(Clone, Copy, Debug)]
struct Edge {
    source: usize,
    destination: usize,
    weight: usize,
}
struct Graph {
    edge_list: Vec<Vec<Edge>>,
}
impl Graph {
    fn new(number_of_nodes: usize) -> Self {
        Graph {
            edge_list: vec![vec![]; number_of_nodes],
        }
    }
    fn add_edge(
        &mut self,
        Edge {
            source,
            destination,
            weight,
        }: Edge,
    ) {
        self.edge_list[source].push(Edge {
            source,
            destination,
            weight,
        });
    }
}

fn dfs(now: usize, prev: usize, cnt_list: &mut Vec<usize>, graph: &Graph) {
    for &Edge {
        source: _,
        destination,
        weight: _,
    } in &graph.edge_list[now]
    {
        if destination == prev {
            continue;
        }

        cnt_list[destination] += cnt_list[now];
        dfs(destination, now, cnt_list, graph);
    }
}

fn main() {
    // Input
    input! {
        n: usize, q: usize,
        ab_list: [(usize, usize); n - 1],
        px_list: [(usize, usize); q],
    }

    // Initialize
    // 0-basedに直す
    let ab_list = ab_list
        .into_iter()
        .map(|(a, b)| (a - 1, b - 1))
        .collect::<Vec<(usize, usize)>>();
    let px_list = px_list
        .into_iter()
        .map(|(p, x)| (p - 1, x))
        .collect::<Vec<(usize, usize)>>();

    let mut cnt_list = {
        let mut cnt_list = vec![0; n];
        for &(p, x) in &px_list {
            cnt_list[p] += x;
        }
        cnt_list
    };

    let graph: Graph = {
        let mut graph = Graph::new(n);
        for &(a, b) in &ab_list {
            graph.add_edge(Edge {
                source: a,
                destination: b,
                weight: 1,
            });
            graph.add_edge(Edge {
                source: b,
                destination: a,
                weight: 1,
            });
        }
        graph
    };

    // Solve
    let ans_list = {
        dfs(0, INF, &mut cnt_list, &graph);
        cnt_list
    };

    // Output
    for &ans in &ans_list {
        print!("{} ", ans);
    }
}
