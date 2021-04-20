/// What should the type of _function be?
pub fn map<T, U>(input: Vec<T>, mut f: impl FnMut(T) -> U) -> Vec<U> {
    let mut result = vec![];
    for value in input.into_iter() {
        result.push(f(value));
    }
    result
}
