use std::collections::VecDeque;
pub struct Solution;
impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        #[derive(PartialEq)]
        struct Point {
            x: usize,
            y: usize,
        }
        struct Direction {
            x: i32,
            y: i32,
        }
        const fn mp(x: usize, y: usize) -> Point {
            Point { x, y }
        }
        const fn md(x: i32, y: i32) -> Direction {
            Direction { x, y }
        }
        let direction = [
            md(-1, -1),
            md(-1, 0),
            md(-1, 1),
            md(0, -1),
            md(0, 1),
            md(1, -1),
            md(1, 0),
            md(1, 1),
        ];

        let n = grid.len();
        let start_point = mp(0, 0);
        let end_point = mp(n - 1, n - 1);
        if grid[start_point.x][start_point.y] != 0 || grid[end_point.x][end_point.y] != 0 {
            return -1;
        }
        if n == 1 {
            return 1;
        }

        let mut queue = VecDeque::from([mp(0, 0)]);
        let mut dis_map = vec![vec![0; n]; n];
        dis_map[start_point.x][start_point.y] = 1;

        while let Some(p) = queue.pop_front() {
            for d in &direction {
                let new_x = p.x as i32 + d.x;
                let new_y = p.y as i32 + d.y;
                let new_ux = new_x as usize;
                let new_uy = new_y as usize;
                if new_x < 0
                    || new_ux >= n
                    || new_y < 0
                    || new_uy >= n
                    || grid[new_ux][new_uy] != 0
                    || dis_map[new_ux][new_uy] > 0
                {
                    continue;
                }

                dis_map[new_ux][new_uy] = dis_map[p.x][p.y] + 1;
                let np = mp(new_ux, new_uy);
                if np == end_point {
                    return dis_map[new_ux][new_uy];
                }

                queue.push_back(np);
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use crate::{solution::shortest_path_binary_matrix::Solution, vecnd};

    #[test]
    fn it_works() {
        fn t(grid: Vec<Vec<i32>>, res: i32) {
            assert_eq!(Solution::shortest_path_binary_matrix(grid), res);
        }
        t(vecnd![[0, 1], [1, 0]], 2);
        t(vecnd![[0, 0, 0], [1, 1, 0], [1, 1, 0]], 4);
        t(vecnd![[1, 0, 0], [1, 1, 0], [1, 1, 0]], -1);
        t(vecnd![[0]], 1);
    }
}
