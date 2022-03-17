use inquire::{Confirm, CustomType, Select, Text};

// Asks to encode or decode
pub fn ask_encode_or_decode() -> String {
  Select::new(
    "Would you like to encode a new message, or decode an existing one?",
    vec!["Encode", "Decode"],
  )
  .prompt()
  .unwrap()
  .to_owned()
}

// Asks to run the program again
pub fn ask_go_again() -> bool {
  Confirm::new("Would you like to go again?")
    .with_default(false)
    .prompt()
    .unwrap()
}

// Asks for any text with a message that is provided with `text` param
pub fn ask_for_text(text: &str) -> String {
  Text::new(&format!("Please enter {}", text))
    .prompt()
    .unwrap()
}

// Asks for any usize with a message that is provided with `text` param
pub fn ask_for_num(text: &str) -> usize {
  CustomType::<usize>::new(&format!("Please enter {}", text))
    .with_parser(&|i| match i.parse::<usize>() {
      Ok(val) => Ok(val),
      Err(_) => Err(()),
    })
    .with_error_message("Please type a valid number")
    .prompt()
    .unwrap()
}
