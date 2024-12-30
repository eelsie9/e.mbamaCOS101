use std::io;

fn main() {
    let mut names: Vec<String> = Vec::new();
    let mut years: Vec<u32> = Vec::new();
    let mut input1 = String::new();
    let mut highestindex = 0;
    let mut highestyear = 0;
    let mut morethan = false;

    println!("This is a program to determine the candidate with the highest years of programming experience.");
    println!("Enter the number of people interviewed");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let candidate_number:u32 = input1.trim().parse().expect("Invalid input");

    for i in 0..candidate_number
    {
        let mut input2 = String::new();
        let mut input3 = String::new();
        println!("Enter the name of applicant number {}", i+1);
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        names.push(input2.trim().to_string());

        let ii = i as usize;
        println!("Enter the number of years of programming experience {}, applicant number {} has", names[ii], i+1);
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let year:u32 = input3.trim().parse().expect("Invalid input");
        years.push(year);
    }

    for y in 0..years.len()
    {
        let index = y as usize;
        if y == 0
        {
            highestindex = y as usize;
            highestyear = years [0];
        }
        else
        {
            if years[index] > highestyear
            {
                highestindex = index as usize;
                highestyear = years[index];
            }
        }
    }
    let mut v = vec![names[highestindex].clone()];
    let mut v2 = vec![years[highestindex].clone()];

    for z in 0..years.len()
    {
        let index = z as usize;

        if years[index] == highestyear && highestindex != index
        {
            morethan = true;
            v.push(names[index].clone());
            v2.push(years[index].clone());
        }
    }

    println!("\n\n\nALL CANDIDATES");

    for i in 0..names.len()
    {
        let index = i as usize;

        println!("{}. Name of candidate: {} \nYears of programming experience: {}", i+1, names[index], years[index]);
    }

    if morethan
    {
        println!("\nThere are many candidates with the highest years of experience");
        println!("There are:");
        for a in 0..v.len()
        {
            let index = a as usize;
            println!("{}. Name of candidate: {} \nYears of programming experience: {}", a+1, v[index], v2[index]);
        }
    }
    else
    {
        println!("\nApplicant number {} has the highest number of years of programming experience. \nName of Applicant: {}\nYears of programming experience: {}", highestindex+1, names[highestindex], years[highestindex]);
    }
}
