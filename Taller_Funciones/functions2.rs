fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_function() {
        call_me(3);
        // Agrega aserciones u otras pruebas aquí si es necesario
    }
}
