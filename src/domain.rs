//! Pure calculation rules over exact decimals (ADR-0002, ADR-0005).
//! No HTTP or JSON types here.

use rust_decimal::Decimal;

/// Why a calculation was rejected. Mapped to HTTP errors in `http`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalcError {
    InvalidNumber,
    MissingOperand,
    UnexpectedOperand,
    DivisionByZero,
    NegativeSqrt,
    UnsupportedExponent,
    Overflow,
}

/// The operations the API offers (FR-2, FR-3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Percent,
    Sqrt,
}

impl Operation {
    /// Whether the operation takes a second operand `b`.
    pub fn is_binary(self) -> bool {
        self != Operation::Sqrt
    }
}

/// Computes `op` over `a` and, for binary operations, `b`.
/// Results are normalized (no trailing zeros, FR-4).
pub fn calculate(op: Operation, a: Decimal, b: Option<Decimal>) -> Result<Decimal, CalcError> {
    use CalcError::*;
    use Operation::*;
    let b = match (op.is_binary(), b) {
        (true, Some(b)) => b,
        (true, None) => return Err(MissingOperand),
        (false, Some(_)) => return Err(UnexpectedOperand),
        (false, None) => Decimal::ZERO,
    };
    let result = match op {
        Add => a.checked_add(b).ok_or(Overflow)?,
        Subtract => a.checked_sub(b).ok_or(Overflow)?,
        Multiply => a.checked_mul(b).ok_or(Overflow)?,
        Divide | Modulo if b.is_zero() => return Err(DivisionByZero),
        Divide => a.checked_div(b).ok_or(Overflow)?,
        Modulo => a.checked_rem(b).ok_or(Overflow)?,
        Power => power(a, b)?,
        Percent => a
            .checked_mul(b)
            .and_then(|x| x.checked_div(Decimal::ONE_HUNDRED))
            .ok_or(Overflow)?,
        Sqrt if a.is_sign_negative() && !a.is_zero() => return Err(NegativeSqrt),
        Sqrt => rust_decimal::MathematicalOps::sqrt(&a).ok_or(Overflow)?,
    };
    Ok(result.normalize())
}

/// `a ^ b` for integer `b` only (PRD section 7, decision 1).
fn power(a: Decimal, b: Decimal) -> Result<Decimal, CalcError> {
    use rust_decimal::prelude::ToPrimitive;
    if !b.fract().is_zero() {
        return Err(CalcError::UnsupportedExponent);
    }
    let exp = b.to_i64().ok_or(CalcError::Overflow)?;
    if a.is_zero() && exp < 0 {
        return Err(CalcError::DivisionByZero);
    }
    rust_decimal::MathematicalOps::checked_powi(&a, exp).ok_or(CalcError::Overflow)
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

    fn calc(op: Operation, a: &str, b: Option<&str>) -> Result<String, CalcError> {
        calculate(op, d(a), b.map(d)).map(|r| r.to_string())
    }

    fn ok(op: Operation, a: &str, b: &str) -> String {
        calc(op, a, Some(b)).unwrap()
    }

    #[test]
    fn fr2_prd_examples() {
        use Operation::*;
        assert_eq!(ok(Add, "0.1", "0.2"), "0.3");
        assert_eq!(ok(Subtract, "5", "7.5"), "-2.5");
        assert_eq!(ok(Multiply, "1.5", "4"), "6");
        assert_eq!(ok(Divide, "1", "3"), "0.3333333333333333333333333333");
        assert_eq!(ok(Modulo, "10", "3"), "1");
        assert_eq!(ok(Power, "2", "10"), "1024");
        assert_eq!(ok(Percent, "15", "200"), "30");
    }

    #[test]
    fn fr2_modulo_takes_sign_of_dividend() {
        assert_eq!(ok(Operation::Modulo, "-10", "3"), "-1");
        assert_eq!(ok(Operation::Modulo, "10", "-3"), "1");
        assert_eq!(ok(Operation::Modulo, "5.5", "2"), "1.5");
    }

    #[test]
    fn fr2_power_integer_exponents() {
        use Operation::Power;
        assert_eq!(ok(Power, "2", "-2"), "0.25");
        assert_eq!(ok(Power, "1.5", "2"), "2.25");
        assert_eq!(ok(Power, "-2", "3"), "-8");
        assert_eq!(ok(Power, "7", "0"), "1");
        assert_eq!(ok(Power, "2", "3.0"), "8");
    }

    #[test]
    fn fr2_percent_of_decimals() {
        assert_eq!(ok(Operation::Percent, "12.5", "80"), "10");
        assert_eq!(ok(Operation::Percent, "0.1", "0.2"), "0.0002");
    }

    #[test]
    fn fr3_sqrt() {
        use Operation::Sqrt;
        assert_eq!(calc(Sqrt, "16", None).unwrap(), "4");
        assert_eq!(calc(Sqrt, "0", None).unwrap(), "0");
        assert_eq!(calc(Sqrt, "2.25", None).unwrap(), "1.5");
        let root2 = calc(Sqrt, "2", None).unwrap();
        assert!(root2.starts_with("1.41421356237309504880168872"), "{root2}");
    }

    #[test]
    fn fr4_results_have_no_trailing_zeros() {
        assert_eq!(ok(Operation::Add, "2.50", "0"), "2.5");
        assert_eq!(ok(Operation::Multiply, "0.50", "2.00"), "1");
    }

    #[test]
    fn fr4_rounds_half_to_even() {
        // 1e-28 * 0.5 = 5e-29: a tie at the 28th decimal, rounds to the even 0.
        assert_eq!(
            ok(Operation::Multiply, "0.0000000000000000000000000001", "0.5"),
            "0"
        );
        // 3e-28 * 0.5 = 1.5e-28: a tie, rounds to the even 2e-28.
        assert_eq!(
            ok(Operation::Multiply, "0.0000000000000000000000000003", "0.5"),
            "0.0000000000000000000000000002"
        );
    }

    #[test]
    fn fr5_domain_errors() {
        use CalcError::*;
        use Operation::*;
        let max = "79228162514264337593543950335";
        assert_eq!(calc(Divide, "1", Some("0")), Err(DivisionByZero));
        assert_eq!(calc(Modulo, "1", Some("0.0")), Err(DivisionByZero));
        assert_eq!(calc(Power, "0", Some("-1")), Err(DivisionByZero));
        assert_eq!(calc(Sqrt, "-4", None), Err(NegativeSqrt));
        assert_eq!(calc(Power, "2", Some("0.5")), Err(UnsupportedExponent));
        assert_eq!(calc(Add, max, Some("1")), Err(Overflow));
        assert_eq!(calc(Multiply, max, Some("2")), Err(Overflow));
        assert_eq!(calc(Power, "10", Some("29")), Err(Overflow));
        assert_eq!(calc(Power, "2", Some(max)), Err(Overflow));
        assert_eq!(calc(Percent, max, Some("200")), Err(Overflow));
        assert_eq!(calc(Divide, max, Some("0.1")), Err(Overflow));
    }

    #[test]
    fn fr3_operand_count_is_checked() {
        assert_eq!(
            calc(Operation::Add, "1", None),
            Err(CalcError::MissingOperand)
        );
        assert_eq!(
            calc(Operation::Sqrt, "4", Some("1")),
            Err(CalcError::UnexpectedOperand)
        );
    }
}
