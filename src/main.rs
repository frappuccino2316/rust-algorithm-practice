use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        r: usize,
        c: usize,
        sy: usize,
        sx: usize,
        gy: usize,
        gx: usize,
    }

    let mut board: Vec<Vec<char>> = vec![vec![]; r];
    let mut g: Vec<Vec<usize>> = vec![vec![]; (r - 2) * (c - 2) + 1];

    for i in board.iter_mut().take(r) {
        input! {
            cl: String,
        }

        let row: Vec<char> = cl.chars().collect();
        for j in row {
            i.push(j);
        }
    }

    for (i, _) in board.iter().enumerate().take(r - 1).skip(1) {
        for j in 1..c {
            if board[i][j] == '.' && board[i][j + 1] == '.' {
                let idx1 = (i - 1) * (c - 2) + j;
                let idx2 = (i - 1) * (c - 2) + j + 1;
                g[idx1].push(idx2);
                g[idx2].push(idx1);
            }
        }
    }

    for i in 1..r {
        for j in 1..c {
            if board[i][j] == '.' && board[i + 1][j] == '.' {
                let idx1 = (i - 1) * (c - 2) + j;
                let idx2 = i * (c - 2) + j;
                g[idx1].push(idx2);
                g[idx2].push(idx1);
            }
        }
    }

    // スタート座標の頂点番号
    let start_id = (sy - 2) * (c - 2) + sx - 1;
    // ゴール座標の頂点番号
    let goal_id = (gy - 2) * (c - 2) + gx - 1;

    // ↓から幅優先探索
    // キューを作成し、start_idを初期値に入れる
    let mut queue = VecDeque::new();
    queue.push_back(start_id);

    // 拡張点の最短距離を格納する変数を作成
    // スタート位置の頂点は0
    let mut dist = vec![-1; g.len()];
    dist[start_id] = 0;

    // 隣接する頂点のうち小さい順に探索するため、隣接する頂点のリストをソートしておく
    for adjacent_list in &mut g {
        adjacent_list.sort_unstable();
    }

    while !queue.is_empty() {
        // キューの先頭をチェック
        let pos = queue[0];
        queue.pop_front();

        for i in 0..g[pos].len() {
            let nex = g[pos][i];
            if dist[nex] == -1 {
                dist[nex] = dist[pos] + 1;
                queue.push_back(nex);
            }
        }
    }

    println!("{}", dist[goal_id]);
}
