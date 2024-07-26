#[cfg(test)]
mod tests {
    use std::fs;
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

    #[test]
    fn stat() {
        let result1 = roll_str("2056d10").unwrap();
        let result2 = roll_str("2056d10").unwrap();
        let result3 = roll_str("2056d10").unwrap();
        let result4 = roll_str("2056d10").unwrap();
        let result5 = roll_str("2056d10").unwrap();
        let result6 = roll_str("2056d10").unwrap();
        let mut r: Vec<f32> = vec![];
        for i in 0..result1.value.len() {
            let f = (result1.value[i] as f32
                + result2.value[i] as f32
                + result3.value[i] as f32
                + result4.value[i] as f32
                + result5.value[i] as f32
                + result6.value[i] as f32) / 6f32;
            r.push(f);
        }
        fs::write("stat.csv", r.into_iter().map(|i| format!("{}\n", i)).collect::<String>()).expect("error");
        assert!(true);
    }
}