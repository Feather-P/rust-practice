use::std::io::stdin;

fn main() {
    let a: [u32 ; 6] = [1,2,3,4,5,6];

    println!("请输入一个想查找的数组index");

    let mut index = String::new();

    stdin()
    .read_line(&mut index)
    .expect("读取行失败");

    let index: usize = index.trim().parse().expect("输入的不是一个有效数字");

    let target = a[index];

    println!("你想查找的数为:{target}")
}
