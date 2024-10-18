use crate::item::Item;

pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn add_new_item(&mut self, new_item: Item) {
        self.items.push(new_item);
    }
}
