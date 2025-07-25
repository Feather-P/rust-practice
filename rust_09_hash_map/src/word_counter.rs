use std::collections::HashMap;

pub struct WordCount{
    word_hash: HashMap<String, u32>
}

impl WordCount {
    pub fn new() -> Self {
        Self{
            word_hash: HashMap::new()
        }
    }

    pub fn add_single_word(&mut self, target: &str) -> () {
        self.check_existance(target);
        self.word_hash.insert(target.to_string(), self.word_hash.get(target).copied().unwrap_or(0) + 1);
    }

    pub fn add_sentence(&mut self, target: &str) -> () {
        for word in target.split_whitespace() {
            self.check_existance(word);
            self.word_hash.insert(word.to_string(), self.word_hash.get(word).copied().unwrap_or(0) + 1);
        }
    }

    pub fn get_count(&self, target: &str) -> u32{
        *self.word_hash.get(target).unwrap()
    }

    pub fn print_all(&self) -> () {
        for (word, count) in &self.word_hash{
            println!("word: {word}, count: {count}");
        }
    }
    
    fn check_existance(&mut self, target: &str)  -> () {
        self.word_hash.entry(target.to_string()).or_insert(0);
    }
}