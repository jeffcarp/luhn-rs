/*! Validates strings and computes check digits using the Luhn algorithm.

It's not a great checksum, but it's used in a bunch of places (credit
card numbers, ISIN codes, etc.).  More information is available on
[wikipedia](https://en.wikipedia.org/wiki/Luhn_algorithm).
*/

/// Validates the given string using the Luhn algorithm.
///
/// Typically such strings end in a check digit which is chosen in order
/// to make the whole string validate.
pub fn valid(pan: &str) -> bool {
    // Walk the digits right-to-left without collecting them.
    let mut numbers = digits(pan).rev().peekable();
    // Reject input that contains no digits. Without this guard the Luhn
    // sum is 0 when there are no digits and `0 % 10 == 0` returns `true`,
    // causing `valid("")` and `valid("banana")` to be accepted as valid.
    if numbers.peek().is_none() {
        return false;
    }
    let mut is_odd: bool = true;
    let mut sum: u32 = 0;
    for digit in numbers {
        let value = if is_odd {
            digit
        } else {
            digit / 5 + (2 * digit) % 10
        };
        // Reduce as we go so arbitrarily long input can't overflow.
        sum = (sum + value) % 10;
        is_odd = !is_odd
    }

    sum == 0
}

/// Yields the decimal digits of `string`, silently skipping any other
/// characters (e.g. the spaces or dashes in a formatted card number).
fn digits(string: &str) -> impl DoubleEndedIterator<Item = u32> + '_ {
    string.chars().filter_map(|c| c.to_digit(10))
}

/// Splits a number in 0..=99 into its decimal digits (e.g. 25 => [2, 5], 4 => [4]).
fn u8_to_digits(n: u8) -> impl Iterator<Item = u8> {
    let (tens, units) = (n / 10, n % 10);
    if tens > 0 {
        Some(tens).into_iter().chain(Some(units))
    } else {
        None.into_iter().chain(Some(units))
    }
}

