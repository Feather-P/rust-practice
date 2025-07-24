use std::{cmp::Ordering, io, process};
use rand::Rng;

fn main() {
    println!("来猜个数");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("秘密数字是：{secret_number}");
    
    loop{
        println!("输入你的数字：");
    
        let mut num = String::new();

        io::stdin()
        .read_line(&mut num)
        .expect("读取失败");

        if num.trim().eq("Q") || num.trim().eq("quit") { process::exit(0); }

        println!("你猜的是: {num}");

        let num: u32 = match num.trim().parse() {
            Ok(res) => res,
            Err(_) => continue,
        };

        match num.cmp(&secret_number) {
            Ordering::Less => println! ("太小！"),
            Ordering::Greater => println! ("太大！"),
            Ordering::Equal =>{
                println! ("你赢了！");
                break;
            } 
        }
    }
    
}
