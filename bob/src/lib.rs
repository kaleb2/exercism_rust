fn is_question(message: &str) -> bool {
    message.trim().ends_with("?")
}

fn is_yelling(message: &str) -> bool {
    if message.chars().filter(|c: &char| c.is_alphabetic()).count() == 0 {
        return false;
    }
    message
        .chars()
        .filter(|c: &char| c.is_alphabetic())
        .all(|c: char| c.is_uppercase())
}

fn is_whitespace(message: &str) -> bool {
    message.chars().all(|c: char| c.is_whitespace())
}

pub fn reply(message: &str) -> &str {
    if is_whitespace(message) {
        return "Fine. Be that way!";
    } else if is_question(message) && is_yelling(message) {
        return "Calm down, I know what I'm doing!";
    } else if is_question(message) {
        return "Sure.";
    } else if is_yelling(message) {
        return "Whoa, chill out!";
    }

    "Whatever."
}
