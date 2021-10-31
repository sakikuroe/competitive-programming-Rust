use proconio::input;

fn main() {
        // Input
        input! {
                s: String,
        }

        // Solve
        let v = s.clone() + &s;

        let strings = {
                let mut res = (0..s.len())
                        .into_iter()
                        .map(|i| (&v[i..i + s.len()]).to_string())
                        .collect::<Vec<String>>();
                res.sort();

                res
        };

        // Output
        println!("{}", strings[0]);
        println!("{}", strings[s.len() - 1]);
}
