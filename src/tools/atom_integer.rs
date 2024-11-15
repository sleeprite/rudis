pub struct AtomInteger {
    value: i64,
}

impl AtomInteger {

    pub fn new() -> Self {
        AtomInteger {
            value: 0,
        }
    }

    pub fn get(&self) -> i64 {
        self.value
    }

    pub fn increment(&mut self) {
        self.value += 1;
    }
}