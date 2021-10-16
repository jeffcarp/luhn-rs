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
    luhn3::valid(pan.as_bytes())
}

/// Computes the Luhn check digit for the given string.
///
/// The string formed by appending the check digit to the original string
/// is guaranteed to be valid.  Input must be uppercase alphanumeric
/// ASCII; panics otherwise.
pub fn checksum(input: &[u8]) -> u8 {
    luhn3::checksum(input).expect("Input is not valid")
}

/// Computes the Luhn check digit for the given string.
///
/// The string formed by appending the check digit to the original string
/// is guaranteed to be valid.
pub fn safe_checksum(input: &[u8]) -> Option<u8> {
    luhn3::checksum(input)
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
