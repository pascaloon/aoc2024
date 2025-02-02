
pub fn add_tuple<T>(a: (T, T), b: (T, T)) -> (T, T)
    where T: std::ops::Add<Output = T> {
    (a.0 + b.0, a.1 + b.1)
}