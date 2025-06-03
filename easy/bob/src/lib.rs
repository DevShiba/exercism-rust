pub fn reply(message: &str) -> &str {
    let trimmed_message = message.trim();

    if trimmed_message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = message.trim_end().ends_with('?');

    let has_alphabetic_chars = trimmed_message.chars().any(|c| c.is_alphabetic());
    let is_all_caps_if_letters_exist = trimmed_message
        .chars()
        .filter(|c| c.is_alphabetic()) 
        .all(|c| c.is_uppercase());   
    
    let is_yelling = has_alphabetic_chars && is_all_caps_if_letters_exist;

    if is_yelling && is_question {
        "Calm down, I know what I'm doing!"
    } else if is_yelling {
        "Whoa, chill out!"
    } else if is_question {
        "Sure."
    } else {
        "Whatever."
    }
}