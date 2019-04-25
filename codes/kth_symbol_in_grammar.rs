struct Solution {}
impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        let mid = 2i32.pow((n - 1) as u32) / 2;
        if k == 1 {
            0
        } else if k <= mid {
            Solution::kth_grammar(n - 1, k)
        } else {
            if Solution::kth_grammar(n, k - mid) == 0 {
                1
            } else {
                0
            }
        }
    }
}

fn main() {
    println!("{}", Solution::kth_grammar(4, 5));
}
