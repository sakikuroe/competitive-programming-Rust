use proconio::input;

struct Point {
    x: isize,
    y: isize,
}

fn dfs(src: Point, h: usize, w: usize, cnt: usize, ans: &mut usize, board: &mut Vec<Vec<isize>>) {
    board[src.y as usize][src.x as usize] = 0;

    if cnt > *ans {
        *ans = cnt;
    }

    let v: Vec<Point> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)]
        .into_iter()
        .map(|(x, y)| Point {
            x,
            y,
        })
        .collect::<Vec<Point>>();
    for i in 0..4 {
        let dest = Point {
            y: src.y + v[i].y,
            x: src.x + v[i].x,
        };
        if 0 <= dest.y
            && dest.y < h as isize
            && 0 <= dest.x
            && dest.x < w as isize
            && board[dest.y as usize][dest.x as usize] == 1
        {
            dfs(
                Point {
                    y: dest.y,
                    x: dest.x,
                },
                h,
                w,
                cnt + 1,
                ans,
                board,
            )
        }
    }

    board[src.y as usize][src.x as usize] = 1;
}

fn main() {
    // Input
    input! {
        m:usize, n:usize,
        mut board: [[isize;m];n],
    }

    // Solve
    let mut ans: usize = 0;

    for i in 0..n {
        for j in 0..m {
            if board[i][j] == 1 {
                dfs(
                    Point {
                        y: i as isize,
                        x: j as isize,
                    },
                    n,
                    m,
                    1,
                    &mut ans,
                    &mut board,
                );
            }
        }
    }

    // Output
    println!("{}", ans);
}
