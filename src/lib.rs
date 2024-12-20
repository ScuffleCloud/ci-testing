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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(2, 3), 6);
    }

    #[test]
    fn test_fib() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(1, 2), -1);
    }
}
