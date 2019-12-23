pub fn is_leap_year(year: u64) -> bool {
  let divides = |k: u64, n: u64| -> bool { n % k == 0 };
  divides(4, year) && !divides(100, year) || divides(400, year)
}
