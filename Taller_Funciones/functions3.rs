fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_function() {
        call_me(3); // Proporciona un valor u32 como argumento
        // Agrega aserciones u otras pruebas aquí si es necesario
    }
}

