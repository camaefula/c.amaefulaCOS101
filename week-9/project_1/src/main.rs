use std::io::Write;

fn main() {
    let header = "Lager, Stout, Non-Alcholic\n";
    let row_1 = "33 Export,Desperados,Goldberg,Heinken,Star\n";
    let row_2 = "Legend,Turbo King,Williams\n";
    let row_3 = "Maltina,Amstel Malta,Malta Gold,Fayrouz\n";

    let mut file = std::fs::File::create("project.txt").expect("create failed");
    file.write_all("Welcome to Nigerian Breweries Plc\n"
        .as_bytes()).expect("Write failed");
    file.write_all(header.as_bytes()).expect("write failed ");
    file.write_all(row_1.as_bytes()).expect("write failed ");
    file.write_all(row_2.as_bytes()).expect("write failed ");
    file.write_all(row_3.as_bytes()).expect("write failed ");

}