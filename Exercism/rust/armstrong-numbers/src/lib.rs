pub fn is_armstrong_number(num: u32) -> bool {

    let mut total: i32 = 0;

    let num_str = num.to_string();
    let num_len = num_str.len() as u32;

    for digit in num_str.chars()
    {
        total = total + i32::pow(digit.to_digit(10).unwrap().try_into().unwrap(), num_len);
    }
    
    total == num.try_into().unwrap()   
}