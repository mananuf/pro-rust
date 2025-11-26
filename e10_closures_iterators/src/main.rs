use std::iter::Sum;

fn main() {
    let mut x: u8 = 0;
    let mut increment = || x += 1;
    increment();
    increment();
    increment();
    increment();
    println!("{}", x);

    let v: fn(i32, i32) -> bool = verify;

    let z = |y: u8| x + y;

    let vc = vec![1, 2, 3, 4];
    let mv = || vc.iter().max();
    println!("{:?}", mv());

    let square = |x: i32| x + 10;
    println!("{}", square(2));

    let closure_string: String = String::from("Hello");
    let mover = move || println!("{closure_string}");
    mover();
    mover();
    mover();
    fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
        move |f: i32| factor * f
    }

    let times = make_multiplier(10);

    println!("{}", times(3));
    // println!("{}", mover());

    let res: Vec<_> = (1..)
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .take(5)
        .collect();

    println!("ITERATOR PIPELINE: {:?}", res);

    let v = vec!["1", "foo", "3", "1.4"];
    let nums: Vec<i32> = v.iter().filter_map(|s| s.parse::<i32>().ok()).collect();

    println!("FILTER MAP: {:?}", nums);

    let nested = vec![vec![1, 2], vec![3]];
    let flat: Vec<_> = nested.into_iter().flat_map(|v| v).collect();

    println!("FLAT MAP: {:?}", flat);

    let sum: i32 = (1..10).filter(|f| f % 2 == 0).map(|f| f * f).sum();

    println!("SUM: {:?}", sum);

    let max = vec![3, 5, 2, 12, 5].into_iter().reduce(|a, b| a.max(b));
    println!("REDUCE: {:?}", max);

    let running_sum = (1..=5)
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .collect::<Vec<_>>();

    println!("STATE manipulation {:?}", running_sum);

    let mut it = vec![1, 2, 3].into_iter().peekable();
    if let Some(&v) = it.peek() {
        println!("PEEKABLE: {}", v);
    }

    let doubles: Vec<i32> = vec![1..10]
        .into_iter()
        .flat_map(|range| range)
        .filter(|f| f % 2 == 0)
        .map(|f| f * f)
        .collect();

    println!("doubles {:?}", doubles);

    let strings_to_int: Vec<i32> = vec!["1", "hello", "64", "viva"]
        .into_iter()
        .filter_map(|f| f.parse::<i32>().ok())
        .collect();

    println!("strings_to_int {:?}", strings_to_int);

    let nested = vec![vec![1, 2], vec![3, 4, 5]];
    let flattened_sum: i32 = nested.into_iter().flat_map(|f| f).sum();

    println!("flattened_sum {:?}", flattened_sum);
}

fn verify(a: i32, b: i32) -> bool {
    if a > b {
        return true;
    }

    false
}
