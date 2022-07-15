use std::io;
use modulize::Item;
use modulize::sale;
use modulize::buy;

fn main() {
    let mut item_list: Vec<Item> = Vec::new();

    loop {
        let mut input = String::new();

        println!("Are you 1. sellor or 2. buyer? (input 1 or 2):");

        io::stdin().read_line(&mut input).expect("Fail to read");

        let input: u32 = input.trim().parse().expect("Please type a number!");

        if input == 1 {
            println!("Please register item.");
            let mut name = String::new();
            let mut price = String::new();
            let mut production_cost = String::new();

            io::stdin().read_line(&mut name).expect("Fail to read");
            io::stdin().read_line(&mut price).expect("Fail to read");
            io::stdin().read_line(&mut production_cost).expect("Fail to read");

            let price: usize = price.trim().parse().expect("Please type a number1!");
            let production_cost: usize = production_cost.trim().parse().expect("Please type a number2!");

            sale::register_item(name, price, production_cost, &mut item_list);
        } else if input == 2 {
            println!("Please input the name of item that you are finding.");

            let mut name = String::new();
            io::stdin().read_line(&mut name).expect("Fail to read");

            buy::get_item_info(name, &item_list);
        } else {
            println!("asdadf");
        }
    }
}
