fn main() {
    vec_literal();
    out_of_bounds_access();
    iterator_vec();
    pass_reference();
    copy_trait();

    enumerate();
    on_iteratirs();
    loop_break();
    for_continue();
    loop_labels();
}

fn vec_literal() {
    let v = vec![1, 2, 3, 4, 5]; // v: Vec<i32>
    for x in v {
        println!("{}", x);
    }

    // 访问元素
    println!("The third element of vector is {}", vec![1, 2, 3, 4, 5][2]);

    let vv = vec![0; 10]; // ten zeros
    for xx in vv {
        println!("xx: {}", xx);
    }
}

fn out_of_bounds_access() {
    let v = vec![1, 2, 3];
    match v.get(7) {
        Some(x) => println!("Item 7 is {}", x),
        None => println!("Sorry, this vector is too short."),
    }
}

fn iterator_vec() {
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("A reference to {}", i);
    }
    for i in &mut v {
        println!("A mutable reference to {}", i);
    }
    for i in v {
        println!("Take ownership of the vector and its elements {}", i);
    }
}

fn pass_reference() {
    let v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("This is a reference to {}", i);
    }
    for i in &v {
        println!("This is a reference to {}", i);
    }
}

// 基本类型都实现了 Copy trait, 会完整拷贝
fn copy_trait() {
    let v = 1;
    let v2 = v;
    println!("v is: {}", v);
}

fn enumerate() {
    for (index, value) in (5..10).enumerate() {
        println!("index = {} and value = {}", index, value);
    }
}

fn on_iteratirs() {
    let lines = "Hello World\nRakudo Star\nRakudo Perl 6".lines();

    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }
}

fn loop_break() {
    let mut x = 5;
    loop {
        x += x - 3;
        println!("{}", x);

        if x % 5 == 0 {
            break;
        }
    }
}

fn for_continue() {
    for x in 0..10 {
        if x % 2 == 0 {
            continue;
        }
        println!("{}", x)
    }
}

fn loop_labels() {
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 {
                continue 'outer;
            } // continues the loop over x
            if y % 2 == 0 {
                continue 'inner;
            } // continues the loop over y
            println!("x: {}, y: {}", x, y);
        }
    }
}
