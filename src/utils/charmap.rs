pub struct CharMap<'a> {
    data: &'a[u8],
    pub width: i32,
    pub height: i32,
    overrides: Vec<u8>
}
impl CharMap<'_> {
    pub fn from_str(data: &str) -> CharMap {
        let width = data.lines().next().unwrap().len() as i32;
        let height = data.lines().count() as i32;
        let data = data.as_bytes();
        let overrides = Vec::new();
        CharMap { data, width, height, overrides }
    }

    pub fn get_char(&self, x: i32, y: i32) -> char {
        match (x, y) {
            (_, _) if x < 0 => '\0',
            (_, _) if x >= self.width => '\0',
            (_, _) if y < 0 => '\0',
            (_, _) if y >= self.height => '\0',
            (_, _) => {
                let i = (y * (self.width + 1)) + x; // +1 for the \n chars
                if self.overrides.len() > 0 {
                    let o = self.overrides[i as usize] as char;
                    if o != '\0' {
                        return o;
                    }
                }
                *self.data.get(i as usize).unwrap() as char
            }
        }
    }

    pub fn set_override(&mut self, x: i32, y: i32, v: char) {
        if self.overrides.is_empty() {
            self.overrides = vec!['\0' as u8; ((self.width+1) * self.height) as usize];
        }

        match (x, y) {
            (_, _) if x < 0 => panic!("Out of Bound!"),
            (_, _) if x >= self.width => panic!("Out of Bound!"),
            (_, _) if y < 0 => panic!("Out of Bound!"),
            (_, _) if y >= self.height => panic!("Out of Bound!"),
            (_, _) => {
                let i = (y * (self.width + 1)) + x; // +1 for the \n chars
                self.overrides[i as usize] = v as u8;
            }
        }
    }

    pub fn remove_override(&mut self, x: i32, y: i32) {
        self.set_override(x, y, '\0');
    }
}