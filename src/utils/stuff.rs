
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

pub trait MyVecUtils<T>
where T: Copy {
    fn add_repeated(&mut self, count: u32, value: T);
}
impl<T> MyVecUtils<T> for Vec<T>
where T: Copy {
    fn add_repeated(&mut self, count: u32, value: T)
     {
         self.reserve(count as usize);
        for _ in 0..count {
            self.push(value);
        }
    }
}
