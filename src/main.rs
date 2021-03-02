use regex;
use std::fs; // For reading input files

const LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

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

fn char_to_num(c: char) -> usize {
    LETTERS.iter().position(|&x| x == c).unwrap()
}

fn num_to_char(n: usize) -> char {
    LETTERS[n]
}

fn string_to_vec(s: String) -> Vec<char> {
    s.chars().collect::<Vec<char>>()
}

fn vec_to_string(v: Vec<char>) -> String {
    v.iter().collect()
}

fn string_to_nums(s: String) -> Vec<usize> {
    string_to_vec(s)
        .into_iter()
        .map(|x| char_to_num(x))
        .collect::<Vec<usize>>()
}

fn nums_to_string(v: Vec<usize>) -> String {
    v.into_iter().map(|x| num_to_char(x)).collect::<String>()
}

fn main() {
    // let in_file = "poem.txt";
    // let in_text = read_input(in_file);
    // println!("Key:\n{}", in_text);
    // let in_nums = string_to_nums(in_text);
    // println!("Key nums:\n{:?}", in_nums);
    let key_file = "key.txt";
    let key_text = read_input(key_file);
    println!("Key:\n{}", key_text);
    let key_nums = string_to_nums(key_text);
    println!("Key nums:\n{:?}", key_nums);
}
