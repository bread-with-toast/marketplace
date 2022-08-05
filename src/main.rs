use std::{thread, time};

struct Item {
    name: String,
    description: String,
    rating: i8,
    category: Categories,
    price: f32,
    owned: bool,
    money: f32
}

enum Categories {
    Toy,
    Office,
    Gaming,
    Clothing,
    Vehicle,
    Gadget,
    Other
}

fn sleep(s: u64) {
    thread::sleep(time::Duration::from_secs(s));
}

impl Item {
    fn show_item(&self) {
        println!("Name: {}", self.name);
        sleep(2);
        println!("Description: {}", self.description);
        sleep(2);
        println!("Rating: {}", self.rating);
        sleep(2);

        match self.category {
            Categories::Toy => println!("Category: Toys"),
            Categories::Office => println!("Category: Office"),
            Categories::Gaming => println!("Category: Gaming"),
            Categories::Clothing => println!("Category: Clothing"),
            Categories::Vehicle => println!("Category: Vehicle"),
            Categories::Gadget => println!("Category: Gadget"),
            Categories::Other => println!("Category: Other")
        }

        sleep(2);
        println!("Price: {}", self.price);
    }

    fn buy_item(&mut self) {
        if self.owned {
            println!("Item is already owned!");
        } else if self.owned == false && self.money >= self.price {
            println!("Item bought!");
            self.money = self.money - self.price;
        } else {
            println!("Cannot purchase item! Item costs {} and you only have {}!", self.price, self.money);
        }
    }

    fn sell_item(&mut self) {
        if self.owned {
            self.owned = false;
            self.money = self.money + self.price;

            println!("Item sold!");
            println!("You now have {}", self.money);
        } else {
            println!("You do not own this item!");
        }
    }
}

fn main() {
    let mut my_first_item = Item {
        name: "Item A".to_string(),
        description: "This is a test item".to_string(),
        rating: 3,
        category: Categories::Other,
        price: 32.49,
        owned: false,
        money: 29.93
    };

    my_first_item.sell_item();
}
