pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    }

    let mut n_digits = 0;
    let mut temp_num_for_digit_count = num;
    while temp_num_for_digit_count > 0 {
        temp_num_for_digit_count /= 10;
        n_digits += 1;
    }

    let mut sum_of_powers: u64 = 0;
    let mut temp_num_for_sum_calc = num; 
    let num_as_u64 = num as u64;

    while temp_num_for_sum_calc > 0 {
        let digit = (temp_num_for_sum_calc % 10) as u32; 
        let power_of_digit = digit.pow(n_digits);     
        sum_of_powers += power_of_digit as u64;        
        
        if sum_of_powers > num_as_u64 {
            return false;
        }
        
        temp_num_for_sum_calc /= 10; 
    }
    sum_of_powers == num_as_u64
}
 