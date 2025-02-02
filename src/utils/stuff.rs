
pub fn add_tuple<T>(a: (T, T), b: (T, T)) -> (T, T)
    where T: std::ops::Add<Output = T> {
    (a.0 + b.0, a.1 + b.1)
}

pub fn permutate<T>(a: &[T]) -> Vec<(T, T)>
where T: Copy {
    let mut results = Vec::with_capacity((a.len() * a.len()) - 1);
    for i in 0..a.len() {
        for j in 0..a.len() {
            if i != j {
                results.push((a[i], a[j]))
            }
        }
    }
    
    results
}