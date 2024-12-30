use std::io;

fn main () {
	let mut count = 0;
	

	while count =< 50 {
		count+=1;

		if count = 50 {
			break;
		}

    }

	println!("\nEnter your name");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
   println!("Your name is {}",name);

}   
