use std::collections::HashMap;

fn main() {
    mutable_borrow_later_used();
    scope_tree();
    hash_map();
}

fn mutable_borrow_later_used() {
    let mut x = 22;
    let p = &mut x; // mutable borrow
    *p += 1; // use p
    println!("{}", x); // now we can use x
}

fn scope_tree() {
    let mut names = ["abe", "beth", "cory", "diane"]; // `names` scope start
    let alias = &mut names[0]; // `alias` scope start
    *alias = "alex"; // write to `*alias`
    println!("{}", names[0]); // read of `names[0]`
                              // `alias` scope end
                              // `name` scope end
}

fn get_default<'a>(map: &'a mut HashMap<usize, String>, key: usize) -> &'a mut String {
    if !map.contains_key(&key) {
        map.insert(key, String::from("rakudo"));
    }
    map.get_mut(&key).unwrap()
}

fn hash_map() {
    let mut map = HashMap::new();
    map.insert(22usize, String::from("hello, world!"));
    map.insert(44usize, String::from("rakudo, star!"));

    assert_eq!(get_default(&mut map, 22), "hello, world!");
    assert_eq!(get_default(&mut map, 66), "rakudo");
}
