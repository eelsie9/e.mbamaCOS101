struct Laptop {
    brand:String,
    price:u32
}

struct LaptopStore {
    laptops: Vec<Laptop>
}

impl LaptopStore {
    fn total_cost(&self, brand_quantity:usize)->u32 {
        self.laptops.iter().map(|laptop| laptop.price * brand_quantity as u32).sum()
    }

}

fn main() {
    let store = LaptopStore {
        laptops : vec! [
        Laptop {brand:String::from("HP"), price:650_000},
        Laptop {brand:String::from("IBM"), price:755_000},
        Laptop {brand:String::from("Toshiba"), price:550_000},
        Laptop {brand:String::from("Dell"), price:850_000}
    ],
};

    let brand_quantity = 3;
    let total = store.total_cost(brand_quantity);

    println!("The total cost for purchasing 3 laptops from each brand is N{}", total);
}
