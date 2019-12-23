pub fn build_proverb(list: &[&str]) -> String {
    // Deal with the irregular case first
    if list.len() == 0 {
        return String::new();
    }

    // Pre-allocate the space for the large result String
    let proverb_length = list.iter().map(|s| s.len()).sum::<usize>() * 2 + list.len() * 30 + 27;
    let mut proverb = String::with_capacity(proverb_length);

    // Build the result via push_str(), which is faster than the alternatives
    for (wanted_item, lost_item) in list.iter().zip(list[1..].iter()) {
        proverb.push_str("For want of a ");
        proverb.push_str(wanted_item);
        proverb.push_str(" the ");
        proverb.push_str(lost_item);
        proverb.push_str(" was lost.\n");
    }

    // The final verse has a different structure
    proverb.push_str("And all for the want of a ");
    proverb.push_str(list[0]);
    proverb.push_str(".");

    proverb
}
