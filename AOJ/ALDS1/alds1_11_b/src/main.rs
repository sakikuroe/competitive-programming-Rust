// use proconio::input;
#[derive(Clone, Copy, Debug)]
struct Edge {
    source: usize,
    destination: usize,
    weight: usize,
}

#[derive(Debug)]
struct Graph {
    edge_list: Vec<Vec<Edge>>,
    number_of_nodes: usize,
}
impl Graph {
    fn new(number_of_nodes: usize) -> Self {
        Graph {
            edge_list: vec![vec![]; number_of_nodes],
            number_of_nodes,
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

fn dfs(
    graph: &Graph,
    discover_finish_list: &mut Vec<(Option<usize>, Option<usize>)>,
    v: usize,
    time: &mut usize,
) {
    discover_finish_list[v].0 = Some(*time);
    for &Edge {
        source: _,
        destination,
        weight: _,
    } in &graph.edge_list[v]
    {
        if discover_finish_list[destination].0 == None {
            *time += 1;
            dfs(graph, discover_finish_list, destination, time);
        }
    }

    *time += 1;
    discover_finish_list[v].1 = Some(*time);
}

fn main() {
    // Input
    // input! {
    //     n: usize,
    // }
    // let mut gragh: Vec<Vec<usize>> = vec![vec![]; n];
    // for i in 0..n {
    //     input! {
    //         _: usize, k: usize,
    //     }
    //     for _j in 0..k {
    //         input! {
    //             m: usize,
    //         }
    //         gragh[i].push(m);
    //     }
    // }
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse::<usize>().unwrap();

    // Initialize
    // 0-basedでグラフをつくる
    let graph: Graph = {
        let mut graph = Graph::new(n);
        for i in 0..n {
            line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            let _: usize = iter.next().unwrap().parse::<usize>().unwrap();
            let k: usize = iter.next().unwrap().parse::<usize>().unwrap();

            let m_line: Vec<usize> = (0..k)
                .map(|_| iter.next().unwrap().parse::<usize>().unwrap())
                .collect();
            for j in 0..k {
                graph.add_edge(Edge {
                    source: (i),
                    destination: (m_line[j] - 1),
                    weight: 1,
                });
            }
        }
        graph
    };

    let mut discover_finish_list: Vec<(Option<usize>, Option<usize>)> = vec![(None, None); n];

    // Solve
    let mut t = 0;
    for i in 0..n {
        if discover_finish_list[i] == (None, None) {
            t += 1;
            dfs(&graph, &mut discover_finish_list, i, &mut t);
        }
    }

    // Output
    for (i, &(d, f)) in discover_finish_list.iter().enumerate() {
        println!("{} {} {}", i + 1, d.unwrap(), f.unwrap());
    }
}
