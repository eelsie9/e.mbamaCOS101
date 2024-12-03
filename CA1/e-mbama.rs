use std::io;

fn main(){

	println!("How many siblings do you have?");
	let mut sibling_number = String::new();
    io::stdin().read_line(&mut sibling_number).expect("Failed to read input");
    let sibling_number:i32 = sibling_number.trim().parse().expect("Input is invalid");

for i in 0..sibling_number {
	println!("Enter first name");
	let mut first_name = String::new();
    io::stdin().read_line(&mut first_name).expect("Failed to read input");
    
    println!("Enter age");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:i32 = age.trim().parse().expect("Input is invalid");

    println!("Enter gender");
    let mut gender = String::new();
    io::stdin().read_line(&mut gender).expect("Failed to read input");
    let gender = gender.trim().to_lowercase();
   
    println!("Enter country of residence");
    let mut resident_country = String::new();
    io::stdin().read_line(&mut resident_country).expect("Failed to read input");

    println!("Are you married?");
    let mut marriage = String::new();
    io::stdin().read_line(&mut first_name).expect("Failed to read input");
    let marriage = marriage.trim().to_lowercase();
    
    let marriage:bool = if marriage { "yes" } .
        true
    } else {
        false
    };

    if marriage == true {
    	
    	println!("Do you have any children?");
    	let mut children = String::new();
    io::stdin().read_line(&mut children).expect("Failed to read input");
    let children = children.trim().to_lowercase();
    if children == "yes" {

    	println!("Enter number of children");
    	let mut children_number = String::new();
    io::stdin().read_line(&mut children_number).expect("Failed to read input");
    let children_number:i32 = children_number.trim().parse().expect("Input is invalid");

     for i in 0..children_number {
     	println!("Enter child name");
     	let mut child_name = String::new();
    io::stdin().read_line(&mut child_name).expect("Failed to read input");

    println!("Enter child age");
    let mut child_age = String::new();
    io::stdin().read_line(&mut child_age).expect("Failed to read input");
    let child_age:i32 = child_age.trim().parse().expect("Input is invalid");

    println!("Enter daycare or school name");
     	let mut school_name = String::new();
    io::stdin().read_line(&mut school_name).expect("Failed to read input");

    } else if marriage == false {

	println!("Are you a student or employed?");
	let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read input");
    if answer = employed {

    	println!("Is the job remote, on-site, or hybrid?");
    		let mut answer2 = String::new();
    io::stdin().read_line(&mut answer2).expect("Failed to read input");
    } else if answer = student {

    	println!("Enter university");
    	let mut uni = String::new();
    io::stdin().read_line(&mut uni).expect("Failed to read input");

    println!("Enter course of study");
    let mut course = String::new();
    io::stdin().read_line(&mut course).expect("Failed to read input");

    println!("Enter year of study");
    let mut year = String::new();
    io::stdin().read_line(&mut year).expect("Failed to read input");
    let year:i32 = year.trim().parse().expect("Input is invalid");
    } 

     } 
    else {
     	println!("Invalid input");

    } 
    else {
     	println!("Invalid input");

}



}



    

    }

    

     


     	
     }


     

   
   


