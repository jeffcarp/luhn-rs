/*! Validates strings using the Luhn algorithm.

It's not a great checksum, but it's used in a bunch of places (credit
card numbers, ISIN codes, etc.).  More information is available on
[wikipedia](https://en.wikipedia.org/wiki/Luhn_algorithm).
*/

/// Validates the given string using the Luhn algorithm.
///
/// Typically such strings end in a check digit which is chosen in order
/// to make the whole string validate.
pub fn valid(pan: &str) -> bool {
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
    use super::*;

    #[test]
    fn accepts_4111111111111111() {
        assert!(valid("4111111111111111"));
    }

    #[test]
    fn accepts_49927398716() {
        assert!(valid("49927398716"));
    }

    #[test]
    fn rejects_4111111111111112() {
        assert!(!valid("4111111111111112"));
    }

    #[test]
    fn rejects_234() {
        assert!(!valid("234"));
    }
}
