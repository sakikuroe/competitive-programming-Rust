use proconio::input;
#[derive(Clone, Debug)]
struct Node {
        value: usize,
        lazy: usize,
}
impl Node {
        fn new() -> Node {
                Node {
                        value: 0,
                        lazy: 0,
                }
        }
}
#[derive(Debug)]
struct LazySegmentTree {
        size: usize,
        nodes: Vec<Node>,
}
impl LazySegmentTree {
        fn new(n: usize) -> Self {
                let size = 2u32.pow(((n - 1) as f64).log2() as u32 + 1) as usize;
                LazySegmentTree {
                        size,
                        nodes: vec![Node::new(); 2 * size - 1],
                }
        }
        fn eval(&mut self, idx: usize) {
                if idx == 1 {
                        println!("lazy {:?}",self.nodes[idx].lazy);
                }
                if self.nodes[idx].lazy != 0 {
                        self.nodes[idx].value += self.nodes[idx].lazy;

                        if idx < self.size - 1 {
                                self.nodes[2 * idx + 1].lazy += self.nodes[idx].lazy;
                                self.nodes[2 * idx + 2].lazy += self.nodes[idx].lazy;
                        }
                        self.nodes[idx].lazy = 0;
                }
        }
        fn get_value(&mut self, idx: usize) -> usize {
                self.eval(idx);
                self.nodes[idx].value
        }
        fn update_sub(&mut self, a: usize, b: usize, idx: usize, l: usize, r: usize) {
                self.eval(idx);
                println!("{}", idx);
                if a <= l && r <= b {
                        self.nodes[idx].lazy += 1;
                } else if a < r && l < b {
                        self.update_sub(a, b, 2 * idx + 1, l, (l + r) / 2);
                        self.update_sub(a, b, 2 * idx + 2, (l + r) / 2, r);
                        self.nodes[idx].value =
                                self.get_value(2 * idx + 1)+ self.get_value(2 * idx + 2);
                        }
        }
        fn update(&mut self, l: usize, r: usize, ) {
                self.update_sub(l, r, 0,  0, self.size);
        }
        fn find_sub(&mut self, a: usize, b: usize, idx: usize, l: usize, r: usize) -> usize {
                self.eval(idx);
                if a <= l && r <= b {
                        return self.nodes[idx].value;
                } else if a < r && l < b {
                        return 
                                self.find_sub(a, b, 2 * idx + 1, l, (l + r) / 2)+
                                self.find_sub(a, b, 2 * idx + 2, (l + r) / 2, r);
                } else {
                        return 0;
                }
        }
        fn find(&mut self, l: usize, r: usize) -> usize {
                self.find_sub(l, r, 0, 0, self.size)
        }
}
fn main() {
        // Input
        input! {
                n: usize, q_size: usize,
                qlr_list: [(usize, usize, usize); q_size],
        }
    
        // Initialize
        let mut seg_tree = LazySegmentTree::new(n);

        // Solve
        let mut l1 = 0;
        let mut l2 = 1;
        let mut l3 = 45;
        for i in 1..5 {
                for _ in 0..l2 {
                        for _ in 0..l3 {
                                print!(" ");
                        }
                        print!("[{}: {}, {}]",l1 ,seg_tree.nodes[l1].value, seg_tree.nodes[l1].lazy);
                        l1 += 1;
                }
                println!();
                l3 /=2;
                l2 *= 2;
        }
        println!();
        let ans_list = {
                let mut res :Vec<usize> = vec![];

                for (q,l,r) in qlr_list {
                        if q == 1 {
                                seg_tree.update(l, r);
                        } else {
                                res.push(
                                        seg_tree.find(l, r)
                                );
                        }
                        let mut l1 = 0;
                        let mut l2 = 1;
                        let mut l3 = 45;
                        println!("[ {} {} {} ]",q,l,r);
                        for i in 1..5 {
                                for _ in 0..l2 {
                                        for _ in 0..l3 {
                                                print!(" ");
                                        }
                                        print!("[{}: {}, {}]",l1 ,seg_tree.nodes[l1].value, seg_tree.nodes[l1].lazy);
                                        l1 += 1;
                                }
                                println!();
                                l3 /=2;
                                l2 *= 2;
                        }
                        println!();

                }

                res
        };
        
        // Output
        for ans in ans_list {
                println!("{}", ans);
        }
}
