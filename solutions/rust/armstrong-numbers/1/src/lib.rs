pub fn is_armstrong_number(num: u32) -> bool {
    let exponent = if num == 0 { 1 } else { num.ilog10() + 1 };
    let mut temp = num;
    let mut acum = 0;

    while temp > 0 {
        let digit = temp % 10;
        acum += digit.pow(exponent);
        temp /= 10;
    }
    acum == num
}