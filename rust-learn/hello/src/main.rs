use std::collections::HashMap;

fn main() {
    println!("--------STRINGS------");
    let str: &str = "hello";
    let mut string: String = String::from("Hello worlds");

    let slice = &string[..6];
    let len = slice.len();
    println!("{}", len);

    string.push('1');
    string.push_str("! HI");
    string = string.replace("Hello", "Bye");
    println!("{}", string);

    let n = 3;
    if (n > 0) {
        println!("greater than 0")
    } else {
        println!("not grater than 0")
    }

    for i in 0..8 {
        println!("{}", i)
    }

    let mut i = 0;
    while i < 4 {
        println!("{}", i);
        i += 1;
        if i == 3 {
            println!("exting...");
            break;
        }
    }

    let j = 8;
    match j {
        0 => println!("macthed 0"),
        1 | 2 => println!("matched 1, 2"),
        3..=9 => println!("matched 3 ..8"),
        _ => println!("default"),
    }

    println!("-----------STRUCTS-----------");
    let birdName = String::from("Bird1");
    let bird = Bird {
        name: birdName,
        attack: 5,
    };
    bird.print_name();

    println!("{}, {}", bird.can_fly(), bird.is_animal());

    struct Bird {
        name: String,
        attack: u64,
    }

    impl Bird {
        fn print_name(&self) {
            println!("{}", self.name)
        }
    }

    impl Animal for Bird {
        fn can_fly(&self) -> bool {
            true
        }

        //override
        fn is_animal(&self) -> bool {
            false
        }
    }

    // rust dont support inheritance so traits are helpful
    trait Animal {
        fn can_fly(&self) -> bool;
        fn is_animal(&self) -> bool {
            true
        }
    }

    println!("--------------ENUM------------");

    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(5);
    let c: MyEnum = MyEnum::C { x: 10, y: 20 };

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    #[derive(Debug)]
    enum MyEnum {
        A,
        B(i32),
        C { x: i32, y: i32 },
    }

    println!("------------VECTOR------------");

    let mut vec: Vec<i64> = vec![1, 2, 3, 4, 5];
    println!("len {}", vec.len());
    vec.push(7);
    vec.remove(0);
    println!("{:?}", vec);

    println!("--------HASH MAPS-------");
    let mut map = HashMap::new();
    map.insert(0, "HI");
    map.insert(1, "Hello");

    println!("{:?}", map);

    match map.get(&0) {
        Some(str) => println!("Found : {}", str),
        None => println!("Doesn't exist in map"),
    }
    match map.get(&2) {
        Some(str) => println!("{}", str),
        None => println!("Doesn't exist in map"),
    }

    map.remove(&0);
    println!("{:?}", map);

    println!("-------OPTION------");

    fn divide(dividend: i32, divisor: i32) -> Option<i32> {
        if dividend % divisor != 0 {
            None
        } else {
            Some(dividend / divisor)
        }
    }

    let divide1: Option<i32> = divide(4, 2);
    let divide2: Option<i32> = divide(2, 3);

    println!("{:?} unwraps to {}", divide1, divide1.unwrap());
    //uncomment below to see panic(exception)
    // println!("{:?} unwarps to {}", divide2, divide2.unwrap());

    println!("-----RESULT------");
    #[derive(Debug)]
    enum MyError {
        Error1,
    }
    //Err, an enum that contains error code
    //Ok(value), A wrapper that contains a value
    fn divideValue(dividend: i32, divisor: i32) -> Result<i32, MyError> {
        if dividend % divisor != 0 {
            Err(MyError::Error1)
        } else {
            Ok(dividend / divisor)
        }
    }

    let divideRes = divideValue(5, 2);

    //way - 1
    match divideRes {
        Ok(v) => println!("{}", v),
        Err(err) => println!("{:?}", err),
    }
    //ways - 2
    // if divideRes.is_ok() {
    //     println!("{}", divideRes.unwrap());
    // }

    //way - 3
    // println!("{}", divideRes.unwrap());

    //way -4
    // println!("{}", divideRes.unwrap_or(100));
}
