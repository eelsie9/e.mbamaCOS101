fn main() {

    let mut info: Vec<String> = Vec::new();
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut index:usize = 0;
    let mut title = " ";
    let publicservant_lvl = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
    let officeadmin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement", "Classroom Teacher", "Snr Teacher", "Leading Teacher", "Deputy Principal", "Principal"];
    let mut psl = " ";

    println!("Welcome to the Public Server APS Level Checker.");
    println!("Input the type of Public Servant you are. (Office Administrator, Academic, Lawyer or Teacher");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    info.push(input1.trim().to_string());

    println!("How many years of experience do you have?");
    std::io::stdin().read_line(&mut input2).expect("Failed to read input");
    info.push(input2.trim().to_string());

    let years:f32 = info[1].trim().parse().expect("Invalid Input");
    let occupation = info[0].trim().to_lowercase();

    if years >= 1.0 && years <= 2.0 {
        index = 0;
    }
    else if years >= 3.0 && years <= 5.0 {
        index = 1;
    }
    else if years > 5.0 && years <= 8.0 {
        index = 2;
    }
    else if years > 8.0 && years <= 10.0 {
        index = 3;
    }
    else if years > 10.0 && years <= 13.0 {
        index = 4;
    }
    else if years > 13.0 {
        index = 5;
    }
    else {
        println!("ERROR - Invalid input");
    }

    if occupation == "office administrator" {
        title = officeadmin[index];
        psl = publicservant_lvl[index];
    }
    else if occupation == "academic" {
        title = academic[index];
        psl = publicservant_lvl[index];
    }
    else if occupation == "lawyer" {
        title = lawyer[index];
        psl = publicservant_lvl[index];
    }
    else if occupation == "teacher" {
        title = teacher[index];
        psl = publicservant_lvl[index];
    }

    println!("Occupation: {}\nYears of experience: {}\nTitle: {}\nPublic Servant Level: {}", occupation, years, title, psl);
}
