use inventory::Inventory;

mod inventory;
mod item;

fn main() {
    let mut inventory = Inventory { items: Vec::new() };
    let new_item = item::Item::create_item("Glace", 6.50, 12, item::Category::Drink);
    inventory.add_new_item(new_item);

    println!("{:?}", inventory.items[0].get_categorie());
    inventory.items[0].set_categorie(&item::Category::Food);
    println!("new categorie => {:?}", inventory.items[0].get_categorie());
}
