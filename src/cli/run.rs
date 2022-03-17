use super::user_input::{ask_encode_or_decode, ask_for_num, ask_for_text, ask_go_again};
use crate::ciphers::ceasar_cipher;

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
