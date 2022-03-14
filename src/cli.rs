use crate::ciphers::ceasar_cipher;
use inquire::{Confirm, CustomType, Select, Text};

pub fn run_cli() {
  let mut running = true;

  while running {
    {
      // Asks whether to encode or decode `text` text and by `num_add` much
      let encode_or_decode = ask_encode_or_decode();
      let text = ask_for_text(&format!("text to {}", &encode_or_decode));
      let num_add = ask_for_num("number shift by");

      // Prints results along with provided configs
      println!("\n{}ing: {text}", &encode_or_decode[..5]);
      println!("{}ing By: {num_add}", &encode_or_decode[..5]);
      println!(
        "Result: {}\n",
        ceasar_cipher(&encode_or_decode, text, num_add)
      );
    }

    // Asks whether to run the ceasar cipher again
    running = ask_go_again();
  }
}

// Asks to encode or decode
fn ask_encode_or_decode() -> String {
  Select::new(
    "Would you like to encode a new message, or decode an existing one?",
    vec!["Encode", "Decode"],
  )
  .prompt()
  .unwrap()
  .to_owned()
}

// Asks to run the program again
fn ask_go_again() -> bool {
  Confirm::new("Would you like to go again?")
    .with_default(false)
    .prompt()
    .unwrap()
}

// Asks for any text with a message that is provided with `text` param
fn ask_for_text(text: &str) -> String {
  Text::new(&format!("Please enter {}", text))
    .prompt()
    .unwrap()
}

// Asks for any usize with a message that is provided with `text` param
fn ask_for_num(text: &str) -> usize {
  CustomType::<usize>::new(&format!("Please enter {}", text))
    .with_parser(&|i| match i.parse::<usize>() {
      Ok(val) => Ok(val),
      Err(_) => Err(()),
    })
    .with_error_message("Please type a valid number")
    .prompt()
    .unwrap()
}
