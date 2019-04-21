struct Solution {}
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if num_rows > 0 {
            result.push(vec![1]);
            for j in 0..(num_rows as usize) - 1 {
                let next_row = Solution::pascal(&result[j]);
                result.push(next_row);
            }
        }
        result
    }
    pub fn pascal(row: &Vec<i32>) -> Vec<i32> {
        let mut result = vec![1];
        for j in 0..row.len() {
            if j + 1 >= row.len() {
                result.push(1);
            } else {
                result.push(row[j] + row[j + 1]);
            }
        }
        result
    }
}

fn main() {
    println!("{:?}", Solution::generate(5));
}
