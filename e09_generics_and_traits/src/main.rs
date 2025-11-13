use std::fmt::Debug;


trait Describable: Debug {
    fn describe(&self) -> String;
}


type Items = Vec<Box<dyn Describable + Send + Sync>>;

#[derive(Debug)]
struct Tool<'a> {
    name: &'a str,
    weight: f32
}

#[derive(Debug)]
struct Electronic<'a> {
    name: &'a str,
    power: f32
}

#[derive(Debug)]
struct Food<'a> {
    name: &'a str,
    calories: f32
}

#[derive(Debug)]
struct Warehouse {
    items: Items
}

impl<'a> Tool<'a> {
    fn new(name: &'a str, weight: f32) -> Self {
        Self { name, weight }
    }
}

impl<'a> Describable for Tool<'a> {
    fn describe(&self) -> String {
        format!("\nname: {}\nweight: {}", self.name, self.weight)
    }
}

impl<'a> Electronic<'a> {
    fn new(name: &'a str, power: f32) -> Self {
        Self { name, power }
    }
}

impl<'a> Describable for Electronic<'a> {
    fn describe(&self) -> String {
        format!("\nname: {}\npower: {}", self.name, self.power)
    }
}

impl<'a> Food<'a> {
    fn new(name: &'a str, calories: f32) -> Self {
        Self { name, calories }
    }
}

impl<'a> Describable for Food<'a> {
    fn describe(&self) -> String {
        format!("\nname: {}\ncalories: {}", self.name, self.calories)
    }
}

impl Warehouse  {
    fn add(items: Items)-> Self {
        Self{ items }
    }

    fn list(&self) -> Vec<String> {
        let item_descriptions = self.items.iter().map(|i| i.describe()).collect();
        vec![item_descriptions] 
    }
}

fn main() {
    let hammer: Tool<'_> = Tool::new("hammer", 24.0);
    let spaghetti: Food<'_> = Food::new("spaghetti", 10.0);
    let television: Electronic<'_> = Electronic::new("Hisense tv", 210.5);

    let warehouse: Warehouse  = Warehouse::add(vec![Box::new(hammer), Box::new(spaghetti), Box::new(television)]);

    let items = warehouse.list();

    for a in items {
        println!("{a}")
    }
}


