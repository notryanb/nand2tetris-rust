#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LogicValue {
    False,
    True,
}


// NAND Gate
pub fn nand(a: LogicValue, b: LogicValue) -> LogicValue {
    match (a, b) {
        (LogicValue::True, LogicValue::True) => LogicValue::False,
        (LogicValue::True, LogicValue::False) => LogicValue::True,
        (LogicValue::False, LogicValue::True) => LogicValue::True,
        (LogicValue::False, LogicValue::False) => LogicValue::True,
    }
}

pub fn not(a: LogicValue) -> LogicValue {
    nand(a, a)
}

pub fn and(a: LogicValue, b: LogicValue) -> LogicValue {
    let nand_result = nand(a, b);
    not(nand_result)
}

pub fn or(a: LogicValue, b: LogicValue) -> LogicValue {
    let nand_result_1 = nand(a, a);
    let nand_result_2 = nand(b, b);
    nand(nand_result_1, nand_result_2)
}

pub fn xor(a: LogicValue, b: LogicValue) -> LogicValue {
    let not_a = not(a);
    let not_b = not(b);
    let nand_result_1 = nand(a, not_b);
    let nand_result_2 = nand(not_a, b);
    nand(nand_result_1, nand_result_2)

}

#[cfg(test)]
mod tests {
    use super::*;
   
        #[test]
    fn nand_test() {
        assert_eq!(nand(LogicValue::True, LogicValue::True), LogicValue::False);
        assert_eq!(nand(LogicValue::False, LogicValue::True), LogicValue::True);
        assert_eq!(nand(LogicValue::True, LogicValue::False), LogicValue::True);
        assert_eq!(nand(LogicValue::False, LogicValue::False), LogicValue::True);
    }
    
    #[test]
    fn not_test() {
        assert_eq!(not(LogicValue::True), LogicValue::False);
        assert_eq!(not(LogicValue::False), LogicValue::True);
    }
    
    #[test]
    fn and_test() {
        assert_eq!(and(LogicValue::True, LogicValue::True), LogicValue::True);
        assert_eq!(and(LogicValue::False, LogicValue::True), LogicValue::False);
        assert_eq!(and(LogicValue::True, LogicValue::False), LogicValue::False);
        assert_eq!(and(LogicValue::False, LogicValue::False), LogicValue::False);
    }
    
    #[test]
    fn or_test() {
        assert_eq!(or(LogicValue::True, LogicValue::True), LogicValue::True);
        assert_eq!(or(LogicValue::False, LogicValue::True), LogicValue::True);
        assert_eq!(or(LogicValue::True, LogicValue::False), LogicValue::True);
        assert_eq!(or(LogicValue::False, LogicValue::False), LogicValue::False);
    }
    
    #[test]
    fn xor_test() {
        assert_eq!(xor(LogicValue::True, LogicValue::True), LogicValue::False);
        assert_eq!(xor(LogicValue::False, LogicValue::True), LogicValue::True);
        assert_eq!(xor(LogicValue::True, LogicValue::False), LogicValue::True);
        assert_eq!(xor(LogicValue::False, LogicValue::False), LogicValue::False);
    }
}
