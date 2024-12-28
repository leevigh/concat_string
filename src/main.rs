fn main() {
    let string1: String = String::from("Hello");
    let string2: String = String::from(" world");

    let concatenated_string: String;

    concatenated_string = concatenate_strings(&string1, &string2);

    println!("{}", concatenated_string);
}

fn concatenate_strings(string_1: &str, string_2: &str) -> String {
    let mut result: String = String::new();

    result.push_str(string_1);
    result.push_str(string_2);

    result
}
