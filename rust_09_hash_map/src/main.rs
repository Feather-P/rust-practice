use std::collections::HashMap;

mod word_counter;
fn main() {
    let mut scores: HashMap<String, i32>= HashMap::new();

    // 向hashmap中写入数据
    scores.insert(String::from("Kai"), 30);
    scores.insert(String::from("Kikiwi"), 20);

    let name = String::from("Kai");
    let score = scores.get(&name).copied().unwrap_or(0);
    println!("{}",score);

    for (key, value) in &scores{
        let key_ref = key.as_str();
        println!("Key is: {key_ref} , Value is: {value}");
    }

    // 使用entry在假如存在已用键的情况下插入
    scores.entry("Kai".to_string()).or_insert(0);

    // 以下内容调用自己写的word_couonter
    use word_counter::WordCount;
    let mut my_counter: WordCount = WordCount::new();

    my_counter.add_sentence("I REALLY REALLY LIKE U");
    my_counter.add_single_word("YES");
    my_counter.print_all();


    let count = my_counter.get_count("YES");
    println!("The count of word \"YES\" is: {count}")
}
