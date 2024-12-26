use std::io::Write;

fn main() {
    let name_of_commisioner = vec!["Aigbogun Alamba Daudu","Muritala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
    let geopolitical_zone = vec!["South west","North east","South south","South east"];

    let mut merged_doc = String::new();
     for i in merged_doc.as_bytes(){
        file.write_all(i.as_bytes()).expect("Failed to write");
     }
     let mut file = std::fs::File::create("EFCC.txt").expect("write failed");
     file.write_all(merged_doc.as_bytes()).expect("Write failed");
}
