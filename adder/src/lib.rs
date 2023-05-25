pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 8,
        };
        let smaller = Rectangle {
            width: 5,
            height: 2,
        };

        assert!(larger.can_hold(&smaller));
    }
}
