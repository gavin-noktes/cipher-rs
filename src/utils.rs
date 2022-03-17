// Handle uppercase letters
pub fn push_letter_upper(list: &Vec<char>, letter_check: char, letter_add: char) -> Vec<char> {
  let mut list = list.clone();

  if letter_check.is_ascii_uppercase() {
    // Makes sure uppercase letters are treated as such in the final vec
    list.push(letter_add.to_ascii_uppercase());
  } else {
    // Makes sure lowercase letters are treated as such in the final vec
    list.push(letter_add);
  }

  list
}
