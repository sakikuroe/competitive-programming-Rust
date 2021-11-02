use cargo_snippet::snippet;
use std::cmp;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::ops;
use std::usize::MAX;
const MOD: usize = 1_000_000_007;

#[snippet(name = "inf", prefix = "use std::usize::MAX;")]
const INF: usize = MAX / 3;

#[snippet(name = "node", prefix = "use std::cmp::Ordering;")]
#[derive(Eq, PartialOrd, PartialEq)]
struct Node {
        vertex: usize,
        priory: usize,
}
#[snippet(name = "node")]
impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
                self.priory.cmp(&other.priory).reverse()
        }
}

#[snippet(name = "edge")]
#[derive(Clone, Copy, Debug)]
struct Edge {
        src: usize,
        dst: usize,
        weight: usize,
}

#[snippet(name = "graph", include = "edge", prefix = "use std::collections::BinaryHeap;")]
#[derive(Debug)]
struct Graph {
        edges: Vec<Vec<Edge>>,
        size: usize,
}

#[snippet(name = "graph")]
impl Graph {
        fn new(size: usize) -> Self {
                Graph {
                        edges: vec![vec![]; size],
                        size,
                }
        }
        fn add_edge(&mut self, e: Edge) {
                self.edges[e.src].push(Edge {
                        src: e.src,
                        dst: e.dst,
                        weight: e.weight,
                });
        }
}
#[snippet(name = "dijkstra", include = "inf, graph, node")]
impl Graph {
        fn dijkstra(&self, start: usize) -> Vec<usize> {
                let mut dist = vec![INF; self.size];
                dist[start] = 0;

                let mut que = BinaryHeap::new();
                que.push(Node {
                        vertex: start,
                        priory: 0,
                });

                while let Some(node) = que.pop() {
                        if node.priory < dist[node.vertex] {
                                continue;
                        }
                        for &e in &self.edges[node.vertex] {
                                if dist[e.dst] > dist[e.src] + e.weight {
                                        dist[e.dst] = dist[e.src] + e.weight;
                                        que.push(Node {
                                                vertex: e.dst,
                                                priory: dist[e.dst],
                                        });
                                }
                        }
                }
                dist
        }
}
#[snippet(name = "vertex")]
#[derive(Eq, Debug)]
struct Vertex {
        priory: usize,
}
impl Ord for Vertex {
        fn cmp(&self, other: &Self) -> Ordering {
                self.priory.cmp(&other.priory).reverse()
        }
}
impl PartialOrd for Vertex {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
        }
}
impl PartialEq for Vertex {
        fn eq(&self, other: &Self) -> bool {
                self == other
        }
}

#[snippet(name = "topological_sort")]
impl Graph {
        fn topological_sort(&self) -> Result<Vec<usize>, &str> {
                let mut indegree = {
                        let mut res = vec![0; self.size];
                        for edge_list in &self.edges {
                                for &e in edge_list {
                                        res[e.dst] += 1;
                                }
                        }
                        res
                };
                let res = {
                        let mut sorted: Vec<usize> = vec![];
                        let mut que = {
                                let mut res = BinaryHeap::new();
                                for i in 0..self.size {
                                        if indegree[i] == 0 {
                                                res.push(Vertex {
                                                        priory: i,
                                                });
                                        }
                                }
                                res
                        };
                        while let Some(Vertex {
                                priory,
                        }) = que.pop()
                        {
                                for &e in &self.edges[priory] {
                                        indegree[e.dst] -= 1;
                                        if indegree[e.dst] == 0 {
                                                que.push(Vertex {
                                                        priory: e.dst,
                                                })
                                        }
                                }
                                sorted.push(priory);
                        }

                        if sorted.len() == self.size {
                                Ok(sorted)
                        } else {
                                Err("-1")
                        }
                };
                res
        }
}
#[snippet(name = "node2", prefix = "use std::cmp::{Ordering, Reverse};")]
#[derive(Eq)]
struct Node2 {
        vertex: (usize, usize),
        priory: Reverse<usize>,
}
#[snippet(name = "node2")]
impl Ord for Node2 {
        fn cmp(&self, other: &Self) -> Ordering {
                self.priory.cmp(&other.priory)
        }
}
#[snippet(name = "node2")]
impl PartialOrd for Node2 {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
        }
}
#[snippet(name = "node2")]
impl PartialEq for Node2 {
        fn eq(&self, other: &Self) -> bool {
                self.priory == other.priory
        }
}

