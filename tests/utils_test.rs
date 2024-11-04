use alloy::primitives::U256;
use marketh_rs::utils::*;

#[test]
fn test_parse_u256_valid_decimal() {
    // Valid decimal input within range
    let value = "1234567890";
    let result = parse_u256(value).unwrap();
    assert_eq!(result, U256::from(1234567890u64));
}

#[test]
fn test_parse_u256_valid_decimal_max_value() {
    // Valid decimal input at maximum uint256 value
    let value = "115792089237316195423570985008687907853269984665640564039457584007913129639935";
    let result = parse_u256(value).unwrap();
    assert_eq!(result, U256::MAX);
}

#[test]
fn test_parse_u256_invalid_decimal_non_numeric() {
    // Invalid decimal input (non-numeric characters)
    let value = "not_a_number";
    let result = parse_u256(value);
    assert!(matches!(result, Err(ParseU256Error::InvalidU256String)));
}

#[test]
fn test_parse_u256_decimal_overflow() {
    // Decimal input exceeding maximum uint256 value
    let value = "115792089237316195423570985008687907853269984665640564039457584007913129639936"; // MAX_UINT_256 + 1
    let result = parse_u256(value);
    assert!(matches!(result, Err(ParseU256Error::InvalidU256String)));
}

#[test]
fn test_parse_u256_negative_decimal() {
    // Negative decimal input
    let value = "-1234567890";
    let result = parse_u256(value);
    // U256 cannot represent negative numbers, so parsing will fail
    assert!(matches!(result, Err(ParseU256Error::InvalidU256String)));
}

#[test]
fn test_parse_u256_valid_hex() {
    // Valid hexadecimal input within range
    let value = "0xabcdef0123456789";
    let result = parse_u256(value).unwrap();
    let expected = U256::from(12379813738877118345_u128);
    assert_eq!(result, expected);
}

#[test]
fn test_parse_u256_valid_hex_max_value() {
    // Valid hexadecimal input at maximum uint256 value
    let value = "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
    let result = parse_u256(value).unwrap();
    assert_eq!(result, U256::MAX);
}

#[test]
fn test_parse_u256_invalid_hex_non_numeric() {
    // Invalid hexadecimal input (non-hex characters)
    let value = "0xg12345";
    let result = parse_u256(value);
    assert!(matches!(result, Err(ParseU256Error::InvalidU256Hex)));
}

#[test]
fn test_parse_u256_hex_overflow() {
    // Hexadecimal input exceeding maximum uint256 value
    let value = "0x1ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
    // This is MAX_UINT_256 + 1
    let result = parse_u256(value);
    assert!(matches!(result, Err(ParseU256Error::InvalidU256Hex)));
}

#[test]
fn test_parse_u256_invalid_hex_missing_prefix() {
    // Hexadecimal input without "0x" prefix, should be parsed as decimal
    let value = "abcdef";
    let result = parse_u256(value);
    // Since "abcdef" is not a valid decimal number, it should fail
    assert!(matches!(result, Err(ParseU256Error::InvalidU256String)));
}

#[test]
fn test_parse_u256_zero_value() {
    // Zero value in decimal
    let value = "0";
    let result = parse_u256(value).unwrap();
    assert_eq!(result, U256::from(0_u8));
}

#[test]
fn test_parse_u256_hex_zero_value() {
    // Zero value in hexadecimal
    let value = "0x0";
    let result = parse_u256(value).unwrap();
    assert_eq!(result, U256::from(0_u8));
}

#[test]
fn test_parse_u256_empty_string() {
    // Empty string input
    let value = "";
    let result = parse_u256(value);
    assert!(matches!(result, Err(ParseU256Error::InvalidU256String)));
}

#[test]
fn test_parse_u256_hex_empty_string() {
    // Empty hexadecimal string after "0x"
    let value = "0x";
    let result = parse_u256(value);
    assert!(matches!(result, Err(ParseU256Error::InvalidU256Hex)));
}

#[test]
fn test_parse_u256_leading_zeros_decimal() {
    // Decimal input with leading zeros
    let value = "00001234567890";
    let result = parse_u256(value).unwrap();
    assert_eq!(result, U256::from(1234567890_u64));
}

#[test]
fn test_parse_u256_leading_zeros_hex() {
    // Hexadecimal input with leading zeros
    let value = "0x0001abcdef";
    let result = parse_u256(value).unwrap();
    let expected = U256::from(28036591_u64);
    assert_eq!(result, expected);
}
