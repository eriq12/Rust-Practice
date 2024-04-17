fn main() {
    print!("{}\n", string_bigger_than_other("b", "ab"));
}

pub fn string_bigger_than_other(string_one: &str, string_two: &str) -> bool{
    for (char_one, char_two) in string_one.chars().rev().zip(string_two.chars().rev()) {
        print!("{} > {}?\n", char_one, char_two);
        if char_one != char_two {
            return char_one > char_two;
        }
    }
    string_one.len() > string_two.len()
}