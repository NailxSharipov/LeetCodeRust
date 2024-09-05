struct Task62;

impl Task62 {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m < 2 || n < 2 {
            return 1
        }

        let mut array = vec![1; n as usize];

        for _ in 1..m {
            let mut s = 1;
            for x in 1..n {
                let i = x as usize;
                s += array[i];
                array[i] = s;
            }
        }

        array[n as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::task_62::Task62;

    #[test]
    fn test_0() {
        let result = Task62::unique_paths(3, 7);
        assert_eq!(result, 28);
    }

    #[test]
    fn test_1() {
        let result = Task62::unique_paths(3, 2);
        assert_eq!(result, 3);
    }
}