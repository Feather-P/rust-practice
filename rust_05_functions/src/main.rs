fn main() {
    println!("This is the first function");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("读取行失败");
    another_function(input.trim().parse().expect("输入的不是一个有效数字"));
}

fn another_function(n: i32){
    println!("This is another function.");
    println!("这是你输入的数字:{n}");
}