fn main() {
    disjoint_capture_in_closures();
    new_bindings();
    iterate_by_value();

    let (x, y) = try_from_and_try_into(3);
    println!("{:?} {:?}", x, y);
}

#[derive(Debug)]
struct SomeStruct {
    x: String,
    y: String,
}

// Closures in Rust 2021 will capture only the fields of an object you use,
// instead of the entire object
fn disjoint_capture_in_closures() {
    let a = SomeStruct {
        x: "a".to_string(),
        y: "b".to_string(),
    };
    drop(a.x); // Move out of one field of the struct
    println!("{:?}", a.y); // Still use another field of the struct is OK

    let c = || println!("{:?}", a.y);
    c();

    let mut x = ("Rakudo".to_string(), "Star".to_string());
    let mut f = || x.0 += "..."; // captures only &mut x.0

    println!("{:?}", f());
    // f now borrows only x.0, so we can still modify x.1

    println!("{:?}", x.1 += "...");
}

#[derive(Debug)]
struct Matrix {
    data: Vec<f64>,
    row_len: usize,
}

fn new_bindings() {
    let matrix @ Matrix { row_len, .. } = Matrix {
        data: vec![0f64; 5],
        row_len: 42,
    };
    println!("{:?}", matrix);
    println!("{:?}", row_len);
}

// `array.into_iter()` now iterates by value, instead of giving references.
fn iterate_by_value() {
    let v = vec![1, 2, 3, 4];
    for (i, x) in v.into_iter().enumerate() {
        println!("{:?} {:?}", i, x);
    }
}

fn try_from_and_try_into(x: u32) -> (i32, Vec<u8>) {
    let x: u8 = x.try_into().unwrap_or(u8::MAX);
    let v = Vec::from_iter(0..x);
    let n = i32::try_from(v.len()).unwrap();
    (n, v)
}

fn panic_assert() {
    let a = 5;
    //panic!("hey {a}");
}
