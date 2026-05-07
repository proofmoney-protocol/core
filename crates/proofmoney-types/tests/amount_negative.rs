use proofmoney_types::Amount;

#[test]
fn rejects_malformed_decimal_amounts() {
    assert!(Amount::parse_prm_decimal("").is_err());
    assert!(Amount::parse_prm_decimal("abc").is_err());
    assert!(Amount::parse_prm_decimal("1.2.3").is_err());
    assert!(Amount::parse_prm_decimal("-1").is_err());
    assert!(Amount::parse_prm_decimal("1.123456789").is_err());
}

#[test]
fn parses_smallest_unit_without_float_math() {
    let amount = Amount::parse_prm_decimal("0.00000001").expect("smallest unit should parse");
    assert_eq!(amount.0, 1);
    assert_eq!(amount.to_prm_string(), "0.00000001");
}

#[test]
fn parses_whole_and_fractional_amounts() {
    let amount = Amount::parse_prm_decimal("12.34567890").expect("decimal should parse");
    assert_eq!(amount.to_prm_string(), "12.34567890");
}
