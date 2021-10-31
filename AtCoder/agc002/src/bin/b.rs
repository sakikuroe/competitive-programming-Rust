use proconio::input;
use proconio::marker::Usize1;

fn main() {
        // Input
        input! {
                n: usize, m: usize,
                xy_list: [(Usize1, Usize1); m],
        }
    
        // Initialize
        let mut graph = vec![(1, false); n];
        graph[0] = (1,true);
        
        // Solve
        for (x, y) in xy_list {
                if graph[x].1 {
                        graph[y].1 = true;
                }
                graph[x].0 -= 1;
                graph[y].0 += 1;
                if graph[x].0 == 0 {
                        graph[x].1 = false;
                }
        }

        let mut ans = 0;
        for i in 0..n {
                if graph[i].0 > 0 && graph[i].1 {
                        ans += 1;
                }
        }
        
        // Output
        println!("{}", ans);
}
