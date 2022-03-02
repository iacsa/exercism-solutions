pub type Value = i64;

pub fn answer(s: &str) -> Option<Value> {
    WordProblem::new(s).answer()
}

struct WordProblem {
    s: String,
}

impl WordProblem {
    fn new(s: &str) -> Self {
        WordProblem { s: s.to_string() }
    }

    // Check whether the next token is the expected token.
    // This is used when we need to ensure that the syntax is correct, but we don't
    // need to actually do something with this token.
    fn expect(&self, iter: &mut dyn Iterator<Item = &str>, s: &'static str) -> Option<()> {
        iter.next().filter(|&t| s == t).map(|_| ())
    }

    // Check that the next token is a number and parse it to Value type.
    fn expect_number<F>(&self, iter: &mut dyn Iterator<Item = &str>, f: F) -> Option<Value>
    where
        F: Fn(Value) -> Value,
    {
        iter.next()
            .and_then(|word| word.parse().ok())
            // A number must always be followed with an operation
            .and_then(|number| self.expect_operation(iter, f(number)))
    }

    // Check that the next token is a number and parse it to Value type.
    fn expect_exponent<F>(&self, iter: &mut dyn Iterator<Item = &str>, f: F) -> Option<Value>
    where
        F: Fn(Value) -> Value,
    {
        iter.next()
            .and_then(|word| word.parse().ok())
            // A number must always be followed with an operation
            .and_then(|number| self.expect_operation(iter, f(number)))
    }

    fn expect_operation(&self, iter: &mut dyn Iterator<Item = &str>, v: Value) -> Option<Value> {
        if let Some(word) = iter.next() {
            match word {
                "plus" => self.expect_number(iter, |y| v + y),
                "minus" => self.expect_number(iter, |y| v - y),
                "multiplied" => {
                    self.expect(iter, "by")?;
                    self.expect_number(iter, |y| v * y)
                }
                "divided" => {
                    self.expect(iter, "by")?;
                    self.expect_number(iter, |y| v / y)
                }
                /* NOT WORKING */
                /*
                "raised" => {
                    self.expect(iter, "to")?;
                    self.expect(iter, "th")?;
                    self.expect_exponent(iter, |y| v.pow(y as u32))
                }*/
                _ => None, // incorrect query
            }
        } else {
            Some(v) // end of question
        }
    }

    fn answer(&self) -> Option<Value> {
        // The question mark is necessary for a correct query ...
        if !self.s.ends_with("?") {
            return None;
        }
        // ... but it's otherwise unimportant, so ignore it.
        let mut words = self.s[..self.s.len() - 1].split_whitespace();

        // Queries always start with "What is".
        self.expect(&mut words, "What")?;
        self.expect(&mut words, "is")?;

        // Thereafter, process the query recursively.
        // It must begin with a number.
        self.expect_number(&mut words, |x| x)
    }
}
