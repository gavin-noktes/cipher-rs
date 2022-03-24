pub fn shuffle_chipher(text: String, num: usize) -> String {
  // encode and decode does the same thing and encode (or) decode
  // twice turn the text back to normal 

  // turn text to char vector
  let text_vec: Vec<char> = text.chars().collect();

  // create 2D vector of char (row = ceiling of text_length/num)
  let mut row: usize = text.len() / num;
  if text.len() % num > 0 {
      row += 1; 
  }

  let mut encoded_text_vec:Vec<char> = Vec::new();

  // insert char from text_vec
  for i in 0..num {
      for j in 0..row {
          if (i + j*num) >= text.len() {
              break;
          }
          encoded_text_vec.push(text_vec[i + j*num]);
      }
  }

  // return encoded text
   encoded_text_vec.iter().collect::<String>()
}