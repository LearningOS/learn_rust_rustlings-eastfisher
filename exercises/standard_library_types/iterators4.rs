// iterators4.rs


pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    // 0 -> 0 = 0
    // 1 -> 1 = 1
    // 2 -> 2 * 1 = 2
    // 3 -> 3 * 2 * 1 = 6
    // 不用递归的不会
    // 看了下答案, 需要用到fold()
    match num {
        0 => 0,
        1 => 1,
        _ => num * factorial(num-1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
