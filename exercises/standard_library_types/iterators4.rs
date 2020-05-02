// iterators4.rs

// I AM DONE

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return factorial of num
    // Do not use:
    // - return

    // let mut f = 1;
    // for i in  1..=num{
    //     f *= i;
    // }
    // f

    // For extra fun don't use:
    // - imperative style loops (for, while)
    // - additional variables

    // if num == 1 {1} else{factorial(num-1) * (num)}
    
    // For the most fun don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    // (1..=num).product()
    (1..=num).fold(1, |acc, x| acc * x)

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
