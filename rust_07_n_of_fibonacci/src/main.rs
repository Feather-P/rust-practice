fn main() {
    let mut n: [u32;2] = [0,1];
    let mut count: u32 = 1;
    let mut input = String::new();
    let mut next: u32 = n[0] + n [1];
    std::io::stdin().read_line(&mut input).expect("无法读取行");
    while count < input.trim().parse().expect("不是一个合法的数字"){
        if n[0] >= n[1]{
            next = n[0] + n [1];
            n[1] = next;
            count += 1;
            continue;
        }
        if n[1] > n[0]{
            next = n[0] + n [1];
            n[0] = next;
            count += 1;
            continue;
        }
    }
    println!("{next}");
}
