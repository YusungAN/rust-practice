use super::Item;

pub fn register_item(name: String, price: usize, product_cost: usize, item_list: &mut Vec<Item>) {
    for i in item_list.as_slice() {
        if name.eq(&i.name) {
            println!("already exist item.");
            return;
        }
    }
    
    let item = Item {
        name: name,
        price: price,
        production_cost: product_cost
    };

    item_list.push(item);
}
