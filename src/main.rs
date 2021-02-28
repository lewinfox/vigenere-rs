use regex;
use std::fs; // For reading input files

/// Read and preprocess input
///
/// Accepts a file path or string literal and returns an uppercased `String` containing the file
/// contents. In the event that `input` is not a valid file path, returns `input` uppercased.
///
/// # Arguments
///
/// * `input`: A string containing either a file path or some input text.
fn read_input(input: &str) -> String {
    // The function will assume `input` is a file path. If an error occurs during reading this
    // probably means it's not a file, and it will use the input string as the text instead.

    // TODO: We should probably do some more error handling here - what if the file exists but it's
    // binary? Should we be returning an Option or Result or something?
    let res = match fs::read_to_string(input) {
        Ok(s) => s,
        Err(_) => {
            println!(
                "File `{}` does not exist. Treating as string literal.",
                input
            );
            String::from(input)
        }
    };
    // The strings should be alphanumeric, uppercased and contain no whitespace.
    let re = regex::Regex::new(r"[^[:alnum:]]").unwrap();
    re.replace_all(&res, "").to_uppercase()
}

fn main() {
    let in_file = "poem.txts";
    let in_text = read_input(in_file);
    let key_file = "key.txt";
    let key_text = read_input(key_file);
    println!("KEY:\n{}\nINPUT:\n{}", key_text, in_text);
}
