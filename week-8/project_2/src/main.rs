fn main() {
    
    println!("EY  Nigeria Interview Portal");

    //initializing a  multiple Tuple
    let name_years = ("Augustine",1,"Anthony",2,"Chisom",3);

    println!("Original tuple = {:?}",name_years);


    // Destructure the tuple directly into `name` and `years`
    let (name,years) = ("Yemi",20);
    println!("Name is {} , Years of experience is {}",name,years);
}