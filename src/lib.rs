#![doc = include_str!("../README.md")]

/// Validates the given string using the Luhn algorithm.
///
/// Typically such strings end in a check digit which is chosen in order
/// to make the whole string validate.
pub fn valid(pan: &str) -> bool {
    validate_slice(pan.as_bytes())
}

/// Computes the Luhn check digit for the given string.
///
/// The string formed by appending the check digit to the original string
/// is guaranteed to be valid.  Input must be uppercase alphanumeric
/// ASCII; panics otherwise. Use [calc_checksum] if panics are not acceptable.
pub fn checksum(input: &[u8]) -> u8 {
    calc_checksum(input).expect("Non alphanum ASCII") as u8
}

/// Iterate over decimal digits of base 10 or base 36 number
struct DigitsIterator<I>(Option<u8>, I);
impl<I> DigitsIterator<I> {
    pub fn new(vals: I) -> Self where {
        Self(None, vals)
    }
}
impl<I> Iterator for DigitsIterator<I>
where
    I: Iterator<Item = u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.0.take() {
            return Some(n);
        }
        let d = self.1.next()?;
        if d < 10 {
            Some(d)
        } else {
            self.0 = Some(d % 10);
            Some(d / 10)
        }
    }
}

#[derive(Default)]
struct Dig {
    sum: usize,
    five_or_higher: usize,
}

fn luhn_fold(raw: &[u8]) -> Option<(Dig, Dig)> {
    let mut invalid = false;
    let base36 = raw.iter().map(|c| {
        if (b'0'..=b'9').contains(c) {
            *c - b'0'
        } else if (b'A'..=b'Z').contains(c) {
            *c - b'A' + 10
        } else {
            invalid = true;
            0
        }
    });

    // Luhn algorithm calls for a sum of digits in odd and even places starting
    // from the end of the stream with additional transmogrification applied to
    // digits on even (if calculating missing check digit) or odd (if checking the
    // check digit) places starting from the right. Since we don't know in advance
    // which digits will be on odd and which - on even places we collect them
    // into two separate sums also collecting a bit of extra information that
    // would allow us to perform transmogrification when needed once we know
    // which group of digits needs to be changed
    let mut sum = (Dig::default(), Dig::default());
    for d in DigitsIterator::new(base36) {
        if d >= 5 {
            sum.0.five_or_higher += 1;
        }
        sum.0.sum += usize::from(d);
        sum = (sum.1, sum.0)
    }
    if invalid {
        return None;
    }
    Some(sum)
}

/// Validate a Luhn checksum of an arbitrary slice
///
/// Takes a byte slice with Luhn check digit _present_ and checks if it is valid.
/// Invalid ISINs (non alphadecimal and non ASCII) are invalid for obvious reasons.
///
/// ```rust
/// # use luhn::*;
/// // A valid ISIN
/// let microsoft = b"US5949181045";
/// assert!(validate_slice(microsoft));
///
/// // Not a valid ISIN
/// let banana = b"banana";
/// assert!(!validate_slice(banana));
///
/// // Even less valid ISIN
/// let noms = "口水鸡";
/// assert!(!validate_slice(noms.as_bytes()));
/// use std::io;
/// let mut input = String::new();
/// io::stdin().read_line(&mut input)?;
/// # Ok::<(), io::Error>(())
/// ```
pub fn validate_slice(raw: &[u8]) -> bool {
    if let Some(sum) = luhn_fold(raw) {
        (sum.0.sum * 2 - sum.0.five_or_higher * 9 + sum.1.sum) % 10 == 0
    } else {
        false
    }
}

