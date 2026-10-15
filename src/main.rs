fn main() {
    print!("{}", process(String::from("hello MATMEX")));
}

fn process(_text: String) -> String {
    let (start, end) = match _text.split_whitespace().next() {
        Some(word) => {
            let start = match _text.find(word) {
                Some(start) => start,
                None => 0,
            };

            (start, start + word.len())
        }
        None => (0, 0),
    };

    let mut new_string = String::new();
    new_string.push_str(&_text[start..end]);
    new_string.push('\n');
    let mut _text_copy = _text;
    _text_copy.push_str(" rust");
    new_string.push_str(_text_copy.as_str());
    new_string.push('\n');
    new_string.push_str(_text_copy.as_str());
    new_string.push('\n');

    return new_string;
}

#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;
