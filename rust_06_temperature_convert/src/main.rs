fn main() {
    let mut mode = String::new();
    let mut temp = String::new();

    println!("这是一个华氏度/摄氏 度温度转换器，输入0从摄氏度转换到华氏度，输入1从华氏度转换为摄氏度");
    std::io::stdin()
    .read_line(&mut mode)
    .expect("无法读取行");

    let mode: u8 = mode.trim().parse().expect("输入的不是一个有效布尔值");

    println!("输入温度");

    std::io::stdin()
    .read_line(&mut temp)
    .expect("无法读取温度输入");

    if mode == 0{
        let temp: f64 = temp.trim().parse().expect("输入的温度不是合法浮点数");
        let ans = convert_to_fahrenheit(temp).to_string();
        println!("转换为华氏度: {ans} °F");

    }
    if mode == 1{
        let temp: f64 =temp.trim().parse().expect("输入的温度不是合法浮点数");
        let ans = convert_to_celsius(temp).to_string();
        println!("转换为摄氏度: {ans} °C");
    }
}

fn convert_to_fahrenheit(c: f64) -> f64{
    let res = c * 1.8 + 32.0;
    return res;
}

fn convert_to_celsius(c: f64) -> f64{
    let res = (c - 32.0) / 1.8;
    return res;
}
