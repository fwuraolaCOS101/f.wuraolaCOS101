use std::io::Write;

fn main() {
    let name_of_commissioner = vec!["Aigbogun Alamba Daudu","Muritala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
    let geopolitical_zone = vec!["South west","North east","South south","South east","South west"];

    let mut combined_data = Vec::new();
    for i in 0..name_of_commissioner.len() {
        combined_data.push(format!("{}\t{}\t{}\t{}",i+1, name_of_commissioner[i],
            geopolitical_zone[i], ministry[i]));
    }

    let mut file = std::fs::File::create("merged_data.txt").expect("Write failed");
    file.write_all("SN\tNAME OF COMMISSIONER\tGEOPOLITICAL ZONE\tMINISTRY\n".as_bytes());

    for i in combined_data {
        file.write_all(format!("{}\n",i).as_bytes());
    }
    println!("Merged dataset formed");
}    