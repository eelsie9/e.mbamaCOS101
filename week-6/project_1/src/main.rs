use std::io;

fn main() {
    println!("Welcome to our restaurant. Take a look at our menu \nMenu

        P = Poundo Yam and Edinkaiko Soup - N3,200.00
        F = Fried Rice and Chicken        - N3,000.00
        A = Amala and Ewedu Soup          - N2,500.00
        E = Eba and Egusi Soup            - N2,000.00
        W = White Rice and Stew           - N2,500.00
        ");

        let menu = [
            ("P", "Poundo Yam & Edinkaiko Soup", 3_200.00),
            ("F", "Fried Rice and Chicken", 3_000.00),
            ("A", "Amala and Ewedu Soup", 2_500.00),
            ("E", "Eba and Egusi Soup", 2_000.00),
            ("W", "White Rice and Stew", 2_500.00),

        ];

        let mut total_amount: f32 = 0.0;

        loop {
            println!("\n Make your order here; input (P for Poundo Yam and Edinkaiko Soup, \nF for Fried Rice and Chicken, \nA for Amala and Ewedu Soup, \nE for Eba and Egusi Soup, \nW for White Rice and Stew):");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input.");
            let choice = input.trim().to_uppercase();

            let cost = menu.iter().find(|&&(code, _, _)| code == choice);
            if let Some(&(_, name, price)) = cost {
                println!("You selected: {} N {}.", name, price);

                println!("\nHow many would you have?");
                let mut input2 = String::new();
                io::stdin().read_line(&mut input2).expect("Enter the quantity you want");
                let number: f32 = match input2.trim().parse(){
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid quantity. Please enter a valid number.");
                        continue;
                    }
                };
                total_amount += number * price;

                println!("Your current balance is: {}", total_amount);
                println!("\nDo you want to continue? Yes or no.");
                let mut input3 = String::new();
                io::stdin().read_line(&mut input3).expect("Invalid input. Please type [yes] or [no]");
                let answer: bool = match input3.trim().to_lowercase().as_str(){
                    "yes" => true,
                    "no" => false,
                    _=> {
                        println!("Invalid input; put in either yes or no.");
                        return;
                    }
                };
                if answer == false {
                    break;
                }
            }
        }    
}
