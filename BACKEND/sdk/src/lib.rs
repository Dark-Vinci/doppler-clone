pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod my_module {
    pub fn div(a: usize, b: usize) -> Option<usize> {
        if b == 0 {
            return None;
        }

        return Some(a / b);
    }

    pub fn mul(a: usize, b: usize) -> usize {
        a * b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
