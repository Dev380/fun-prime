use rayon::prelude::*;
pub fn is_prime(num: u128, first: u128) -> bool {
    (first..num).into_par_iter().all(|i| num % i != 0)
    /*
    for i in first ..num {
        if num  % i == 0 {
            return false;
        } else {
            continue;
        };
    }
    true
    */
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prime_true() {
        assert!(is_prime(2, 2));
    }
    #[test]
    fn test_prime_false() {
        assert!(!is_prime(24, 2));
    }
    #[test]
    #[ignore]
    fn expensive_prime_true() {
        assert!(is_prime(546754754801 * 546754754771, 2))
    }
}
