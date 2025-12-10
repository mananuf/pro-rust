use e18_macros::{make_getter, make_variable};


macro_rules! sum {
    ($x: expr, $y: expr) => {
        $x + $y
    };
}

macro_rules! hello{
    ($name: expr) => {
        println!("Hello {}", $name)
    }
}

struct Data {
    value: u8,
    name: String,
}

impl Data {
    fn new(value: u8, name: &str) -> Self {
        Self { value, name: name.to_string() }
    }

    make_getter!(value, &u8);
    make_getter!(name, &String);
}

fn main() {
    hello!("moshood");
    println!("{}", sum!(1, 2));

    // ident
    make_variable!(x);
    println!("{}", x);

    let data = Data::new(8, "Hello");
    println!("{}", data.value());
    println!("{}", data.name());

    // make_getter!(macro_fn);
    // let a = macro_fn;
}
