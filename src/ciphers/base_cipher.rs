pub trait Cipher {
  fn encode(&self, text: &str) -> String;
  fn decode(&self, text: &str) -> String;
}