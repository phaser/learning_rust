struct Solution;
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle = vec![vec![1]];
        for i in 0..(num_rows - 1) {
            let mut row = vec![1];
            for j in 0..(i + 1) {
                let num = if j + 1 > i { 0 } else { triangle[i as usize][j as usize + 1] };
                row.push(triangle[i as usize][j as usize] + num);
            }
            triangle.push(row);
        }
        triangle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let num_rows = 5;
        let result = Solution::generate(num_rows);
        assert_eq!(result, vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1], vec![1, 4, 6, 4, 1]]);
    }
}