use cipher_rs::cli::run::run_cli;
use colored::*;

fn main() {
  print!(
    "\n\n{}\n\n\n",
    "WELCOME TO GAVIN'S CIPHER MONSTROSITY"
      .bold()
      .bright_green()
      .underline()
  );

  run_cli();
}
