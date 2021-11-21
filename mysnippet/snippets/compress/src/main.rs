use cargo_snippet::snippet;

#[snippet(name = "compress")]
fn compress(v: &mut Vec<usize>, max_size: usize) -> Vec<usize> {
        let mut vs = vec![];
        for a in v.into_iter() {
                for direct in vec![-1, 0, 1] {
                        if 0 <= (*a) as isize + direct && (*a) as isize + direct <= max_size as isize {
                                vs.push(((*a) as isize + direct) as usize);
                        }
                }
        }

        vs.sort();
        vs.dedup();
        for a in v.into_iter() {
                *a = vs.binary_search(a).unwrap();
        }

        vs
}

fn main() {
        let mut v = vec![300, 300, 100, 600, 150];
        let vs = compress(&mut v, 600);

        println!("{:?}", &v);
        for (i, s) in vs.into_iter().enumerate() {
                println!("i: {}, s: {}", i, s);
        }
}
