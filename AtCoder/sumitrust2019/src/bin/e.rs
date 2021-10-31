use proconio::input;

const MOD: usize = 1000000007;

fn main() {
    // Input
    input! {
        n: usize,
        a_list: [usize; n],
    }
    
    // Solve
    let mut ans: usize = 1;
    let mut table = vec![0 as usize; 3];
    for a in &a_list {
        let mut index = 0;
        let cnt = {
            let mut cnt : usize = 0;
            for i in 0..3 {
                if table[i] == *a {
                    index = i;
                    cnt += 1;
                }
            }
            cnt
        };
        ans *= cnt;
        ans %= MOD;
        table[index] += 1;
    }

    // Output
    println!("{}", ans);
}
