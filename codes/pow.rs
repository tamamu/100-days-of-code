struct Solution {}
impl Solution {
    //// Stack overflow!!
    // pub fn my_pow(x: f64, n: i32) -> f64 {
    //     if n == 0 {
    //         1.0
    //     } else if n > 0 {
    //         x * Solution::my_pow(x, n - 1)
    //     } else {
    //         1.0 / Solution::my_pow(x, -n)
    //     }
    // }
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut ans = 1f64;
        let mut n = n as i64;
        let mut mul = x as f64;
        // x^(-n) = (1/x)^n
        if n < 0 {
            n = -n;
            mul = 1f64 / x;
        }
        while n > 0 {
            // when n is odd, multiply x once
            if n & 1 == 1 {
                ans *= mul;
            }
            // x^2n = (x * x)^n
            mul *= mul;
            // when n is odd, n = (n - 1) / 2
            // or else, n = n / 2
            n >>= 1;
        }
        ans
    }
}

fn main() {
    assert_eq!(Solution::my_pow(2.000000, 10), 1024.00000);
    assert_eq!(Solution::my_pow(2.000000, -2), 0.25000);
    println!("{}", Solution::my_pow(0.000001, 2147483647));
    println!("{}", Solution::my_pow(2.000000, -2147483648));
}
