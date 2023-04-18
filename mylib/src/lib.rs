#[cfg(test)]
mod tests {

    use crate::adder::add_two;
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_add_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn another() {
        panic!("make test failed");
    }

    use guess::Guess;
    use rect::Rectangle;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 4,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn test_greeting() {
        let name: &str = "calor";
        let result = greeting(name);
        // assert!(result.contains("calor"));
        assert!(
            result.contains("calor"),
            "greeting did not contains name '{}',value was '{}'",
            name,
            result
        );
    }

    #[test]
    #[should_panic]
    fn test_guess() {
        Guess::new(200);
    }
}
pub mod rect {

    #[derive(Debug)]
    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
    }

    impl Rectangle {
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
}
pub mod adder {
    pub fn add_two(a: i32) -> i32 {
        a + 2
    }
}

pub fn greeting(name: &str) -> String {
    String::from(name)
}

pub mod guess {
    pub struct Guess {
        value: i32,
    }
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("guess value must between 1 and 100, got {}", value);
            }
            Guess { value }
        }
    }
}
