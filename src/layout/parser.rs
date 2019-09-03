/// https://limpet.net/mbrubeck/2014/08/11/toy-layout-engine-2.html

pub struct Parser {
  pos: usize, // "usize" is an unsigned integer, similar to "size_t" in C
  input: String,
}

impl Parser {
  // Read the current character without consuming it.
  fn next_char(&self) -> char {
    self.input[self.pos..].chars().next().unwrap()
  }

  // Do the next characters start with the given string?
  fn starts_with(&self, s: &str) -> bool {
    self.input[self.pos..].starts_with(s)
  }

  // Return true if all input is consumed.
  fn eof(&self) -> bool {
    self.pos >= self.input.len()
  }

  // Return the current character, and advance self.pos to the next character.
  fn consume_char(&mut self) -> char {
    let mut iter = self.input[self.pos..].char_indices();
    let (_, cur_char) = iter.next().unwrap();
    let (next_pos, _) = iter.next().unwrap_or((1, ' '));
    self.pos += next_pos;
    return cur_char;
  }

  /// Consume and discard zero or more whitespace characters.
  fn consume_whitespace(&mut self) {
    self.consume_while(char::is_whitespace);
  }

  /// Consume characters until `test` returns false.
  fn consume_while<F>(&mut self, test: F) -> String
  where
    F: Fn(char) -> bool,
  {
    let mut result = String::new();
    while !self.eof() && test(self.next_char()) {
      result.push(self.consume_char());
    }
    result
  }
}
