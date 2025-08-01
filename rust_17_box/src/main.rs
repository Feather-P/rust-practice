use std::{fmt, ops::Deref};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
                        _ => write!(f, ", {}", next),
                    }
                }
                // 当整个结构都为空时打印Nil
                List::Nil => write!(f, "Nil"),
            }
        }
    }

    {
        // Example 15-3 and more
        use List::{Cons, Nil};
        let test_list = Box::new(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));
        println!("{test_list}");
        let blank_list = Box::new(Nil);
        println!("{}", blank_list)
    }

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

#[cfg(test)]
mod tests {
    use crate::MyBox;

    #[test]
    fn test_deref_trait() {
        let fool_pointer = MyBox::new(5);
        assert_eq!(5, *fool_pointer)
    }
}
