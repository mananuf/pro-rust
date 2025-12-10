use e18_macros::{call_twice, make_getter, make_struct, make_variable};


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


    // type_macros
    make_struct!(StructBuilder; data, Vec<u8>);
    let struct_builder = StructBuilder::new(vec![1,2,3,4,5]);
    println!("{:#?}", struct_builder.data);

    fn double(value: u8) -> u8 {
        value * 2
    }

    let res = call_twice!(double, 2);
    println!("{res}");
}
