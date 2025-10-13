fn small_number_name(n: u8) -> &'static str {
    match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => "", 
    }
}

fn tens_name(n: u8) -> &'static str {
    match n {
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => "", 
    }
}

fn encode_under_100(n: u8) -> String {
    if n < 20 {
        small_number_name(n).to_string()
    } else {
        let tens = n / 10;
        let ones = n % 10;
        if ones == 0 {
            tens_name(tens).to_string()
        } else {
            format!("{}-{}", tens_name(tens), small_number_name(ones))
        }
    }
}

fn encode_under_1000(n: u16) -> String {
    if n < 100 {
        encode_under_100(n as u8)
    } else {
        let hundreds = n / 100;
        let remainder = n % 100;
        if remainder == 0 {
            format!("{} hundred", small_number_name(hundreds as u8))
        } else {
            format!(
                "{} hundred {}",
                small_number_name(hundreds as u8),
                encode_under_100(remainder as u8)
            )
        }
    }
}

pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let chunks = [
        (1_000_000_000_000_000_000, "quintillion"),
        (1_000_000_000_000_000, "quadrillion"),
        (1_000_000_000_000, "trillion"),
        (1_000_000_000, "billion"),
        (1_000_000, "million"),
        (1_000, "thousand"),
    ];

    let mut num = n;
    let mut parts = Vec::new();

    for &(divisor, name) in &chunks {
        if num >= divisor {
            let count = num / divisor;
            parts.push(format!("{} {}", encode_under_1000(count as u16), name));
            num %= divisor;
        }
    }

    if num > 0 {
        parts.push(encode_under_1000(num as u16));
    }

    parts.join(" ")
}