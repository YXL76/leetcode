/*
 * @lc app=leetcode id=79 lang=rust
 *
 * [79] Word Search
 */

pub struct Solution;

// @lc code=start
impl Solution {
    const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let chars: Vec<char> = word.chars().collect();
        let len = chars.len();
        let rows = board.len();
        let cols = board[0].len();

        if rows * cols < len {
            return false;
        }

        let mut flags = vec![vec![false; cols]; rows];
        for i in 0..rows {
            for j in 0..cols {
                if board[i][j] == chars[0]
                    && Self::find(
                        &chars,
                        &board,
                        &mut flags,
                        &len,
                        &(rows as i32),
                        &(cols as i32),
                        i as i32,
                        j as i32,
                        1,
                    )
                {
                    return true;
                }
            }
        }

        false
    }

    fn find(
        chars: &Vec<char>,
        board: &Vec<Vec<char>>,
        mut flags: &mut Vec<Vec<bool>>,
        len: &usize,
        rows: &i32,
        cols: &i32,
        i: i32,
        j: i32,
        step: usize,
    ) -> bool {
        if step == *len {
            return true;
        }
        flags[i as usize][j as usize] = true;
        for (di, dj) in Self::DIRECTIONS.iter() {
            let ni = i + di;
            let nj = j + dj;
            if (ni < 0
                || ni >= *rows
                || nj < 0
                || nj >= *cols
                || flags[ni as usize][nj as usize]
                || board[ni as usize][nj as usize] != chars[step])
            {
                continue;
            }
            if Self::find(chars, board, flags, len, rows, cols, ni, nj, step + 1) {
                return true;
            }
        }
        flags[i as usize][j as usize] = false;
        false
    }
}
// @lc code=end
