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
