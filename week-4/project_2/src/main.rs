use std::io;

fn main() {
    println!("Enter employee experience (yes/no):");
    let mut experience =String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience =experience.trim().to_lowercase();
    let experience =experience.trim().to_lowercase();
    println!("Enter employee age:");
    let mut input_age =String::new();
    io::stdin().read_line(&mut input_age).expect("Failed to read age");
    let age:i32 =input_age.trim().parse().expect("Please enter a valid age");
    if experience =="yes" && age >=40
    {
        println!("Annual incentives:N1_560_000.00");
    } else if experience =="yes" && age <28{
        println!("Annual incentives:N1_300_000.00");
    } else if experience =="no" {
        println!("Annual incentives:N100_00.00");
    } else {
        println!("No incentives category found.");
    }
}