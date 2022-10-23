fn main() {
    let a = 40 * 5 / 100;
    let b = SomeStruct::new("");

    println!("{}", a);
    // println!("{}", std::mem::size_of());
}

struct SomeStruct {
    a: String,
}

impl SomeStruct {
    pub fn new(s: &str) -> Self {
        Self {
            a: s.to_string(),
        }
    }
}