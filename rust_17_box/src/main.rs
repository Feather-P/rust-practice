use std::fmt;

fn main() {
    let b = Box::new(5);
    println!("i32 data was stored in the heap: {b}");

    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                List::Cons(value, next) => {
                    // 先写当前值
                    write!(f, "{}", value)?;

                    // 如果不是 Nil，继续写后面的元素
                    match **next {
                        List::Nil => Ok(()),
                        _ => write!(f, " -> {}", next),
                    }
                }
                List::Nil => write!(f, "Nil"),
            }
        }
    }

    {
        // Example 15-3 and more
        use List::{Cons, Nil};
        let test_list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        println!("{test_list}")
    }
}