/// Computes the Luhn check digit for the given string.
///
/// The string formed by appending the check digit to the original string
/// is guaranteed to be valid.  Input must be uppercase alphanumeric
/// ASCII; panics otherwise.
pub fn checksum(input: &[u8]) -> u8 {
    // This implementation is based on the description found
    // [here](https://en.wikipedia.org/wiki/International_Securities_Identification_Number).

    // Convert a char into an index into the alphabet [0-9,A-Z].
    fn encode_char(c: u8) -> u8 {
        match c {
            b'0'..=b'9' => c - b'0',
            b'A'..=b'Z' => c - b'A' + 10,
            _ => panic!("Not alphanumeric: {}", c),
        }
    }

    // Encode the chars in the input and concatenate them digit-wise.
    // Eg. "3C" => [3, 1, 2]
    // FIXME: This allocates.  Is it necessary?
    // One char may become two digits => max length is input.len() * 2.
    let mut ds = Vec::<u8>::with_capacity(input.len() * 2);
    ds.extend(
        input
            .iter()
            .copied()
            .map(encode_char)
            .flat_map(u8_to_digits),
    );

    // The even-indexed digits, as numbered from the back, are added digit-wise.
    let checksum_even = ds
        .iter()
        .rev()
        .skip(1)
        .step_by(2)
        .copied()
        .flat_map(u8_to_digits)
        .sum::<u8>();

    // The odd-indexed digits, as numbered from the back, are doubled first.
    let checksum_odd = ds
        .iter()
        .rev()
        .step_by(2)
        .map(|&x| x * 2)
        .flat_map(u8_to_digits)
        .sum::<u8>();

    let checksum = checksum_even + checksum_odd;

    // (checksum + luhn digit) % 10 must be zero.  Working backwards:
    let digit = (10 - (checksum % 10)) % 10;

    // convert to ASCII
    digit + 48
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

    #[test]
    fn rejects_empty_input() {
        // An empty string has no check digit and cannot be a valid card number.
        assert!(!valid(""));
    }

    #[test]
    fn rejects_non_numeric_input() {
        // Non-digit characters are not valid Luhn input; the library must
        // not accept strings that contain no digits.
        assert!(!valid("banana"));
        assert!(!valid("abc"));
        assert!(!valid("----"));
        assert!(!valid("xyz"));
        assert!(!valid("ØØØ"));
    }

    #[test]
    fn accepts_known_valid_numbers() {
        // Controls: well-known valid Luhn numbers must still validate.
        assert!(valid("4111111111111111"));
        assert!(valid("49927398716"));
        assert!(valid("4012888888881881"));
        assert!(valid("79927398713"));
    }

    #[test]
    fn rejects_known_invalid_numbers() {
        // Controls: known invalid numbers must still be rejected.
        assert!(!valid("234"));
        assert!(!valid("79927398710"));
    }

    #[test]
    fn digits_extracts_digits() {
        let collect = |s| digits(s).collect::<Vec<u32>>();
        assert_eq!(collect("12345"), vec![1, 2, 3, 4, 5]);
        assert_eq!(collect("1a2b3c"), vec![1, 2, 3]);
        assert_eq!(
            collect("4111-1111 1111"),
            vec![4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
        );
        assert_eq!(collect("banana"), Vec::<u32>::new());
        assert_eq!(collect(""), Vec::<u32>::new());
        // Only ASCII 0-9 count; other Unicode numerals are skipped,
        // matching the previous `parse::<u32>()` behavior.
        assert_eq!(collect("１٣"), Vec::<u32>::new());
    }

    #[test]
    fn u8_to_digits_splits_correctly() {
        let collect = |n| u8_to_digits(n).collect::<Vec<u8>>();
        assert_eq!(collect(0), vec![0]);
        assert_eq!(collect(7), vec![7]);
        assert_eq!(collect(10), vec![1, 0]);
        assert_eq!(collect(25), vec![2, 5]);
        assert_eq!(collect(99), vec![9, 9]);
    }

    #[test]
    fn accepts_formatted_numbers() {
        assert!(valid("4111 1111 1111 1111"));
        assert!(valid("4111-1111-1111-1111"));
        assert!(!valid("4111 1111 1111 1112"));
    }

    fn validate_isin(xs: [u8; 12]) -> bool {
        let digit = checksum(&xs[0..11]);
        digit == xs[11]
    }

    #[test]
    fn validate_some_good_isins() {
        // I got these from <http://www.isin.org>.
        assert!(validate_isin(*b"US5949181045")); // Microsoft
        assert!(validate_isin(*b"US38259P5089")); // Google
        assert!(validate_isin(*b"US0378331005")); // Apple
        assert!(validate_isin(*b"BMG491BT1088")); // Invesco
        assert!(validate_isin(*b"IE00B4BNMY34")); // Accenture
        assert!(validate_isin(*b"US0231351067")); // Amazon
        assert!(validate_isin(*b"US64110L1061")); // Netflix
        assert!(validate_isin(*b"US30303M1027")); // Facebook
        assert!(validate_isin(*b"CH0031240127")); // BMW Australia
        assert!(validate_isin(*b"CA9861913023")); // Yorbeau Res
    }

    #[test]
    fn fail_some_bad_isins() {
        assert!(!validate_isin(*b"US5949181040")); // Microsoft (checksum zeroed)
        assert!(!validate_isin(*b"US38259P5080")); // Google (checksum zeroed)
        assert!(!validate_isin(*b"US0378331000")); // Apple (checksum zeroed)
        assert!(!validate_isin(*b"BMG491BT1080")); // Invesco (checksum zeroed)
        assert!(!validate_isin(*b"IE00B4BNMY30")); // Accenture (checksum zeroed)
        assert!(!validate_isin(*b"US0231351060")); // Amazon (checksum zeroed)
        assert!(!validate_isin(*b"US64110L1060")); // Netflix (checksum zeroed)
        assert!(!validate_isin(*b"US30303M1020")); // Facebook (checksum zeroed)
        assert!(!validate_isin(*b"CH0031240120")); // BMW Australia (checksum zeroed)
        assert!(!validate_isin(*b"CA9861913020")); // Yorbeau Res (checksum zeroed)

        assert!(!validate_isin(*b"SU5941981045")); // Microsoft (two chars transposed)
        assert!(!validate_isin(*b"US3825P95089")); // Google (two chars transposed)
        assert!(!validate_isin(*b"US0378313005")); // Apple (two chars transposed)
        assert!(!validate_isin(*b"BMG491BT0188")); // Invesco (two chars transposed)
        assert!(!validate_isin(*b"IE00B4BNM3Y4")); // Accenture (two chars transposed)
        assert!(!validate_isin(*b"US2031351067")); // Amazon (two chars transposed)
        assert!(!validate_isin(*b"US61410L1061")); // Netflix (two chars transposed)
        assert!(!validate_isin(*b"US30033M1027")); // Facebook (two chars transposed)
        assert!(!validate_isin(*b"CH0032140127")); // BMW Australia (two chars transposed)
        assert!(!validate_isin(*b"CA9861193023")); // Yorbeau Res (two chars transposed)
    }

    #[test]
    fn readme() {
        // A string which doesn't validate
        let mut s = "11111111".to_string();
        assert!(!valid(&s));

        // Let's fix that
        s.push(checksum(s.as_bytes()) as char);
        assert_eq!(s, "111111118");
        assert!(valid(&s));
    }
}
