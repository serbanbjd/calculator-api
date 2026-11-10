//! Pure calculation rules over exact decimals (ADR-0002, ADR-0005).
//! No HTTP or JSON types here.

use rust_decimal::Decimal;

/// Why a calculation was rejected. Mapped to HTTP errors in `http`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalcError {
    InvalidNumber,
}

/// Parses a plain decimal string: optional `-`, digits, optional `.digits`.
/// Rejects scientific notation, whitespace, signs other than a leading `-`,
/// and values that cannot be represented exactly (FR-4, PRD section 7).
pub fn parse_number(s: &str) -> Result<Decimal, CalcError> {
    let digits = s.strip_prefix('-').unwrap_or(s);
    let (int, frac) = match digits.split_once('.') {
        Some((i, f)) => (i, Some(f)),
        None => (digits, None),
    };
    let all_digits = |p: &str| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit());
    if !all_digits(int) || frac.is_some_and(|f| !all_digits(f)) {
        return Err(CalcError::InvalidNumber);
    }
    Decimal::from_str_exact(s).map_err(|_| CalcError::InvalidNumber)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(s: &str) -> Decimal {
        s.parse().unwrap()
    }

    #[test]
    fn fr1_parses_plain_decimals() {
        for (input, expected) in [
            ("0.1", "0.1"),
            ("-42", "-42"),
            ("007.50", "7.50"),
            ("0", "0"),
        ] {
            assert_eq!(parse_number(input), Ok(d(expected)), "{input}");
        }
    }

    #[test]
    fn fr5_rejects_non_plain_numbers() {
        for input in [
            "", " ", "1e3", "1E3", "+1", "--1", ".5", "5.", "1.2.3", " 1", "1 ", "abc", "0x10",
            "NaN", "inf", "1_000", "١",
        ] {
            assert_eq!(
                parse_number(input),
                Err(CalcError::InvalidNumber),
                "{input:?}"
            );
        }
    }

    #[test]
    fn fr4_rejects_numbers_that_would_lose_precision() {
        // 29 fractional digits cannot be represented exactly.
        assert_eq!(
            parse_number("0.00000000000000000000000000001"),
            Err(CalcError::InvalidNumber)
        );
        // Larger than the 96-bit mantissa.
        assert_eq!(
            parse_number("99999999999999999999999999999999"),
            Err(CalcError::InvalidNumber)
        );
    }

    #[test]
    fn fr4_accepts_max_precision() {
        assert!(parse_number("0.1234567890123456789012345678").is_ok());
        assert!(parse_number("79228162514264337593543950335").is_ok());
    }
}
