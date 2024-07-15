#[derive(Clone)]
struct BigNumber{
    big_number: Vec<u64>,
}

impl BigNumber {

    pub fn new() -> Self {
        BigNumber { big_number: Vec::new() }
    }

    pub fn set_hex(hex_str: &str) -> Self{
        let mut bignumber = BigNumber::new();
        let mut hex_str = hex_str.to_string();

        if hex_str.len() % 16 != 0 {
            let pad_len = 16 - (hex_str.len() % 16);
            hex_str = "0".repeat(pad_len) + &hex_str;
        }

        for chunk in hex_str.as_bytes().rchunks(16) {
            let chunk_str = std::str::from_utf8(chunk).unwrap();
            let value = u64::from_str_radix(chunk_str, 16).unwrap();
            bignumber.big_number.push(value);
        }
        bignumber.big_number.reverse();
        bignumber

    }

    pub fn get_hex(&self) -> String {
        let mut hex_string = String::new();

        for &value in self.big_number.iter() {
            hex_string.push_str(&format!("{:016x}", value));
        }

        while hex_string.starts_with('0') {
            hex_string.remove(0);
        }

        if hex_string.is_empty() {
            hex_string = "0".to_string();
        }

        hex_string

    }

    pub fn xor(n1: &BigNumber, n2: &BigNumber) -> BigNumber {
        let mut result = BigNumber::new();
        let max_len = std::cmp::max(n1.big_number.len(), n2.big_number.len());
        for i in 0..max_len{
            result.big_number.push(*n1.big_number.get(i).unwrap_or(&0) ^ *n2.big_number.get(i).unwrap_or(&0));
        }

        result
    }

    pub fn and(n1: &BigNumber, n2: &BigNumber) -> BigNumber {
        let mut result = BigNumber::new();
        let max_len = std::cmp::max(n1.big_number.len(), n2.big_number.len());
        for i in 0..max_len{
            result.big_number.push(*n1.big_number.get(i).unwrap_or(&0) & *n2.big_number.get(i).unwrap_or(&0));
        }

        result
    }

    pub fn or(n1: &BigNumber, n2: &BigNumber) -> BigNumber {
        let mut result = BigNumber::new();
        let max_len = std::cmp::max(n1.big_number.len(), n2.big_number.len());
        for i in 0..max_len{
            result.big_number.push(*n1.big_number.get(i).unwrap_or(&0) | *n2.big_number.get(i).unwrap_or(&0));
        }

        result
    }

    pub fn inv(&self) -> BigNumber {
        let mut result = BigNumber::new();

        for &value in &self.big_number {
            result.big_number.push(!value);
        }

        result
    }


    pub fn add(n1: &BigNumber, n2: &BigNumber) -> BigNumber {
        let mut result = BigNumber::new();
        let max_len = std::cmp::max(n1.big_number.len(), n2.big_number.len());
        let mut carry = 0;

        for i in 0..max_len {
            let a = *n1.big_number.get(i).unwrap_or(&0);
            let b = *n2.big_number.get(i).unwrap_or(&0);

            let (sum, carry1) = a.overflowing_add(b);
            let (sum, carry2) = sum.overflowing_add(carry);

            result.big_number.push(sum);
            carry = (carry1 as u64) + (carry2 as u64);
        }

        if carry > 0 {
            result.big_number.push(carry);
        }

        result
    }

    pub fn sub(n1: &BigNumber, n2: &BigNumber) -> BigNumber {
        let mut result = BigNumber::new();
        let max_len = std::cmp::max(n1.big_number.len(), n2.big_number.len());
        let mut borrow = 0;

        for i in 0..max_len {
            let a = *n1.big_number.get(i).unwrap_or(&0);
            let b = *n2.big_number.get(i).unwrap_or(&0);

            if a >= b + borrow {
                result.big_number.push(a - b - borrow);
                borrow = 0;
            } else {
                result.big_number.push(u64::MAX - b + a - borrow + 1);
                borrow = 1;
            }
        }

        // Видалення провідних нулів
        while result.big_number.last() == Some(&0) {
            result.big_number.pop();
        }

        if result.big_number.is_empty() {
            result.big_number.push(0);
        }

        result
    }

    fn cmd(n1: &BigNumber, n2: &BigNumber) -> bool {
        if n1.big_number.len() != n2.big_number.len() {
            return n1.big_number.len() > n2.big_number.len();
        }
        for i in 0..n1.big_number.len() {
            if n1.big_number[i] != n2.big_number[i] {
                return n1.big_number[i] > n2.big_number[i];
            }
        }
        false
    }

    pub fn module(number: &BigNumber, modulus: &BigNumber) -> BigNumber {
        let mut result = number.clone();

        while BigNumber::cmd(&result, modulus) {
            result = BigNumber::sub(&result, modulus);
        }

        result
    }

}



fn main() {

    let number1 = BigNumber::set_hex("36f028580bb02cc8272a9a020f4200e346e276ae664e45ee80745574e2f5ab80");

    let number2 = BigNumber::set_hex("70983d692f648185febe6d6fa607630ae68649f7e6fc45b94680096c06e4fadb");

    println!("{:?},  {:?}", number1.big_number, number2.big_number);
    println!("{}, {}", number1.get_hex(), number2.get_hex());


    let number_c_xor = BigNumber::xor(&number1, &number2);
    println!("XOR: {}", number_c_xor.get_hex());

    let number_c_add = BigNumber::add(&number1, &number2);
    println!("ADD: {}", number_c_add.get_hex());

    let number_c_sub = BigNumber::sub(&number1, &number2);
    println!("SUB: {}", number_c_sub.get_hex());

    let number_c_mod = BigNumber::module(&number1, &number2);
    println!("MOD: {}", number_c_mod.get_hex());






}
