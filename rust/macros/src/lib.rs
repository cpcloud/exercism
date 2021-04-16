#[macro_export]
macro_rules! hashmap {
    ($($k:expr => $v:expr,)*) => {{
        let mut result = ::std::collections::HashMap::new();
        $(result.insert($k, $v);)*
        result
        // alternative implementation
        // vec![$(($k, $v)),*].into_iter().collect::<::std::collections::HashMap<_, _>>()
    }};
    ($($k:expr => $v:expr),*) => {{
        $crate::hashmap!($($k => $v,)*)
    }};
}
