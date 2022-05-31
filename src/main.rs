#![allow(dead_code)]


fn main() {
    
    println!("Hello, world!");
}

fn add_first_letter(first: char, letters: Vec<String>) -> Vec<String>{
    letters.iter()
           .map(|letter| format!("{}{}", first, letter))
           .collect()
}

fn digit_to_letters(digit: Option<char>) -> &'static str{
    match digit{
        Some('2') => "abc", 
        Some('3') => "def", 
        Some('4') => "ghi", 
        Some('5') => "jkl", 
        Some('6') => "mno", 
        Some('7') => "pqrs", 
        Some('8') => "tuv", 
        Some('9') => "wxyz", 
        Some(_)   => "",
        None      => "", 
    }
}
