

pub fn number_to_letters(mut num: u32) -> String {
    if num == 0 {
        return String::from("A");
    }
    
    let mut result = String::new();
    
    while num > 0 {
        let remainder = (num - 1) % 26;
        result = char::from(b'A' + remainder as u8).to_string() + &result;
        num = (num - 1) / 26;
    }
    
    result
}

pub fn normalize_monument_name(name: String) -> String {
    let regex = regex::Regex::new(r"([A-Z])").unwrap();

    let name = name.replace("display_name", "")
        .replace("monument_name", "")
        .replace("monument", "");
        
    regex.replace_all(name.as_str(), "_$1")
        .replace("_", " ")
        .to_uppercase()
}