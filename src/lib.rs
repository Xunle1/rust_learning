pub mod tests {
    pub fn add_two(v: i32) -> i32 {
        v + 2
    }
}

#[cfg(test)]
mod unit_tests {
    #[test]
    fn test_add_two() {
        let ret = crate::tests::add_two(3);
        assert!(ret == 5);
    }
}