#[snippet(name = "edge2")]
#[derive(Clone, Copy, Debug)]
struct Edge2 {
        src: (usize, usize),
        dst: (usize, usize),
        weight: usize,
}

#[snippet(
        name = "graph2",
        include = "inf, edge2, node2",
        prefix = "use std::collections::BinaryHeap;"
)]
#[snippet(name = "maze")]
#[derive(Debug)]
struct Graph2 {
        edges: Vec<Vec<Vec<Edge2>>>,
        h: usize,
        w: usize,
}

#[snippet(name = "graph2")]
#[snippet(name = "maze")]
impl Graph2 {
        fn new_maze(h: usize, w: usize, maze: &Vec<Vec<char>>) -> Self {
                let mut graph = Graph2 {
                        edges: vec![vec![vec![]; w]; h],
                        h,
                        w,
                };
                let v_list = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
                for i in 0..h {
                        for j in 0..w {
                                if maze[i][j] == '#' {
                                        continue;
                                }
                                for &v in &v_list {
                                        let (ny, nx) = (i as isize + v.0, j as isize + v.1);
                                        if 0 <= ny
                                                && ny < h as isize
                                                && 0 <= nx
                                                && nx < w as isize
                                                && maze[ny as usize][nx as usize] != '#'
                                        {
                                                graph.add_edge(Edge2 {
                                                        src: (i, j),
                                                        dst: (ny as usize, nx as usize),
                                                        weight: 1,
                                                })
                                        }
                                }
                        }
                }
                graph
        }

        fn add_edge(
                &mut self,
                Edge2 {
                        src,
                        dst,
                        weight,
                }: Edge2,
        ) {
                self.edges[src.0][src.1].push(Edge2 {
                        src,
                        dst,
                        weight,
                });
        }

        fn dijkstra(self, start: (usize, usize)) -> Vec<Vec<usize>> {
                let mut dist = {
                        let mut dist = vec![vec![INF; self.w]; self.h];
                        dist[start.0][start.1] = 0;
                        dist
                };
                let mut que: BinaryHeap<Node2> = {
                        let mut que = BinaryHeap::new();
                        que.push(Node2 {
                                vertex: (start.0, start.1),
                                priory: Reverse(0),
                        });
                        que
                };
                let mut visited = vec![vec![false; self.w]; self.h];
                while let Some(Node2 {
                        vertex,
                        priory: _,
                }) = que.pop()
                {
                        if visited[vertex.0][vertex.1] {
                                continue;
                        }
                        for &Edge2 {
                                src,
                                dst,
                                weight,
                        } in &self.edges[vertex.0][vertex.1]
                        {
                                if dist[dst.0][dst.1] > dist[src.0][src.1] + weight {
                                        dist[dst.0][dst.1] = dist[src.0][src.1] + weight;
                                        que.push(Node2 {
                                                vertex: dst,
                                                priory: Reverse(dist[dst.0][dst.1]),
                                        });
                                }
                        }
                        visited[vertex.0][vertex.1] = true;
                }
                dist
        }
}

#[snippet(name = warshall_floyd, prefix = "use std::cmp;")]
fn warshall_floyd(mat: &mut Vec<Vec<usize>>) {
        for k in 0..mat.len() {
                for i in 0..mat.len() {
                        for j in 0..mat.len() {
                                mat[i][j] = cmp::min(mat[i][j], mat[i][k] + mat[k][j]);
                        }
                }
        }
}

#[snippet(name = "unionfind")]
struct UnionFind {
        parent: Vec<usize>,
        rank: Vec<usize>,
        size: Vec<usize>,
        n: usize,
}
#[snippet(name = "unionfind")]
impl UnionFind {
        fn new(number_of_nodes: usize) -> Self {
                let parent = (0..number_of_nodes).collect::<Vec<usize>>();
                let rank = vec![0; number_of_nodes];
                let size = vec![1; number_of_nodes];
                UnionFind {
                        parent,
                        rank,
                        size,
                        n: number_of_nodes,
                }
        }
        fn is_root(&mut self, x: usize) -> bool {
                self.parent[x] == x
        }
        fn root(&mut self, x: usize) -> usize {
                if self.is_root(x) {
                        x
                } else {
                        self.parent[x] = self.root(self.parent[x]);
                        self.parent[x]
                }
        }
        fn size(&mut self, x: usize) -> usize {
                let a = self.root(x);
                if self.is_root(a) {
                        self.size[a]
                } else {
                        self.size(a)
                }
        }
        fn unite(&mut self, x: usize, y: usize) {
                let x = self.root(x);
                let y = self.root(y);
                if x != y {
                        if self.rank[x] < self.rank[y] {
                                self.size[y] += self.size(x);
                                self.parent[x] = y;
                        } else {
                                self.size[x] += self.size(y);
                                self.parent[y] = x;
                                if self.rank[x] == self.rank[y] {
                                        self.rank[x] += 1;
                                }
                        }
                }
        }
        fn same(&mut self, x: usize, y: usize) -> bool {
                self.root(x) == self.root(y)
        }
        fn clear(&mut self) {
                self.parent = (0..self.n).collect::<Vec<usize>>();
                self.rank = vec![0; self.n];
                let size = vec![1; self.n];
        }
}

