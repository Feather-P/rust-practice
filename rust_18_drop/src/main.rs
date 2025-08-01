use core::str;
use std::cell::RefCell;
use std::rc::Rc;

struct CustomSmartPointer {
    data: String
}

impl CustomSmartPointer{
    fn new(data: &str) -> CustomSmartPointer {
        CustomSmartPointer { data: data.to_string() }
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping a smartpointer with data: {}", self.data)
    }
}

fn main() {
    let p1 = CustomSmartPointer::new("p1");
    std::mem::drop(p1); // 提早清理
    let p2 = CustomSmartPointer::new("p2");
    println!("All Custom smart pointer had created");

    let shared_data = Rc::new(RefCell::new(42));
    
    let clone1 = Rc::clone(&shared_data);
    let clone2 = Rc::clone(&shared_data);
    
    *clone1.borrow_mut() += 10;
    *clone2.borrow_mut() += 20;
    *shared_data.borrow_mut() += 28;
    
    println!("Final value: {}", *shared_data.borrow());
}