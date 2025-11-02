pub mod math {
    pub fn add(a: f64, b: f64) -> f64 {
        a + b
    }
}

pub fn shout(s: &str) -> String {
    s.to_uppercase()
}

pub fn get_index(v: &[i32], idx: usize) -> i32 {
    v[idx]
}
