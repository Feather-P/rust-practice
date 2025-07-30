use std::collections::HashMap;
use std::hash::Hash;

pub struct Inventory {
    items: Vec<Item>,
}

pub struct Item {
    type_stock: StockType,
    color: Option<Color>,
    size: Option<Size>
}

#[derive(PartialEq)]
enum StockType {
    Shirt,
    Hat
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Color {
    Red,
    Blue,
    Green
}

enum Size {
    L,
    M,
    S
}

impl Inventory {
    fn get_most_common_color(&self, t: StockType) -> Option<Color> {
        let mut count_map: HashMap<Color, u32> = HashMap::new();
        for item in &self.items{
            if item.color == None{
                continue;
            }
            if item.type_stock == t {
                let item_color = item.color.as_ref().unwrap();
                let plus = *count_map.get(item_color).unwrap_or(&0) + 1;
                count_map.insert(item.color.clone().unwrap(), plus) ;
            }
        }
        count_map
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(color, _)| color)
    }
}

#[cfg(test)]
mod tests{
    use crate::inventory::{Color, Inventory, Item, Size, StockType};

    #[test]
    fn test_get_most_common_color() {
        let mut my_inventory = Inventory{items: vec![]};
        my_inventory.items.push(
            Item{
                color: Some(Color::Blue),
                size: Some(Size::L),
                type_stock: StockType::Shirt,
            }
        );
        my_inventory.items.push(
            Item{
                color: Some(Color::Blue),
                size: Some(Size::L),
                type_stock: StockType::Shirt,
            }
        );
        my_inventory.items.push(
            Item{
                color: Some(Color::Green),
                size: Some(Size::L),
                type_stock: StockType::Shirt,
            }
        );

        assert_eq!(my_inventory.get_most_common_color(StockType::Shirt).unwrap(), Color::Blue)
    }
}