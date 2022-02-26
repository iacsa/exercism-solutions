use std::cmp::PartialEq;
use std::fmt::Display;
use std::ops::Rem;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T>
where
    T: Copy,
{
    f: Box<dyn Fn(T) -> bool>,
    sub: String,
}

impl<T> Matcher<T>
where
    T: Copy,
{
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: Display,
    {
        Self {
            f: Box::new(matcher),
            sub: format!("{}", subs),
        }
    }

    fn apply(&self, item: T) -> Option<String> {
        ((self.f)(item)).then(|| self.sub.clone())
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T>
where
    T: Copy + Display,
{
    matchers: Vec<Matcher<T>>,
}

impl<T> Fizzy<T>
where
    T: Copy + Display,
{
    pub fn new() -> Self {
        Self { matchers: vec![] }
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        iter.map(move |x| {
            let res = self
                .matchers
                .iter()
                .filter_map(|m| m.apply(x))
                .collect::<String>();
            if res == "" {
                format!("{}", x)
            } else {
                res
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: PartialEq + Copy + Display + Rem<T, Output = T> + From<u8>,
{
    let fizz = Matcher::new(|x| x % T::from(3u8) == T::from(0u8), "fizz");
    let buzz = Matcher::new(|x| x % T::from(5u8) == T::from(0u8), "buzz");
    Fizzy::new().add_matcher(fizz).add_matcher(buzz)
}
