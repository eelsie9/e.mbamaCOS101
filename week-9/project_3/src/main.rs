use std::io::Write;

fn main() {

    let commisioner_name = vec!["\nAigbogugn Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs", "     Justice", "Defense", "  Power & Steel", "   Petroleum"];
    let geo_zone = vec!["  South West", "         North East", "         South South", "     South West", "        South East"];

    let mut file = std::fs::File::create("convicted.ministers.txt").expect("failed to create");

    file.write_all("                     CONVICTED MINISTERS".as_bytes()).expect("write failed");
    file.write_all("\n\nNAME OF COMMISIONER         MINISTRY         GEOPOLITICAL ZONE".as_bytes()).expect("write failed");

    for i in 0..geo_zone.len() {
        let index = i as usize;
        file.write_all(commisioner_name[index].as_bytes()).expect("write failed");
        file.write_all("    ".as_bytes()).expect("write failed");
        file.write_all(ministry[index].as_bytes()).expect("write failed");
        file.write_all("    ".as_bytes()).expect("write failed");
        file.write_all(geo_zone[index].as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }
}
