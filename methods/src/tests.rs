#[cfg(test)]
mod tests {
    use crate::Rectangle;

    #[test]
    fn test_area() {
        let rectangle = Rectangle {
            width: 30,
            height: 50,
        };
        assert_eq!(rectangle.area(), 1500);
    }

    #[test]
    fn test_width() {
        let rect = Rectangle {
            width: 10,
            height: 20,
        };
        assert_eq!(rect.width(), true);
    }

    #[test]
    fn test_can_hold() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };
        assert_eq!(rect1.can_hold(&rect2), true);
        assert_eq!(rect1.can_hold(&rect3), false);
    }
}
