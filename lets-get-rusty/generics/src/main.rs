fn main() {
    let num_list = vec![1, 2, 3, 4, 5];

    let largest = get_largest(num_list);

    // above function works for any list of type vec<i32> but what if we have char list ?
    // we can have one more function to compare chars in a list returns the largest char but how can we do it with single function?
    // we use generics.

    let char_list = vec!['k', 'r', 'i', 's', 'h', 'n', 'a'];

    let char_lar = get_largest(char_list);
    
    println!("Largest num is : {} and Largest char is : {}", largest, char_lar)
}

// <--------Without generics---cant use below function for a list other tham i32 list----->
// fn get_largest(num_list : Vec<i32>) -> i32 {
//     let mut largest =num_list[0];
//     for num in num_list {
//         if num > largest {
//             largest = num
//         }
//     }
//     largest
// }

// below function can be used for i32 list and char list
fn get_largest<T : PartialOrd + Copy>(num_list : Vec<T>) -> T {
    let mut largest =num_list[0];
    for num in num_list {
        if num > largest {
            largest = num
        }
    }
    largest
}



mod generics_in_structs_and_structs {
    // x and y are of same type T
    struct Point<T> {
        x : T,
        y : T
    }

    struct Point2<T, U>{
        x : T,
        y : U
    }


    fn test(){
        let x1 = Point { x : 4, y : 5};
        // x2 will not be possible if struct is : 
        #[allow(unused_doc_comments)]
        /**
         * struct Point {
         *      x : i32,
         *      y : i32
         * }
         */
        // with Point being of generic type we can have any type of point
        let x2 = Point { x : 10.9, y : 8.9};


        let x3 = Point2 { x : 32, y : 5.9};
    }


    // generics in enums
    enum Option<T>{
        Some(T),
        None
    }

    enum Result<T, E>{
        Ok(T),
        Err(E)
    }
}

