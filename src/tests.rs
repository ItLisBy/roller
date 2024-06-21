#[cfg(test)]
mod tests {
    use crate::{roll, Expression, Operation, parse, roll_str};

    #[test]
    fn it_works() {
        let result = roll(
            &Expression {
                number: 5,
                dice: 10,
                modifiers: vec![(Operation::Add, 5)],
            }).unwrap();
        println!("{}", result);
        assert_eq!(result.value.len(), 5);
    }

    #[test]
    fn parse_test() -> Result<(), ()> {
        let expr = parse("5d10+5").unwrap();
        println!("{:?}", expr);
        Ok(())
    }

    #[test]
    fn roll_str_test() {
        let result1 = roll_str("5d10+5").unwrap();
        let result2 = roll_str("d10+5").unwrap();
        let result3 = roll_str("5d10").unwrap();
        assert_eq!(result1.value.len(), 5);
        assert_eq!(result2.value.len(), 1);
        assert_eq!(result3.value.len(), 5);
    }
}