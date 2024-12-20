pub mod i32 {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn mul(a: i32, b: i32) -> i32 {
        a * b
    }

    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }

    pub fn fib(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return n;
        }

        add(fib(sub(n, 1)), fib(sub(n, 2)))
    }
}

pub mod f64 {
    pub fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    pub fn mul(a: f64, b: f64) -> f64 {
        a * b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_i32() {
        assert_eq!(i32::add(1, 2), 3);
    }

    #[test]
    fn test_mul_i32() {
        assert_eq!(i32::mul(2, 3), 6);
    }

    #[test]
    fn test_add_f64() {
        assert_eq!(f64::add(1.0, 2.0), 3.0);
    }

    #[test]
    fn test_mul_f64() {
        assert_eq!(f64::mul(2.0, 3.0), 6.0);
    }

    #[test]
    fn test_fib() {
        assert_eq!(i32::fib(0), 0);
        assert_eq!(i32::fib(1), 1);
        assert_eq!(i32::fib(2), 1);
        assert_eq!(i32::fib(3), 2);
        assert_eq!(i32::fib(4), 3);
    }

    #[test]
    fn test_sub() {
        assert_eq!(i32::sub(1, 2), -1);
    }
}
