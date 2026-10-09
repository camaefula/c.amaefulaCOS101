use std::io;

fn main() {
    println!("===The Shape Calculator ===");
    println!("Select a shape/calculator:");
    println!("1. Area of trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Surface Area of Cube");
    println!("5. Volume of cylinder");

    let choice = read_number("Enter your choice (1-5):");

    match choice as u32 {
        1 => {
            let area = calculate_trapezium();
            println!("\nArea of Trapezium = {:.2}",area);
        }
        2 => {
            let area = calculate_rhombus();
            println!("\nArea of Rhombus ={:.2}", area);
        }
        3 => {
            let area = calculate_parallelogram();
            println!("\nArea of Parallelogram = {:.2}",area);
        }
        4 => {
            let surface_area = calculate_cube();
            println!("\nSurface Areaof Cube ={:.2}",surface_area);
         },
        5 => {
            let volume = calculate_cylinder();
            println!("\nVolume of Cylinder = {:.2}", volume);
        },
        _ => println!("Invalid choice! Please restart and pick a number between 1 and 5.")
    }
  }

// Helper function to handle user input repeatedly until a valid number is provided
fn read_number(prompt: &str) -> f64 {
loop {
    println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input);
            .expect("Failed to read line");

            match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

// Function 1: Trapezium Area
fn calculate_trapezium() -> f64 {
    let height = read_number("Enter height:");
    let base1 = read_number("Enter base 1:");
    let base2 = read_number("Enter base 2:");

    (height / 2.0) * (base1 + base2)
}

// Function 2: Rhombus Area
fn calculate_rhombus() -> f64 {
    let diagonal1 = read_number("Enter diagonal 1:");
    let diagonal2 = read_number("Enter diagonal 2:");

    0.5 * diagonal1 * diagonal2
}

// Function 3: Parallelogram Area
fn calculate_parallelogram() -> f64 {
    let base = read_number("Enter base:");
    let altitude = read_number("Enter altitude (height):");

    base * altitude
}

// Function 4: Cube Surface Area
fn calculate_cube() -> f64 {
    let side = read_number("Enter side length:");

    6.0 * side * side
}

// Function 5: Cylinder Volume
fn calculate_cylinder() -> f64 {
    let radius = read_number("Enter radius:");
    let height = read_number("Enter height:");

    std::f64::consts::PI * radius * radius * height
}