pub fn abbreviate(words: Vec<String>) -> String {
    let mut ans = String::new();
    for word in &words {
        let first_char = word.chars().next().unwrap();
        ans.push_str(first_char.to_string().to_uppercase().as_str());
    };
    ans
}
