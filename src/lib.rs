/// Represents the canonical Logic units in a truth table for boolean logic.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LogicValue {
    False,
    True,
}


/// NAND Gate function (not-and). Takes two logic values
/// and outputs a value based on the following table.
/// 
/// | a | b | out |
/// |---|---|-----|
/// | 0 | 0 | 1   |
/// | 0 | 1 | 1   |
/// | 1 | 0 | 1   |
/// | 1 | 1 | 0   |
///
/// ```
/// # use nand2tetris_rust::{nand, LogicValue};
/// assert_eq!(nand(LogicValue::False, LogicValue::False), LogicValue::True);
/// assert_eq!(nand(LogicValue::False, LogicValue::True), LogicValue::True);
/// assert_eq!(nand(LogicValue::True, LogicValue::False), LogicValue::True);
/// assert_eq!(nand(LogicValue::True, LogicValue::True), LogicValue::False);
/// ```
pub fn nand(a: LogicValue, b: LogicValue) -> LogicValue {
    match (a, b) {
        (LogicValue::True, LogicValue::True) => LogicValue::False,
        (LogicValue::True, LogicValue::False) => LogicValue::True,
        (LogicValue::False, LogicValue::True) => LogicValue::True,
        (LogicValue::False, LogicValue::False) => LogicValue::True,
    }
}

/// Not Gate function. Takes one logic value
/// and outputs a value based on the following table.
/// 
/// | a | out |
/// |---|-----|
/// | 0 | 1   |
/// | 1 | 0   |
///
/// ```
/// # use nand2tetris_rust::{not, LogicValue};
/// assert_eq!(not(LogicValue::False), LogicValue::True);
/// assert_eq!(not(LogicValue::True), LogicValue::False);
/// ```
pub fn not(a: LogicValue) -> LogicValue {
    nand(a, a)
}

/// AND Gate function. Takes two logic values
/// and outputs a value based on the following table.
/// 
/// | a | b | out |
/// |---|---|-----|
/// | 0 | 0 | 0   |
/// | 0 | 1 | 0   |
/// | 1 | 0 | 0   |
/// | 1 | 1 | 1   |
///
/// ```
/// # use nand2tetris_rust::{and, LogicValue};
/// assert_eq!(and(LogicValue::False, LogicValue::False), LogicValue::False);
/// assert_eq!(and(LogicValue::False, LogicValue::True), LogicValue::False);
/// assert_eq!(and(LogicValue::True, LogicValue::False), LogicValue::False);
/// assert_eq!(and(LogicValue::True, LogicValue::True), LogicValue::True);
/// ```
pub fn and(a: LogicValue, b: LogicValue) -> LogicValue {
    let nand_result = nand(a, b);
    not(nand_result)
}


/// OR Gate function. Takes two logic values
/// and outputs a value based on the following table.
/// 
/// | a | b | out |
/// |---|---|-----|
/// | 0 | 0 | 0   |
/// | 0 | 1 | 1   |
/// | 1 | 0 | 1   |
/// | 1 | 1 | 1   |
///
/// ```
/// # use nand2tetris_rust::{or, LogicValue};
/// assert_eq!(or(LogicValue::False, LogicValue::False), LogicValue::False);
/// assert_eq!(or(LogicValue::False, LogicValue::True), LogicValue::True);
/// assert_eq!(or(LogicValue::True, LogicValue::False), LogicValue::True);
/// assert_eq!(or(LogicValue::True, LogicValue::True), LogicValue::True);
/// ```
pub fn or(a: LogicValue, b: LogicValue) -> LogicValue {
    let nand_result_1 = nand(a, a);
    let nand_result_2 = nand(b, b);
    nand(nand_result_1, nand_result_2)
}

/// XOR Gate function (Exclusive OR). Takes two logic values
/// and outputs a value based on the following table.
/// 
/// | a | b | out |
/// |---|---|-----|
/// | 0 | 0 | 0   |
/// | 0 | 1 | 1   |
/// | 1 | 0 | 1   |
/// | 1 | 1 | 0   |
///
/// ```
/// # use nand2tetris_rust::{xor, LogicValue};
/// assert_eq!(xor(LogicValue::False, LogicValue::False), LogicValue::False);
/// assert_eq!(xor(LogicValue::False, LogicValue::True), LogicValue::True);
/// assert_eq!(xor(LogicValue::True, LogicValue::False), LogicValue::True);
/// assert_eq!(xor(LogicValue::True, LogicValue::True), LogicValue::False);
/// ```
pub fn xor(a: LogicValue, b: LogicValue) -> LogicValue {
    let not_a = not(a);
    let not_b = not(b);
    let nand_result_1 = nand(a, not_b);
    let nand_result_2 = nand(not_a, b);
    nand(nand_result_1, nand_result_2)

}
