#![allow(non_snake_case)]
use std::{usize::MAX, collections::BinaryHeap};
use proconio::input;

#[derive(Clone)]
struct Edge {
    source: usize,
    destination: usize,
    dist: usize,
}

type Graph = Vec<Vec<Edge>>;
const INF: usize = MAX;

// (グラフ, 開始位置のindex) -> 各ノードへ到達するのに必要な最小コストのリスト
fn dijkstra (graph: &Graph, start_position: usize) -> Vec<usize> {
    // 各ノードにおけるコストの更新表
    let mut cost_list: Vec<usize> = {
        let mut cost_list: Vec<usize> = vec![INF; graph.len()];
        cost_list[start_position] = 0;
        cost_list
    };
    
    // 探索する必要のあるpositionの優先度付きキュー
    let mut que: BinaryHeap<usize> = {
        let mut que: BinaryHeap<usize> = BinaryHeap::new(); 
        que.push(start_position);
        que
    };

    // queの先頭の要素を順に取り出し、cost_listを更新していく
    while let Some(current_position) = que.pop() {
        for &Edge {source: _, destination, dist} in &graph[current_position] {
            if cost_list[destination] > cost_list[current_position] + dist {
                cost_list[destination] = cost_list[current_position] + dist;
                que.push(destination);
            }
        }
    }

    cost_list
}

fn solve (graph: &Graph, start_position: usize) -> Vec<usize> {
    //　Dijkstra法を使う
    dijkstra(graph, start_position)
}

fn output (cost_list: &Vec<usize>) {
    for &cost in cost_list {
        if cost == INF {
            println!("INF");
        } else {
            println!{"{}", cost};          
        }
    }
}

fn main() {
    // 入力
    input! {
        v_size: usize,
        e_size: usize,
        r: usize,
        std_list: [(usize, usize, usize); e_size],
    }

    // グラフと開始位置の初期化
    let graph: Graph = {
        let mut graph = vec![vec![]; v_size];
        for std in std_list {
            let (start, to, dist) = std;
            graph[start].push(Edge {source: start, destination: to, dist});
        }
        graph
    };
    let start_position: usize = r;

    // 問題の解決
    let cost_list: Vec<usize> = solve(&graph, start_position);
    
    // 出力
    output(&cost_list);
}