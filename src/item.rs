use rand::Rng;

#[derive(Debug)]
pub struct Item {
    id: i32,
    name: String,
    price: f32,
    quantity: i32,
    categorie: Category,
}

impl Item {
    pub fn create_item(name: &str, price: f32, quantity: i32, categorie: Category) -> Item {
        let mut rng = rand::thread_rng();
        let id = rng.gen::<i32>();
        Item {
            id,
            name: name.to_string(),
            price,
            quantity,
            categorie,
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_price(&self) -> f32 {
        self.price
    }

    pub fn get_quantity(&self) -> i32 {
        self.quantity
    }

    pub fn get_categorie(&self) -> String {
        match self.categorie {
            Category::Drink => "Drink".to_string(),
            Category::Electronic => "Electronic".to_string(),
            Category::Food => "Food".to_string(),
            Category::Home => "Home".to_string(),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = (*name).to_string();
    }

    pub fn set_price(&mut self, price: &f32) {
        self.price = *price;
    }

    pub fn set_quantity(&mut self, quantity: &i32) {
        self.quantity = *quantity;
    }

    pub fn set_categorie(&mut self, categorie: &Category){
        self.categorie = *categorie;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Category {
    Electronic,
    Home,
    Food,
    Drink,
}
