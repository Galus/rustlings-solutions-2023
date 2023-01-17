// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.

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
    /*
     * 5! = 5 * 4 * 3 * 2 * 1 = 20 * 3 * 2 = 60 * 2 = 120
     * 1 2 3 4 5
     * acc: 1 , e: 1
     * acc: 2, e:2 
     * acc: 8, e:3
     * acc: 20, e:4
     * acc: 40, e:5     
     *
     */
    if num == 0 { 
        return 1
    }
    (1..=num).fold(1, |product,element| {
        println!("prod: {} el: {}", product, element);
        product*element
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

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
