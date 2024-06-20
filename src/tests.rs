#[cfg(test)]
mod tests {
    use crate::{roll, Expression, Operation};
    use crate::Dice::D10;

    #[test]
    fn it_works() {
        let result = roll(
            &Expression {
                number: 5,
                dice: D10,
                modifiers: vec![(Operation::Add, 5)],
            }).unwrap();
        println!("{}", result);
        assert_eq!(result.value.len(), 5);
    }
}