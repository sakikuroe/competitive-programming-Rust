use cargo_snippet::snippet;

#[snippet(name = "compress2")]
fn compress(v1: &mut Vec<usize>, v2: &mut Vec<usize>, size: usize) -> Vec<usize> {
        let mut vs: Vec<usize> = vec![];
        for a in v1.into_iter() {
                for direct in vec![-1, 0, 1] {
                        if 0 <= (*a) as isize + direct && (*a) as isize + direct <= size as isize {
                                vs.push(((*a) as isize + direct) as usize);
                        }
                }
        }
        for a in v2.into_iter() {
                for direct in vec![-1, 0, 1] {
                        if 0 <= (*a) as isize + direct && (*a) as isize + direct <= size as isize {
                                vs.push(((*a) as isize + direct) as usize);
                        }
                }
        }
        vs.sort();
        vs.dedup();
        for a in v1.into_iter() {
                *a = vs.binary_search(a).unwrap();
        }
        for a in v2.into_iter() {
                *a = vs.binary_search(a).unwrap();
        }
        vs
}


fn main() {
        println!("Hello, world!");
}
