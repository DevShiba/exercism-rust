pub fn encode(source: &str) -> String {
    let mut encoded = String::new();
    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        let mut count = 1;
        while chars.peek() == Some(&c) {
            chars.next();
            count += 1;
        }

        if count > 1 {
            encoded.push_str(&count.to_string());
        }
        encoded.push(c);
    }

    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut count_str = String::new();

    for ch in source.chars() {
        if ch.is_ascii_digit() {
            count_str.push(ch);
        } else {
            let count = count_str.parse::<usize>().unwrap_or(1);
            decoded.push_str(&ch.to_string().repeat(count));
            count_str.clear();
        }
    }

    decoded
}