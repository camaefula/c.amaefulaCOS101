use std::io::Write;

fn main(){
    let header ="Student Name,Matric Number,Department,Level";
    let row_1 ="Oluchi Mordi,Adams Aliyu,Shanla BOlade,Adekune Gold,Blanca Edemah";
    let row_2 ="ACC10211111,ECO10110101,CSC10328828,EEE11020202,MEE10202001";
    let row_3 ="Accounting,Economics,Computer,Electrical,Mechanical";
    let row_4 ="300,100,200,200,100";

    let mut file = std::fs::File::create("project.csv").expect("create failed");
    file.write_all("PAU-SMIS records\n"
        .as_bytes()).expect("write failed");
    file.write_all(header.as_bytes()).expect("write failed");
    file.write_all(row_1.as_bytes()).expect("write failed");
    file.write_all(row_2.as_bytes()).expect("write failed");
    file.write_all(row_3.as_bytes()).expect("write failed");
    file.write_all(row_4.as_bytes()).expect("write failed");
    println!("\nData's looking nice." );    
    
}
