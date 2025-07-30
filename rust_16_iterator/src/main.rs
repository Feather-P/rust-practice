#[derive(PartialEq, Debug)]
enum ShoeSize {
    Size36 = 36,
    Size37 = 37,
    Size38 = 38,
    Size39 = 39,
    Size40 = 40
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: ShoeSize,
    style: String
}

 fn shoes_in_size(source: Vec<Shoe>, size: ShoeSize) -> Vec<Shoe> {
        source.into_iter().filter(|x| x.size == size).collect()
    }

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_shoes_in_size() {
        let shoes = vec![
            Shoe{
                size: ShoeSize::Size36,
                style: String::from("plain")
            },
            Shoe{
                size: ShoeSize::Size38,
                style: String::from("children")
            },
            Shoe{
                size: ShoeSize::Size40,
                style: String::from("Ciallo~")
            }
        ];

        let shoes_filtered = shoes_in_size(shoes, ShoeSize::Size36);
        assert_eq!(
            shoes_filtered,
            vec![
                Shoe{
                size: ShoeSize::Size36,
                style: String::from("plain")
                }
            ]
        )
    }
}
