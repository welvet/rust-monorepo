mod book_struct;

use std::fmt;
use book_struct::Cat;
use std::collections::HashMap;

fn simple_borrow() {
    let vec = vec![1, 2, 3];
    let sl = &vec[0..2];
    println!("{:?}", sl);
}

fn modifying_borrow() {
    let mut vec = vec![1, 2, 3];
    let sl = &mut vec[0..2];
    sl[1] = 4;
    vec[2] = 4;
    // sl[1] = 4; <- impossible cause only one mutable borrow exists at time

    println!("{:?}", vec);
}

fn strs() {
    let s = String::from("abc");
    let added = add_str(&s);
    println!("{}", added);
}

fn add_str(s: &str) -> String {
    let st = String::from(s);
    let nst = st + "abc";
    // st is Moved to "+" (add) function
    nst
}

struct TestStruct {
    name: String,
    age: Option<i32>,
    descr: Option<String>,
}

impl fmt::Display for TestStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "name: {}, age: {}, descr: {}",
            self.name,
            self.age.unwrap_or(-1),
            // self.descr.as_deref().unwrap_or("<none>")
            self.descr.as_ref().map(|x| x.as_str()).unwrap_or("<none>")
        )
    }
}

fn struct_test() {
    let s = TestStruct {
        name: String::from("abc"),
        age: None,
        descr: None,
    };

    let _a = &s.name;
    println!("{}", s);

    let b = String::from(s.name);
    println!("{}", b);
    // println!("{}", s.name); not allowed cause partial borrow
}

impl Cat {
    fn meow2(&self) -> String {
        format!("meow2: {}", self.name)
    }
}

fn extra_cat() {
    let cat = Cat {
        name: String::from("nya")
    };

    println!("{}", cat.meow());
    println!("{}", cat.meow2());
}

fn map() {
    let mut map = HashMap::new();
    map.insert(String::from("test"), "test2");
    map.insert(String::from("test2"), "test2");

    println!("{}", map.get("test").expect("no key"))
}

fn reallymap() {
    let mut map = HashMap::new();
    {
        let s = String::from("test");
        map.insert(s.as_str(), s.as_str());
    }

    // println!("{}", map.get("test2").expect("no key")) wontwork
}

fn main() {
    simple_borrow();
    modifying_borrow();
    strs();
    struct_test();
    extra_cat();
    map();
    reallymap();
}
