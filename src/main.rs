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

/// Convert a character into a numeric position in the alphabet
fn char_to_num(c: char) -> usize {
    LETTERS.iter().position(|&x| x == c).unwrap() + 1
}

#[test]
fn test_char_to_num() {
    assert_eq!(char_to_num('A'), 1);
    assert_eq!(char_to_num('Z'), 26);
}

/// Convert a number into a letter (A == 1)
fn num_to_char(n: usize) -> char {
    LETTERS[n - 1]
}

#[test]
fn test_num_to_char() {
    assert_eq!(num_to_char(1), 'A');
    assert_eq!(num_to_char(26), 'Z');
}
/// Convert a string into a vector of characters
fn string_to_vec(s: &String) -> Vec<char> {
    s.chars().collect::<Vec<char>>()
}

#[test]
fn test_string_to_vec() {
    let hello = String::from("HELLO");
    assert_eq!(string_to_vec(&hello), vec!['H', 'E', 'L', 'L', 'O']);
}

// Convert a string into a vector of numbers
fn string_to_nums(s: &String) -> Vec<usize> {
    string_to_vec(s)
        .into_iter()
        .map(|x| char_to_num(x))
        .collect::<Vec<usize>>()
}

#[test]
fn test_string_to_nums() {
    let s = String::from("AZ");
    let v: Vec<usize> = vec![1, 26];
    assert_eq!(string_to_nums(&s), v);
}

/// Convert a vector of numbers into a string
fn nums_to_string(v: &Vec<usize>) -> String {
    v.into_iter().map(|&x| num_to_char(x)).collect::<String>()
}

#[test]
fn test_nums_to_string() {
    let n: Vec<usize> = vec![1, 26];
    let s = String::from("AZ");
    assert_eq!(nums_to_string(&n), s);
}

/// Add two equal-length vectors together modulo (something)
fn modulo_add(a: &Vec<usize>, b: &Vec<usize>, modulus: usize) -> Vec<usize> {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| {
            let ans = (x + y) % modulus as usize;
            match ans {
                0 => 26,
                _ => ans,
            }
        })
        .collect::<Vec<usize>>()
}

#[test]
fn test_modulo_add() {
    let x: Vec<usize> = vec![25, 26, 1];
    let y: Vec<usize> = vec![1, 1, 1];
    assert_eq!(modulo_add(&x, &y, 26), vec![26, 1, 2]);
}

/// Subtract `b` from `a` modulo `modulus`
fn modulo_subtract(a: &Vec<usize>, b: &Vec<usize>, modulus: i8) -> Vec<usize> {
    // Need to convert to signed integers to handle negative numbers
    let int_a: Vec<i8> = a.into_iter().map(|&i| i as i8).collect();
    let int_b: Vec<i8> = b.into_iter().map(|&i| i as i8).collect();
    int_a
        .iter()
        .zip(int_b.iter())
        .map(|(x, y)| {
            let ans = ((x + modulus - y) % modulus) as usize;
            match ans {
                0 => 26,
                _ => ans,
            }
        })
        .collect()
}

#[test]
fn test_modulo_subtract() {
    let x: Vec<usize> = vec![1, 2];
    let y: Vec<usize> = vec![2, 2];
    assert_eq!(modulo_subtract(&x, &y, 26), vec![25, 26]);
}

fn match_key_length(key: &String, length: usize) -> String {
    let l = key.len();
    let n_repeats = (length / l) + 1;
    let mut new_key = key.repeat(n_repeats);
    new_key.truncate(length);
    new_key
}

#[test]
fn test_match_key_length() {
    assert_eq!(
        match_key_length(&String::from("KEY"), 8),
        String::from("KEYKEYKE")
    );
    assert_eq!(match_key_length(&String::from("KEY"), 0), String::from(""));
}

fn encode(input: &String, key: &String) -> String {
    let new_key = match_key_length(key, input.len());
    let input_nums = string_to_nums(&input);
    let key_nums = string_to_nums(&new_key);
    let out_nums = modulo_add(&input_nums, &key_nums, 26);
    nums_to_string(&out_nums)
}

#[test]
fn test_encode() {
    let s = String::from("YZAB");
    let k = String::from("A");
    assert_eq!(encode(&s, &k), String::from("ZABC"));
}

fn decode(input: &String, key: &String) -> String {
    let new_key = match_key_length(key, input.len());
    let input_nums = string_to_nums(&input);
    let key_nums = string_to_nums(&new_key);
    let out_nums = modulo_subtract(&input_nums, &key_nums, 26);
    nums_to_string(&out_nums)
}

#[test]
fn test_decode() {
    let text = String::from("ZABC");
    let key = String::from("A");
    assert_eq!(decode(&text, &key), String::from("YZAB"));
}

#[test]
fn test_encode_decode() {
    let text = String::from("HELLOWORLD");
    let key = String::from("KEY");
    assert_eq!(decode(&encode(&text, &key), &key), text);
}

fn main() {
    let text_file = "poem.txt";
    let key_file = "key.txt";
    let text = read_input(&text_file);
    let key = read_input(&key_file);
    let encoded = encode(&text, &key);
    let decoded = decode(&encoded, &key);
    println!("{:?}", text);
    println!("{:?}", encoded);
    println!("{:?}", decoded);
}
