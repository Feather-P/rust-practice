use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file_name = "Ciallo.txt";
    let open_file_result = File::open(&file_name);

    let my_file = match open_file_result{
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&file_name) {
                Ok(fc) => fc,
                Err(error) => panic!("Something wrong when creating the file: {error}")
            },
            _ => {
                panic!("Problem when opening the file.")
            }
        }
    };
}
