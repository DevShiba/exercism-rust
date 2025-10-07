pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let mut chars = phrase.chars().peekable();
    let mut prev_is_letter = false;
    let mut prev_is_uppercase = false;
    
    while let Some(ch) = chars.next() {
        if ch.is_alphabetic() {
            let is_uppercase = ch.is_uppercase();
            
            if !prev_is_letter {
                acronym.push(ch.to_ascii_uppercase());
            } else if !prev_is_uppercase && is_uppercase {
                acronym.push(ch);
            }
            
            prev_is_letter = true;
            prev_is_uppercase = is_uppercase;
        } else if ch == '-' || ch == '_' || ch.is_whitespace() {
            prev_is_letter = false;
            prev_is_uppercase = false;
        }
    }
    
    acronym
}
