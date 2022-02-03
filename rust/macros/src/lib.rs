#[macro_export]
macro_rules! hashmap {
    ($($i:expr => $j:expr,)*) => {{
        let mut hm = crate::HashMap::new();
        $(
            hm.insert($i, $j);
        )*
        hm
    }};
    ($($i:expr => $j:expr),*) => {{
        crate::hashmap!($($i => $j,)*)
    }};
}
