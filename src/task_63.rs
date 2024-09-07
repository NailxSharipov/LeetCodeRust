struct Task63;

impl Task63 {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let last_line = &obstacle_grid[m - 1];
        let n = last_line.len();

        let mut max_i = -1;
        for i in 0..n {
            if last_line[i] == 1 {
                max_i = i as i64;
            }
        }

        let mut buffer = vec![1i32; n];
        if max_i >= 0 {
            for i in 0..(max_i + 1) as usize {
                buffer[i] = 0;
            }
        }

        let mut j = m  as i64 - 2;
        while j >= 0 {
            let mut i = n as i64 - 1;
            let mut s = obstacle_grid[j as usize][i as usize];
            while i >= 0 {
                let is_obstacle = obstacle_grid[j as usize][i as usize] == 1;
                if is_obstacle {
                    buffer[i as usize] = 0;
                    s = 0;
                } else {
                    let ss = buffer[i as usize];
                    s += ss;
                    buffer[i as usize] = s;
                }

                i -= 1;
            }

            j -= 1;
        }

        buffer[0]
    }
}


#[cfg(test)]
mod tests {
    use crate::task_63::Task63;

    #[test]
    fn test_0() {
        let data = vec![
            vec![0, 0, 0],
            vec![0, 1, 0],
            vec![0, 0, 0],
        ];
        let s = Task63::unique_paths_with_obstacles(data);
        assert_eq!(s, 2);
    }

    #[test]
    fn test_1() {
        let data = vec![
            vec![0, 1],
            vec![0, 0],
        ];
        let s = Task63::unique_paths_with_obstacles(data);
        assert_eq!(s, 1);
    }

    #[test]
    fn test_2() {
        let data = vec![
            vec![0, 0],
            vec![0, 1],
        ];
        let s = Task63::unique_paths_with_obstacles(data);
        assert_eq!(s, 0);
    }

    #[test]
    fn test_3() {
        let data = vec![
            vec![0, 0, 0],
            vec![0, 1, 0],
            vec![0, 0, 0],
        ];
        let s = Task63::unique_paths_with_obstacles(data);
        assert_eq!(s, 2);
    }
}