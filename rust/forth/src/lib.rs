use std::collections::HashMap;

pub type ValueType = i64;
pub type ForthResult = Result<(), Error>;
type ApplyResult = Result<Vec<ValueType>, Error>;

#[derive(Clone)]
enum Token {
  Value(ValueType),
  Call(String),
  Def(Vec<Token>),
  EndDef,
}

pub struct Forth {
  stack: Vec<ValueType>,
  custom_ops: HashMap<String, Vec<Token>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
  DivisionByZero,
  StackUnderflow,
  UnknownWord,
  InvalidWord,
}

impl Forth {
  pub fn new() -> Self {
    Forth{ stack: vec![], custom_ops: HashMap::new() }
  }

  pub fn format_stack(&self) -> String {
    self.stack.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(" ")
  }

  pub fn eval(&mut self, input: &str) -> ForthResult {
    let input = input.to_lowercase();
    let mut words = input.split(|c: char| c.is_whitespace() || c.is_control())
                         .filter(|&word| word.len() > 0);
    let tokens = self.parse(&mut words, vec![]);

    self.execute_all(tokens)
  }

  fn parse(&self, words: &mut Iterator<Item=&str>, mut tokens: Vec<Token>) -> Vec<Token> {
    match words.next() {
      Some(":") => {
        tokens.push(Token::Def(self.parse(words, vec![])));
        self.parse(words, tokens)
      },
      Some(";") => {
        tokens.push(Token::EndDef);
        tokens
      },
      Some(x) => {
        tokens.push(self.atom(x));
        self.parse(words, tokens)
      },
      None => tokens
    }
  }

  fn atom(&self, word: &str) -> Token {
    match word.parse() {
      Ok(v) => Token::Value(v),
      Err(_) => Token::Call(word.to_string())
    }
  }

  fn execute_all(&mut self, tokens: Vec<Token>) -> ForthResult {
    for token in tokens {
      try!(self.execute(token));
    }
    Ok(())
  }

  fn execute(&mut self, token: Token) -> ForthResult {
    match token {
      Token::EndDef => Err(Error::InvalidWord),
      Token::Value(v) => self.push(v),
      Token::Call(op) => self.call(op),
      Token::Def(def) => self.add_definition(&def[..])
    }
  }

  fn call(&mut self, name: String) -> ForthResult {
    if self.custom_ops.contains_key(&name) {
      let tokens = self.custom_ops.get(&name).unwrap().clone();
      self.execute_all(tokens)
    } else {
      self.primitive(name)
    }
  }

  fn primitive(&mut self, input: String) -> ForthResult {
    match &input[..] {
      "+" => self.apply_2(|x, y| Ok(vec![x + y])),
      "-" => self.apply_2(|x, y| Ok(vec![x - y])),
      "*" => self.apply_2(|x, y| Ok(vec![x * y])),
      "/" => self.apply_2(|x, y| if y == 0 { Err(Error::DivisionByZero) } else { Ok(vec![x / y]) }),
      "dup" => self.apply_1(|x| Ok(vec![x, x])),
      "drop" => self.apply_1(|_| Ok(vec![])),
      "swap" => self.apply_2(|x, y| Ok(vec![y, x])),
      "over" => self.apply_2(|x, y| Ok(vec![x, y, x])),
      _  => Err(Error::UnknownWord)
    }
  }

  fn add_definition(&mut self, def: &[Token]) -> ForthResult {
    match (def.first(), def.last()) {
      // A definition must start with the name to define, and end with an EndDef token
      (Some(&Token::Call(ref name)), Some(&Token::EndDef)) => {
        self.custom_ops.insert(name.to_string(), def[1 .. def.len() - 1].to_vec());
        Ok(())
      },
      _ => Err(Error::InvalidWord)
    }
  }

  // Some shorthands
  fn push(&mut self, v: ValueType) -> ForthResult {
    self.stack.push(v);
    Ok(())
  }
  fn pop(&mut self) -> ValueType {
    self.stack.pop().unwrap()
  }

  // Some helper functions to make evaluating operators easier
  fn apply_1<F> (&mut self, f: F) -> ForthResult where F: Fn (ValueType) -> ApplyResult {
    if self.stack.len() < 1 { return Err(Error::StackUnderflow) }
    let x = self.pop();
    self.process_apply_result(f(x))
  }
  fn apply_2<F> (&mut self, f: F) -> ForthResult where F: Fn (ValueType, ValueType) -> ApplyResult {
    if self.stack.len() < 2 { return Err(Error::StackUnderflow) }
    let (y, x) = (self.pop(), self.pop());
    self.process_apply_result(f(x, y))
  }
  fn process_apply_result (&mut self, res: ApplyResult) -> ForthResult {
    res.map(|mut results| self.stack.append(&mut results))
  }
}