#[snippet(name = "kaibun")]
fn is_palindrome(s: &Vec<char>) -> bool {
        let length = s.len();
        for i in 0..length {
                if !(s[i] == s[(length - 1) - i]) {
                        return false;
                }
        }

        true
}

#[snippet(name = "mint", prefix = "use std::ops;", prefix = "const MOD: usize = 1_000_000_007;")]
#[derive(Clone, Debug)]
struct Mint {
        value: usize,
}
#[snippet(name = "mint")]
impl Mint {
        fn new(value: usize) -> Mint {
                Mint {
                        value: value % MOD,
                }
        }
        fn value(&self) -> usize {
                self.value
        }
        fn mod_pow(&self, n: usize) -> Mint {
                if n == 0 {
                        Mint::new(1)
                } else if n % 2 == 0 {
                        Mint::new((self.clone() * self.clone()).mod_pow(n / 2).value % MOD)
                } else {
                        Mint::new((self.clone() * self.clone().mod_pow(n - 1)).value % MOD)
                }
        }
        fn mod_inverse(&self) -> Mint {
                self.mod_pow(MOD - 2)
        }
        fn mod_factorial(&self) -> Mint {
                if self.value == 0 {
                        Mint::new(1)
                } else {
                        let mut res = Mint::new(1);
                        for i in 1..=self.value {
                                res *= Mint::new(i);
                        }
                        res
                }
        }
}
#[snippet(name = "mint")]
impl ops::Add for Mint {
        type Output = Mint;
        fn add(self, other: Self) -> Self {
                let mut value = self.value + other.value;
                if value >= MOD {
                        value -= MOD;
                }
                Mint {
                        value,
                }
        }
}
#[snippet(name = "mint")]
impl ops::Sub for Mint {
        type Output = Mint;
        fn sub(self, other: Self) -> Self {
                if self.value >= other.value {
                        Mint {
                                value: self.value - other.value,
                        }
                } else {
                        Mint {
                                value: (self.value + MOD) - other.value,
                        }
                }
        }
}
#[snippet(name = "mint")]
impl ops::Mul for Mint {
        type Output = Mint;
        fn mul(self, other: Self) -> Self {
                Mint {
                        value: self.value * other.value % MOD,
                }
        }
}
#[snippet(name = "mint")]
impl ops::Div for Mint {
        type Output = Mint;
        fn div(self, other: Self) -> Self {
                Mint {
                        value: (self * other.mod_inverse()).value % MOD,
                }
        }
}
#[snippet(name = "mint")]
impl ops::AddAssign for Mint {
        fn add_assign(&mut self, other: Self) {
                self.value = (self.clone() + other).value;
        }
}
#[snippet(name = "mint")]
impl ops::SubAssign for Mint {
        fn sub_assign(&mut self, other: Self) {
                self.value = (self.clone() - other).value;
        }
}
#[snippet(name = "mint")]
impl ops::MulAssign for Mint {
        fn mul_assign(&mut self, other: Self) {
                self.value = (self.clone() * other).value;
        }
}
#[snippet(name = "mint")]
impl ops::DivAssign for Mint {
        fn div_assign(&mut self, other: Self) {
                self.value = (self.clone() / other).value;
        }
}
#[snippet(name = "mint")]
fn mod_permutation(n: Mint, r: Mint) -> Mint {
        let mut res = Mint::new(1);
        for i in 0..r.value {
                res *= Mint::new(n.value - i);
        }
        res
}
#[snippet(name = "mint")]
fn mod_combination(n: Mint, r: Mint) -> Mint {
        r.mod_factorial().mod_inverse() * mod_permutation(n, r)
}

