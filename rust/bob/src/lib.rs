pub fn reply(query: &str) -> &'static str {
    let is_question = query.trim().ends_with('?');
    let is_forceful =
        query.chars().any(char::is_uppercase) && !query.chars().any(char::is_lowercase);
    let is_silence = query.trim().is_empty();

    match (is_question, is_forceful, is_silence) {
        (true, true, _) => "Calm down, I know what I'm doing!",
        (true, false, _) => "Sure.",
        (false, true, _) => "Whoa, chill out!",
        (_, _, true) => "Fine. Be that way!",
        (_, _, _) => "Whatever.",
    }
}
