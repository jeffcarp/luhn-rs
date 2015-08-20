pub fn luhn_valid(pan: &str) -> bool {
    let mut numbers = string_to_ints(pan);
    numbers.reverse();
    let mut is_odd: bool = true;
    let mut odd_sum: u32 = 0;
    let mut even_sum: u32 = 0;
    for digit in numbers {
        if is_odd {
            odd_sum += digit;
        } else {
            even_sum += digit / 5 + (2 * digit) % 10;
        }
        is_odd = !is_odd
    }

    (odd_sum + even_sum) % 10 == 0
}

fn string_to_ints(string: &str) -> Vec<u32> {
    let mut numbers = vec![];
    for c in string.chars() {
        let value = c.to_string().parse::<u32>();
        match value {
            Ok(v) => numbers.push(v),
            Err(e) => println!("error parsing number: {:?}", e),
        }
    }
    numbers
}
 
#[cfg(test)]
mod tests {
    use super::luhn_valid;

    #[test]
    fn accepts_4111111111111111() {
        assert!(luhn_valid("4111111111111111"));
    }

    #[test]
    fn accepts_49927398716() {
        assert!(luhn_valid("49927398716"));
    }

    #[test]
    fn rejects_4111111111111112() {
        assert!(!luhn_valid("4111111111111112"));
    }

    #[test]
    fn rejects_234() {
        assert!(!luhn_valid("234"));
    }
}
