#[cfg(test)]
mod tests {
    #[test]
    fn test_variables5() {
        let mut number = "T-H-R-E-E"; // don't change this line
        println!("Spell a Number : {}", number);
        number = "3"; // don't rename this variable
        let number_as_int:i32 = number.parse().expect("Failed to parse");
        println!("Number plus two is : {}", number_as_int + 2);
    }
}