pub fn brackets_are_balanced(string: &str) -> bool {
    let mut bracket_stack: Vec<char> = Vec::new();
    for c in string.chars() {
        if c == '(' || c == '{' || c == '[' {
            bracket_stack.push(c);
        } else if c == ')' {
            match bracket_stack.pop() {
                Some(b) => match b {
                    '(' => continue,
                    '{' => return false,
                    '[' => return false,
                    _ => continue,
                },
                None => return false,
            }
        } else if c == '}' {
            match bracket_stack.pop() {
                Some(b) => match b {
                    '(' => return false,
                    '{' => continue,
                    '[' => return false,
                    _ => continue,
                },
                None => return false,
            }
        } else if c == ']' {
            match bracket_stack.pop() {
                Some(b) => match b {
                    '(' => return false,
                    '{' => return false,
                    '[' => continue,
                    _ => continue,
                },
                None => return false,
            }
        }
    }

    if !bracket_stack.is_empty() {
        return false;
    }

    true
}
