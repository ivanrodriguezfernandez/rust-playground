pub fn is_armstrong_number(num: u32) -> bool {

    let mut total: i32 = 0;

    for digit in num.to_string().chars(){

        total = total + i32::pow(digit.to_digit(10).unwrap().try_into().unwrap(), num.to_string().chars().count().try_into().unwrap());
    }
    
    total == num.try_into().unwrap()   
}