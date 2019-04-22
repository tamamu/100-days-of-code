static mut CACHE: [i32; 31] = [-1; 31];

struct Solution {}
impl Solution {
    pub fn fib(n: i32) -> i32 {
        unsafe {
            let nu = n as usize;
            if CACHE[nu] < 0 {
                if n == 0 || n == 1 {
                    CACHE[nu] = n;
                } else {
                    CACHE[nu] = Solution::fib(n - 1) + Solution::fib(n - 2);
                }
            }
            CACHE[nu]
        }
    }
}

fn main() {
    assert_eq!(1, Solution::fib(2));
    assert_eq!(2, Solution::fib(3));
    assert_eq!(3, Solution::fib(4));
}
