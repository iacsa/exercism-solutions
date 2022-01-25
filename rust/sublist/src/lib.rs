#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(v1: &[T], v2: &[T]) -> Comparison {
    if v1 == v2 {
        Comparison::Equal
    } else if is_sublist(v1, v2) {
        Comparison::Sublist
    } else if is_sublist(v2, v1) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

fn is_sublist<T: PartialEq>(v1: &[T], v2: &[T]) -> bool {
    let (l1, l2) = (v1.len(), v2.len());
    l1 < l2 && (0..l2 - l1 + 1).any(|i| v1 == &v2[i..l1 + i])
}
