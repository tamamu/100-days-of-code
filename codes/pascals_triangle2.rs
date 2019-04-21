// static mut MEMO: [[i32; 34]; 34] = [[0; 34]; 34];
struct Solution {}
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_indexu = row_index as usize;
        let mut result = vec![1];
        if row_index > 0 {
            result.resize(row_indexu + 1, 0);
            let upper_row = Solution::get_row(row_index - 1);
            for j in 1..row_indexu {
                result[j] = upper_row[j - 1] + upper_row[j];
            }
            result[row_indexu] = 1;
        }
        result
    }
    // pub fn get_row(row_index: i32) -> Vec<i32> {
    //     let mut result = Vec::new();
    //     for j in 0..row_index + 1 {
    //         result.push(Solution::pascal_number(row_index as usize, j as usize));
    //     }
    //     result
    // }
    // pub fn pascal_number(j: usize, k: usize) -> i32 {
    //     unsafe {
    //         if MEMO[j][k] == 0 {
    //             if j == 0 || k == 0 || j == k {
    //                 MEMO[j][k] = 1;
    //             } else {
    //                 MEMO[j][k] =
    //                     Solution::pascal_number(j - 1, k - 1) + Solution::pascal_number(j - 1, k);
    //             }
    //         }
    //         MEMO[j][k]
    //     }
    // }
}

fn main() {
    println!("{:?}", Solution::get_row(3));
}
