struct Solution {}

use std::collections::HashMap;
static mut CACHE: [i32; 100000] = [-1; 100000];

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        unsafe {
            let nu = n as usize;
            if CACHE[nu] < 0 {
                if n == 1 || n == 2 {
                    CACHE[nu] = n;
                } else {
                    CACHE[nu] = Solution::climb_stairs(n - 1) + Solution::climb_stairs(n - 2);
                }
            }
            CACHE[nu]
        }
    }
}

fn main() {
    println!("{}", Solution::climb_stairs(2));
    println!("{}", Solution::climb_stairs(3));
}
