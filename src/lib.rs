pub mod i32 {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn mul(a: i32, b: i32) -> i32 {
        a * b
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
}
