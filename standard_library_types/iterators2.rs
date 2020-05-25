// iterators2.rs
// In this module, you'll learn some of unique advantages that iterators can offer.
// Step 1. Complete the `capitalize_first` function to pass the first two cases.
// Step 2. Apply the `capitalize_first` function to a vector of strings.
//         Ensure that it returns a vector of strings as well.
// Step 3. Apply the `capitalize_first` function again to a list.
//         Try to ensure it returns a single string.
// As always, there are hints if you execute `rustlings hint iterators2`!


pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn capitalize_words_for_vector(input: Vec<&str>) -> Vec<String> {
    let mut result_vector: Vec<String> = Vec::new();
    let iterator = input.iter();
    for string_value in iterator {
        result_vector.push(capitalize_first(string_value));
    }

    result_vector
}

pub fn capitalize_words_as_string(input: Vec<&str>) -> String {
    let mut result_string: String = String::new();

    let strings_as_vector = capitalize_words_for_vector(input);

    let iterator = strings_as_vector.iter();

    for string_value in iterator {
        result_string.push_str(string_value);
    }

    result_string
}

#[cfg(test)]
mod tests {
    use super::*;

    // Step 1.
    // Tests that verify your `capitalize_first` function implementation
    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    // Step 2.
    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        let capitalized_words: Vec<String> = capitalize_words_for_vector(words);
        assert_eq!(capitalized_words, ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        let capitalized_words = capitalize_words_as_string(words);
        assert_eq!(capitalized_words, "Hello World");
    }
}
