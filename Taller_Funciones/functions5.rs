// I AM NOT DONE

fn square(num: i32) -> i32 {
    num * num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_function() {
        let answer = square(3);
        assert_eq!(answer, 9); // Verifica que el resultado sea el esperado
    }
}
