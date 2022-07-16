use super::Item;

pub fn get_item_info(name: String, item_list: &[Item]) {
    for i in item_list {
        if name.eq(&i.name) {
            println!("name {}price: {}", i.name, i.price);
            return;
        }
    }
    println!("item does not exist.");
}
