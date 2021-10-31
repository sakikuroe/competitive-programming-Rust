use proconio::input;

//座標圧縮
fn compress(v: &mut Vec<usize>) -> Vec<usize> {
        // vs: 目盛り
        let mut vs: Vec<usize> = vec![];
        for a in v.into_iter() {
                vs.push(*a);
        }
        vs.sort();
        vs.dedup();

        for a in v.into_iter() {
                *a = vs.binary_search(a).unwrap();
        }

        vs
}

fn main() {
        // Input
        input! {
                n: usize,
                mut a_list: [usize; n],
        }

        // Solve
        compress(&mut a_list);

        // Output
        for a in &a_list {
                println!("{}", a);
        }
}
