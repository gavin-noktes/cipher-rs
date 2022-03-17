use crate::util;

pub fn ceasar_cipher(encode_or_decode: &str, text: String, mut num: usize) -> String {
  // Alter this vector to change the way the cipher shifts
  let alphabet: Vec<char> = vec![
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
  ];

  // Makes sure num is less than alphabet.len() by using modulo operator
  // This is to make sure there is no subtraction with overflow
  // Ex: 1 - 2 = usize::MAX (not what we want)
  num = num % alphabet.len();

  // Turns the entered text into a vector of chars in order to loop through them
  let text_vec: Vec<char> = text.chars().collect();

  // Loops through text_vec and turns them into `usize` so math can be run on them
  // Then adds/subtracts those index's by num which provides a new index
  // Then gets the char at alphabet[new index] and returns it
  let mut encoded_text: Vec<char> = Vec::new();
  for letter in text_vec {
    if alphabet.contains(&letter.to_ascii_lowercase()) {
      let letter_pos = alphabet
        .iter()
        .position(|&r| &r == &letter.to_ascii_lowercase())
        .unwrap();

      // If encoding, then push the letter added by num so (a = 0) + 2 = (c = 2)
      if encode_or_decode == "Encode" {
        // Handle uppercase letters for encoding
        encoded_text = util::push_letter_upper(
          &encoded_text,
          letter,
          alphabet[(letter_pos + num) % alphabet.len()],
        );
      } else {
        // Handle where num is greater than letter_pos (subtraction overflow)
        let val: usize;
        if num > letter_pos {
          val = (alphabet.len() + letter_pos) - num;
        } else {
          val = letter_pos - num;
        }
        // Handle uppercase letters for decoding
        encoded_text = util::push_letter_upper(&encoded_text, letter, alphabet[val]);
      }
    } else {
      // If we don't know what letter it is, just add it without adding to it
      // This is for operators, punctuation, numbers, etc
      encoded_text.push(letter)
    }
  }

  // Returns the vec of chars as a String
  encoded_text.iter().collect::<String>()
}
