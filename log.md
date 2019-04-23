# 100 Days Of Code - Log

### Day 1: April 19, 2019

**Today's Progress**: I've solved a problem on LeetCode.

**Thoughts** I've written many of codes for nearly 10 years, but I don't have a systematic knowledge of algorithms. I'm going to learn it from now on! Today I've solved "Reverse String" in Rustlang. In the meanwhile, I found a `swap` method for vector. It's very functional because Rustlang doesn't permit take more than a mutable reference for an element of a vector.  In normal, an element should be cloned and be swapped with a temporary variable, but this way takes extra memory.

**Link(s) to work**
1. [Code](https://github.com/tamamu/100-days-of-code/commit/9cc8d4f6ab1c4372a1c5044b704b1f80f12e2eca)
2. [Reference of `swap` Method](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.swap)


### Day 2: April 20, 2019

**Today's Progress**: I've solved a problem on LeetCode.

**Thoughts** There may exist a more memory-efficient way because my implementation has cloned each element. I think the most efficient way is unsafe. To implement recursive structures are my challenge.

**Link(s) to work**
1. [Code](https://github.com/tamamu/100-days-of-code/commit/3a4868867eecf60355230ca337979bbef3a68c0c)
2. [Reference of `Display` Trait](https://doc.rust-lang.org/std/fmt/trait.Display.html)


### Day 3: April 21, 2019

**Today's Progress**: I've solved a problem on LeetCode.

**Thoughts** First, memoization in Rust is more difficult than other languages! The global variable regulated to be sized in Rust, just like C. Second, using `Box` pointer is also difficult for recursive problem like Linked List. In this case, dividing into initial process and the remainder was key point to solve problem recursively.

**Link(s) to work**
1. [Pascal's Triangle1](https://github.com/tamamu/100-days-of-code/commit/40e249cc5b855841c6f0a2d6262a2ac012445131)
2. [Pascal's Triangle2 (memoized)](https://github.com/tamamu/100-days-of-code/commit/e75df4b6a89270087200846c273fde1efce54493)
3. [Pascal's Triangle3](https://github.com/tamamu/100-days-of-code/commit/d86e3a9b2ebf7f739d864f0f369374e894f1be82)
4. [Reverse Linked List](https://github.com/tamamu/100-days-of-code/commit/d818f60e17fb77e2caf10a4cd1882c448dac6c43)
5. [Rust 0ms Iterative](https://leetcode.com/explore/learn/card/recursion-i/251/scenario-i-recurrence-relation/2378/discuss/225512/Rust-0ms-Iterative)
6. [Reverse a list in Scala](http://www.thedigitalcatonline.com/blog/2015/04/07/99-scala-problems-05-reverse/#the-recursive-solution)
7. [Reference of `Option::take` Method](https://doc.rust-lang.org/std/option/enum.Option.html#method.take)
8. [Reference of Primitive `array` Type](https://doc.rust-lang.org/std/primitive.array.html)


### Day 4: April 22, 2019

**Today's Progress**: I've solved a problem on LeetCode.

**Thoughts** I know that `lazy_static` macro is commonly used for memoization, but it may not be able to use that on LeetCode. What else can I do memoization with unsized cache without `lazy_static`?

**Link(s) to work**
1. [Fibonacci Number](https://github.com/tamamu/100-days-of-code/commit/81cf70ff2aa5901227d7499e57c6222d1a5e5bb2)
2. [Climbing Stairs](https://github.com/tamamu/100-days-of-code/commit/19d9f3c0c45b13c44d6b1cbd841f86b705e26773)
3. [Reference of `thread_local!` Macro](https://doc.rust-lang.org/std/thread/struct.LocalKey.html)


### Day 5: April 23, 2019

**Today's Progress**: I've solved a problem on LeetCode, and done an experiment to confirm `n % 2 == 1` is equivalent to `n & 1 == 1`.

**Thoughts** It's a bit difficult to calculate time complexity and space complexity of recursive function. I don't know Rustlang supports tail recursion optimization, but it seems to be enable for release building.

**Link(s) to work**
1. [Maximum Depth of Binary Tree](https://github.com/tamamu/100-days-of-code/commit/2f32dbfd1f1ca1edd180d9ae60d987f05a9be670)
2. [Pow(x, n)](https://github.com/tamamu/100-days-of-code/commit/74f47c0acd964287679c961b6002db2dc010ddf9)
3. [Rust Trick Solution of Pow(x, n)](https://leetcode.com/explore/learn/card/recursion-i/256/complexity-analysis/2380/discuss/243952/Rust-Tricky-Solution)
