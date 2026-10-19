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

    let mut new_string = _text;
    new_string.push_str(" rust");
    new_string.push('\n');

    let mut result = String::new();
    result.push_str(&new_string[start..end]);
    result.push('\n');

    let mut_ref_string = &mut new_string;
    result.push_str(mut_ref_string);

    let ref_string = &new_string;
    result.push_str(ref_string);

    return result;
}

#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;
