use std::io;

fn main() -> io::Result<()>{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let x = solution(&input);

    println!("{:?}", x);

    Ok(())
}

fn solution(digits: &str) -> Vec<String>{
    let maybe_first_digit = digits.chars().nth(0); // None, if digits is empty
    let first_letters     = digit_to_letters(maybe_first_digit);

    let mut result = vec![];
    for letter in first_letters.chars(){
        result.append(&mut concat_every_string(letter, solution(&digits[1..]))); 
    }

    result
}

fn concat_every_string(first: char, strings: Vec<String>) -> Vec<String>{
    if strings.len() == 0{
        vec![first.to_string()]
    }
    else{
        strings.iter()
               .map(|string| format!("{}{}", first, string))
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
        Some(_)   => "",     // invalid input (e.g. '\n' read from stdin)
        None      => "",     // end of input
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn concat_every_string_empty_vec(){
        let x = concat_every_string('a', vec![]);
        assert_eq!(x, vec!["a"]);
    }

    #[test]
    fn concat_every_string_empty_string(){
        let x = concat_every_string('b', vec!["".to_string()]);
        assert_eq!(x, vec!["b"]);
    }

    #[test]
    fn concat_every_string_empty_strings(){
        let x = concat_every_string('b', vec!["".to_string(), "".to_string(), "".to_string()]);
        assert_eq!(x, vec!["b", "b", "b"]);
    }

    #[test]
    fn concat_every_string_one(){
        let x = concat_every_string('c', vec!["abc".to_string()]);
        assert_eq!(x, vec!["cabc"]);
    }
    
    #[test]
    fn concat_every_string_many(){
        let x = concat_every_string('d', vec!["a".to_string(), 
                                              "ab".to_string(), 
                                              "abc".to_string(), 
                                              "abcd".to_string()]);
        assert_eq!(x, vec!["da".to_string(), 
                           "dab".to_string(), 
                           "dabc".to_string(), 
                           "dabcd".to_string()]);

    }

    #[test]
    fn solution_empty_input(){
        let sol = solution("");
        assert_eq!(sol, Vec::<String>::new()); // can't infer type of empty vector so can't use vec![]
    }

    #[test]
    fn solution_invalid_input(){
        let sol = solution("1");
        assert_eq!(sol, Vec::<String>::new());
    }

    #[test]
    fn solution_very_invalid_input(){
        let sol = solution("?????????");
        assert_eq!(sol, Vec::<String>::new());
    }

    #[test]
    fn solution_one_digit(){
        let sol = solution("2");
        assert_eq!(sol, vec!["a", "b", "c"]);
    }

    #[test]
    fn solution_many_digits(){
        let sol = solution("23");
        assert_eq!(sol, vec!["ad","ae","af","bd","be","bf","cd","ce","cf"]);
    }

    fn is_ordered(data: &[String]) -> bool{
        // checks for every consecutive pair of elements in data
        // that the first element of the pair is smaller than the second one
        data.windows(2).all(|w| w[0] < w[1])
    }

    #[test]
    fn solution_ordered(){
        let sol1 = solution("8");
        assert!(is_ordered(&sol1));

        let sol2 = solution("34");
        assert!(is_ordered(&sol2));

        let sol3 = solution("23456789");
        assert!(is_ordered(&sol3));

        let sol4 = solution("98765432");
        assert!(is_ordered(&sol4));
    }
}
