use std::io;

fn main () {

	for i in 0..100 {
		println!("\nPatient Record:{}", i + 1);
	}

	println!("\nEnter your name");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
   println!("Your name is {}",name);

}   
