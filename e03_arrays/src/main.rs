type ColorWheel = [f32; 3];

#[derive(Debug)]
struct RGB(ColorWheel);

// If your “dynamic” types are known in advance, use an enum.
enum Data {
    Int(i32),
    Float(f64),
    Text(String),
}

trait Displayable {
    fn show(&self);
}

struct IntegerData(i32);
struct  FloatData(f64);

struct  TextData(String);

impl Displayable for IntegerData {
    fn show(&self) {
        println!("Integer: {}", self.0)
    }
}

impl Displayable for FloatData {
    fn show(&self) {
        println!("Float: {}", self.0)
    }
}

impl Displayable for TextData {
    fn show(&self) {
        println!("Text: {}", self.0)
    }
}

fn main() {
    let mut colors: Vec<RGB> = Vec::new();
    colors.push(RGB([12.3, 234.5, 158.65]));
    colors.push(RGB([255.0, 2555.0, 255.0]));

    println!("{colors:#?}");
    println!("{:#?}", colors[0].0[0]);

    // DYNAMIC TYPES FOR ARRAYS
    // for known types
    let mixed: Vec<Data> = vec![
        Data::Int(10),
        Data::Float(3.14),
        Data::Text("hello".to_string()),
    ];

    // When you don’t know all possible types ahead of time — but they share behavior — use traits and trait objects.

    let displayable_data: Vec<Box<dyn Displayable>> = vec![
        Box::new(IntegerData(5)),
        Box::new(FloatData(3.5)),
        Box::new(TextData(String::from("Hello world")))
    ];

    for data in displayable_data {
        data.show();
    }
}
