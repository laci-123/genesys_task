use std::io;

fn main() -> io::Result<()>{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let x = solution(&input);

    println!("{:?}", x);

    Ok(())
}

fn solution(digits: &str) -> Vec<String>{
    let first_digit   = digits.chars().nth(0);
    let first_letters = digit_to_letters(first_digit);

    let mut result = vec![];
    for letter in first_letters.chars(){
        result.append(&mut concat_every_letter(letter, solution(&digits[1..]))); 
    }

    result
}

fn concat_every_letter(first: char, letters: Vec<String>) -> Vec<String>{
    if letters.len() == 0{
        vec![first.to_string()]
    }
    else{
        letters.iter()
               .map(|letter| format!("{}{}", first, letter))
               .collect()
    }
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