/// Non explody version of [checksum].
///
/// Takes a byte slice with Luhn check digit _absent_ and calculates it if possible.
/// For invalid inputs no digits will be calculated
/// ```rust
/// # use luhn::*;
/// // A valid ISIN
/// let microsoft = b"US5949181045";
/// assert_eq!(calc_checksum(&microsoft[..11]), Some('5'));
///
/// // Not a valid ISIN
/// let banana = b"banana";
/// assert_eq!(calc_checksum(banana), None);
///
/// // Even less valid ISIN
/// let noms = "口水鸡";
/// assert_eq!(calc_checksum(&noms.as_bytes()), None);
/// ```
pub fn calc_checksum(raw: &[u8]) -> Option<char> {
    let sum = luhn_fold(raw)?;
    let sum = (sum.1, sum.0);
    let checksum = sum.0.sum * 2 - sum.0.five_or_higher * 9 + sum.1.sum;
    let digit = (10 - (checksum % 10)) % 10;
    Some((digit as u8 + b'0') as char)
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

    #[track_caller]
    fn valid_isin(xs: &[u8]) {
        let digit = checksum(&xs[..11]);
        assert_eq!(digit, xs[11]);
        assert!(validate_slice(&xs));
    }

    #[track_caller]
    fn bad_isin(xs: &[u8]) {
        let digit = checksum(&xs[..11]);
        assert!(!validate_slice(&xs));
        assert!(digit != xs[11]);
    }

    #[test]
    fn validate_some_good_isins() {
        // I got these from <http://www.isin.org>.
        valid_isin(b"US5949181045"); // Microsoft
        valid_isin(b"US38259P5089"); // Google
        valid_isin(b"US0378331005"); // Apple
        valid_isin(b"BMG491BT1088"); // Invesco
        valid_isin(b"IE00B4BNMY34"); // Accenture
        valid_isin(b"US0231351067"); // Amazon
        valid_isin(b"US64110L1061"); // Netflix
        valid_isin(b"US30303M1027"); // Facebook
        valid_isin(b"CH0031240127"); // BMW Australia
        valid_isin(b"CA9861913023"); // Yorbeau Res

        // a set of valid isins with all possible check digits
        valid_isin(b"KR4101R60000");
        valid_isin(b"KR4201QB2551");
        valid_isin(b"KR4201RC3102");
        valid_isin(b"KR4201Q92623");
        valid_isin(b"KR4205QB2904");
        valid_isin(b"KR4301R12825");
        valid_isin(b"KR4301QC2906");
        valid_isin(b"KR4205Q92327");
        valid_isin(b"KR4301QB3228");
        valid_isin(b"KR4301Q93579");
    }

    #[test]
    fn fail_some_bad_isins() {
        bad_isin(b"US5949181040"); // Microsoft (checksum zeroed)
        bad_isin(b"US38259P5080"); // Google (checksum zeroed)
        bad_isin(b"US0378331000"); // Apple (checksum zeroed)
        bad_isin(b"BMG491BT1080"); // Invesco (checksum zeroed)
        bad_isin(b"IE00B4BNMY30"); // Accenture (checksum zeroed)
        bad_isin(b"US0231351060"); // Amazon (checksum zeroed)
        bad_isin(b"US64110L1060"); // Netflix (checksum zeroed)
        bad_isin(b"US30303M1020"); // Facebook (checksum zeroed)
        bad_isin(b"CH0031240120"); // BMW Australia (checksum zeroed)
        bad_isin(b"CA9861913020"); // Yorbeau Res (checksum zeroed)

        bad_isin(b"SU5941981045"); // Microsoft (two chars transposed)
        bad_isin(b"US3825P95089"); // Google (two chars transposed)
        bad_isin(b"US0378313005"); // Apple (two chars transposed)
        bad_isin(b"BMG491BT0188"); // Invesco (two chars transposed)
        bad_isin(b"IE00B4BNM3Y4"); // Accenture (two chars transposed)
        bad_isin(b"US2031351067"); // Amazon (two chars transposed)
        bad_isin(b"US61410L1061"); // Netflix (two chars transposed)
        bad_isin(b"US30033M1027"); // Facebook (two chars transposed)
        bad_isin(b"CH0032140127"); // BMW Australia (two chars transposed)
        bad_isin(b"CA9861193023"); // Yorbeau Res (two chars transposed)
    }
}
