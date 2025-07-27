use std::{fs,fs::File, io::{self, Read}};

fn main() {
    fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
        let file_result = File::open(file_name);
        let mut user_name_file = match file_result{
            Ok(file) => file,
            Err(e   ) => return Err(e)
        };

        let mut user_name = String::new();

        match user_name_file.read_to_string(&mut user_name){
            Ok(_) => Ok(user_name),
            Err(e) => Err(e),
        }
    }

    fn read_username_from_file_easy1(file_name: &str) -> Result<String, io::Error> {
        let mut user_name = String::new();
        File::open(file_name)?.read_to_string(&mut user_name)?;
        Ok(user_name)
    }

    fn read_username_from_file_easy2(file_name: &str) -> Result<String, io::Error> {
        fs::read_to_string(&file_name)
        // The Easiest way
    }
}
