use super::Item;

pub fn register_item(name: String, price: usize, product_cost: usize, item_list: &mut Vec<Item>) {
    let item = Item {
        name: name,
        price: price,
        production_cost: product_cost
    };

    item_list.push(item);
}