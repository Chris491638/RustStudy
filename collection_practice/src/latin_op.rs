pub fn info(text: &str) {
    println!("{}",pigify(text));
}

fn pigify_one(word: &str) -> String {
    let mut chars = word.chars();

    let first_char = match chars.next() {
        Some(c) => c,
        None => return String::new(),
    };

    match first_char {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
        _ => format!("{}-{}ay", chars.as_str(), first_char),
    }
}

fn pigify(text: &str) -> String {
    // map 用法跟 ruby 中类似
    // fold 相当于 ruby 中的 inject
    text.split_whitespace()
    .map(pigify_one)
    .fold(String::new(),folder)
}

fn folder(mut current: String, next: String) -> String {
       if !current.is_empty() {
           current.push(' ');
       }
       current.push_str(&next);
       current
}

fn pigify_bk(text: &str) -> String {
    let mut rs = String::new();
    for v in text.split_whitespace(){
        let vv = pigify_one(v);
        rs.push_str(&vv[..]);
        rs.push_str(" ")
    }
    rs
}