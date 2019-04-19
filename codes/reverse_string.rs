struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let length = s.len();
        let max_count = length / 2;
        for j in 0..max_count {
            s.swap(j, length - j - 1);
        }
    }
}

fn main() {
    let mut s = "hello".chars().collect::<Vec<char>>();
    Solution::reverse_string(&mut s);
    println!("{:?}", s);
}
