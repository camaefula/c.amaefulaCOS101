use std::io::Write;

fn main(){
    let header ="Commisioner,Ministry,Zone";
    let row_1 ="Aigbogun Alamade Daudu,Murtala Afeez Bendu,Okorocha Calistus OGbona,Adewale Jimoh Akaanbi,Osazuwa Faithe Etieye";
    let row_2 ="Internal Affairs,Justice,Defence,Power & Steel,Power & Steel,Petroleum";
    let row_3 ="South West,North East,South South,South West,South East";

    let mut file = std::fs::File::create("project.csv").expect("create failed");
    file.write_all("PAU-SMIS records\n"
        .as_bytes()).expect("write failed");
    file.write_all(header.as_bytes()).expect("write failed");
    file.write_all(row_1.as_bytes()).expect("write failed");
    file.write_all(row_2.as_bytes()).expect("write failed");
    file.write_all(row_3.as_bytes()).expect("write failed");
    println!("\nWeldone babe" );    
    
}
