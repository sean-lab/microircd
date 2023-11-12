
pub fn parse_message(input: &str) -> Option<(&str, &str, Vec<&str>)> {
    let mut parts = input.split(' ');
    let prefix = if let Some(part) = parts.next() {
        if part.starts_with(':') {
            Some(&part[1..])
        } else {
            None
        }
    } else {
        return None;
    };
    let command = parts.next()?;
    let mut params = Vec::new();
    while let Some(part) = parts.next() {
        if part.starts_with(':') {
            params.push(&part[1..]);
            break;
        } else {
            params.push(part);
        }
    }
    Some((prefix.unwrap_or(""), command, params))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_message() {
        let input = ":prefix command param1 param2 :param with spaces";
        let result = parse_message(input);
        assert_eq!(result, Some(("prefix", "command", vec!["param1", "param2", "param with spaces"])));
    }
}