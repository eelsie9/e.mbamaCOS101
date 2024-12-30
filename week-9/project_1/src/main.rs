use std::io::Write;
use std::io::Read;

fn main() {
    let lager = vec!["33 Export", "Desperados", "Goldberg", "Guilder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo King", "Williams"];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    write_to_file(&lager, &stout, &non_alcoholic);
    let mut read = String::new();
    let mut files = std::fs::File::open("drinks.txt").expect("failed to open");
    files.read_to_string(&mut read).expect("read failed");
    println!("{}", read);
}

fn write_to_file(lager: &[&str], stout: &[&str], non_alcoholic: &[&str]) {
    let mut file = std::fs::File::create("drinks.txt").expect("file creation failed");
    
    file.write_all("Lager:".as_bytes()).expect("write failed");

    for beer in lager {
        file.write_all(beer.as_bytes()).expect("write failed");
        file.write_all(", ".as_bytes()).expect("write failed");
    }

    file.write_all("\n\nStout: ".as_bytes()).expect("write failed");

    for beer in stout {
        file.write_all(beer.as_bytes()).expect("write failed");
        file.write_all(", ".as_bytes()).expect("write failed");
    }

    file.write_all("\n\nNon-Alcoholic:".as_bytes()).expect("write failed");

    for beer in non_alcoholic {
        file.write_all(beer.as_bytes()).expect("write failed");
        file.write_all(", ".as_bytes()).expect("write failed");
    }
}
