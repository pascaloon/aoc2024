pub struct CharMap<'a> {
    data: &'a[u8],
    pub width: i32,
    pub height: i32
}

impl CharMap<'_> {
    pub fn from_str(data: &str) -> CharMap {
        let width = data.lines().next().unwrap().len() as i32;
        let height = data.lines().count() as i32;
        let data = data.as_bytes();
        CharMap { data, width, height }
    }

    pub fn get_char(&self, x: i32, y: i32) -> char {
        match (x, y) {
            (_, _) if x < 0 => '\0',
            (_, _) if x >= self.width => '\0',
            (_, _) if y < 0 => '\0',
            (_, _) if y >= self.height => '\0',
            (_, _) => {
                let i = (y * (self.width + 1)) + x; // +1 for the \n chars
                *self.data.get(i as usize).unwrap() as char
            }
        }
    }
}
