use super::Foo;

impl Foo {
    pub fn increment(&mut self) {
        self.bar += 1;
    }
}
