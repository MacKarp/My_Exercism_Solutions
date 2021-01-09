pub fn reply(message: &str) -> &str {
    let yell = message.contains(char::is_uppercase) && !message.contains(char::is_lowercase);

    match (message.trim_end(), yell) {
        ("", false) => "Fine. Be that way!",
        (m, false) if m.ends_with('?') => "Sure.",
        (m, true) if m.ends_with('?') => "Calm down, I know what I'm doing!",
        (_, true) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