#[snippet(name = "compress")]
fn compress(v: &mut Vec<usize>, max_size: usize) -> Vec<usize> {
        let mut vs = vec![];
        for a in v.into_iter() {
                for direct in vec![-1, 0, 1] {
                        if 0 <= (*a) as isize + direct
                                && (*a) as isize + direct <= max_size as isize
                        {
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

#[snippet(name = "compress2")]
fn compress2(v1: &mut Vec<usize>, v2: &mut Vec<usize>, max_size: usize) -> Vec<usize> {
        let mut vs: Vec<usize> = vec![];
        for a in v1.into_iter() {
                for direct in vec![-1, 0, 1] {
                        if 0 <= (*a) as isize + direct
                                && (*a) as isize + direct <= max_size as isize
                        {
                                vs.push(((*a) as isize + direct) as usize);
                        }
                }
        }
        for a in v2.into_iter() {
                for direct in vec![-1, 0, 1] {
                        if 0 <= (*a) as isize + direct
                                && (*a) as isize + direct <= max_size as isize
                        {
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

#[snippet(name = "prime")]
fn is_prime(n: usize) -> bool {
        if n == 0 || n == 1 {
                false
        } else {
                for i in 2..=(n as f64).sqrt() as usize {
                        if n % i == 0 {
                                return false;
                        }
                }
                true
        }
}

#[snippet(name = "prime")]
fn prime_factorization(n: usize) -> Vec<usize> {
        let mut res: Vec<usize> = vec![];
        let mut n = n;
        let mut i = 2;

        while i <= (n as f64).sqrt() as usize {
                if n % i == 0 {
                        res.push(i);
                        n /= i;
                        i = 2;
                } else {
                        i += 1;
                }
        }

        if is_prime(n) {
                res.push(n);
        }

        res
}

#[snippet(name = "dist")]
trait Dist {
        fn dist(&self) -> usize;
}
#[snippet(name = "dist")]
impl Dist for (usize, usize) {
        fn dist(&self) -> usize {
                return (self.0 as isize - self.1 as isize).abs() as usize;
        }
}

#[snippet(name = "bfs", include = "graph", prefix = "use std::collections::VecDeque;")]
fn bfs(graph: &Graph, start_list: &Vec<usize>) -> Vec<usize> {
        let mut que: VecDeque<usize> = VecDeque::new();
        let mut dist_list: Vec<usize> = vec![INF; graph.size];

        for &start in start_list {
                que.push_back(start);
                dist_list[start] = 0;
        }

        while let Some(current) = que.pop_front() {
                for &Edge {
                        src,
                        dst,
                        weight,
                } in &graph.edges[current]
                {
                        if dist_list[dst] > dist_list[src] + weight {
                                dist_list[dst] = dist_list[src] + weight;
                                que.push_back(dst);
                        }
                }
        }

        dist_list
}

#[snippet(name = "bfs2", include = "graph2", prefix = "use std::collections::VecDeque;")]
fn bfs2(graph: &Graph2, start_list: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
        let mut que: VecDeque<(usize, usize)> = VecDeque::new();
        let mut dist_list: Vec<Vec<usize>> = vec![vec![INF; graph.w]; graph.h];

        for &(sy, sx) in start_list {
                que.push_back((sy, sx));
                dist_list[sy][sx] = 0;
        }

        while let Some((current_y, current_x)) = que.pop_front() {
                for &Edge2 {
                        src: (sy, sx),
                        dst: (dy, dx),
                        weight,
                } in &graph.edges[current_y][current_x]
                {
                        if dist_list[dy][dx] > dist_list[sy][sx] + weight {
                                dist_list[dy][dx] = dist_list[sy][sx] + weight;
                                que.push_back((dy, dx));
                        }
                }
        }

        dist_list
}

#[snippet(name = "kruskal", include = "unionfind, edge")]
fn kruskal(edge_list: &Vec<Edge>, v_size: usize, number_of_connected_components: usize) -> usize {
        let mut path_weight_sum: usize = 0;

        let mut cnt = 0;

        let mut unionfind = UnionFind::new(v_size);
        for &Edge {
                src,
                dst,
                weight,
        } in edge_list
        {
                if cnt == number_of_connected_components {
                        break;
                }
                if !unionfind.same(src, dst) {
                        unionfind.unite(src, dst);
                        path_weight_sum += weight;
                        cnt += 1;
                }
        }

        path_weight_sum
}

#[snippet(name = "matrix")]
#[derive(Clone, Debug)]
struct Matrix {
        rows: usize,
        cols: usize,
        data: Vec<Vec<usize>>,
}
#[snippet(name = "matrix", prefix = "use std::ops;")]
impl Matrix {
        fn new(data: Vec<Vec<usize>>) -> Matrix {
                Matrix {
                        rows: data.len(),
                        cols: data[0].len(),
                        data,
                }
        }
        fn mat_pow(&self, n: usize) -> Result<Matrix, &str> {
                if self.rows != self.cols {
                        return Err("Because it is not a square matrix, matrix power cannot be defined.");
                }
                if n == 0 {
                        Ok(Matrix {
                                rows: self.rows,
                                cols: self.cols,
                                data: {
                                        let mut res = vec![vec![0; self.cols]; self.rows];
                                        for i in 0..self.rows {
                                                res[i][i] = 1;
                                        }
                                        res
                                },
                        })
                } else if n % 2 == 0 {
                        Ok((self.clone() * self.clone()).unwrap().mat_pow(n / 2).unwrap())
                } else {
                        Ok((self.clone() * self.clone().mat_pow(n - 1).unwrap()).unwrap())
                }
        }
}
#[snippet(name = "matrix")]
impl ops::Mul for Matrix {
        type Output = Result<Matrix, &'static str>;

        fn mul(self, other: Self) -> Result<Matrix, &'static str> {
                if self.cols != other.rows {
                        return Err("Because the number of columns in the matrix on the left and the number of rows in the matrix on the right are different, 
                        the product of the matrices cannot be calculated.");
                }

                let mut data = vec![vec![0; other.cols]; self.rows];

                for i in 0..self.rows {
                        for j in 0..other.cols {
                                for k in 0..self.rows {
                                        data[i][j] += (self.data[i][k] * other.data[k][j]);
                                }
                        }
                }
                Ok(Self {
                        rows: self.rows,
                        cols: other.cols,
                        data,
                })
        }
}

#[snippet(name = "bit")]
#[derive(Debug)]
struct Bit {
        size: usize,
        bit: Vec<isize>,
}
#[snippet(name = "bit")]
impl Bit {
        fn new(n: usize) -> Bit {
                Bit {
                        size: n,
                        bit: vec![0; n + 1],
                }
        }
        fn sum(&self, mut idx: usize) -> isize {
                let mut res = 0;
                while idx > 0 {
                        res += self.bit[idx];
                        idx -= (idx as isize & -(idx as isize)) as usize;
                }
                res
        }
        fn add(&mut self, mut idx: usize, a: isize) {
                while idx <= self.size {
                        self.bit[idx] = self.bit[idx] + a;
                        idx += (idx as isize & -(idx as isize)) as usize;
                }
        }
        fn lower_bound(&mut self, key: isize) -> usize {
                let mut left = -1;
                let mut right = self.size as isize;

                while right - left != 1 {
                        let mid = ((left + right) / 2) as usize;
                        if key <= self.sum(mid) {
                                right = mid as isize;
                        } else {
                                left = mid as isize;
                        }
                }

                right as usize
        }
}

#[snippet(name = "comb")]
fn comb() -> Vec<Vec<usize>> {
        let mut comb = vec![vec![1 as usize; 51]; 51];
        for i in 1..=50 {
                for j in 1..=i - 1 {
                        comb[i][j] = comb[i - 1][j - 1] + comb[i - 1][j];
                }
        }
        comb
}

#[snippet(name = "segment_tree", include = "inf", prefix = "use std::cmp;")]
#[derive(Debug)]
struct SegmentTree {
        size: usize,
        nodes: Vec<usize>,
}
#[snippet(name = "segment_tree")]
impl SegmentTree {
        fn new(n: usize) -> Self {
                let size = 2u32.pow(((n - 1) as f64).log2() as u32 + 1) as usize;
                SegmentTree {
                        size,
                        nodes: vec![INF; 2 * size - 1],
                }
        }
        fn update(&mut self, mut idx: usize, x: usize) {
                idx += self.size - 1;
                self.nodes[idx] = x;
                while idx != 0 {
                        idx = (idx - 1) / 2;
                        self.nodes[idx] =
                                cmp::min(self.nodes[2 * idx + 1], self.nodes[2 * idx + 2]);
                }
        }
        fn find_sub(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> usize {
                if r <= a || b <= l {
                        return INF;
                }
                if a <= l && r <= b {
                        return self.nodes[k];
                } else {
                        return cmp::min(
                                self.find_sub(a, b, k * 2 + 1, l, (l + r) / 2),
                                self.find_sub(a, b, k * 2 + 2, (l + r) / 2, r),
                        );
                }
        }
        fn find(&self, l: usize, r: usize) -> usize {
                self.find_sub(l, r, 0, 0, self.size)
        }
}
