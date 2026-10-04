// Rust progran to create the restaurant menu
    
use std::io;
fn main () {  
println!("============================");
println!("      WELCOME TO GAB`S RESTAURANT");
println!("Hello! 👋 Welcome to our restaurant");
println!("Take a look at our menu");
println!("============== MENU");
println!("P - Pounded Yam /Edikaiko soup - N3_200");
println!("F - Fried Rice & Chicken  -N3_000");
println!("A  - Amala & Ewedu soup - N2_500");
println!("E - Eba & Egusi soup - N2_000"); 
println!("W - White Rice & Stem - N2_500");
println!("=======================");

println!("Enter food type:");

let mut food_type = String::new();
io::stdin().read_line(&mut food_type).unwrap();
let food_type =food_type.trim().to_uppercase();
let (food_name,price) = match food_type.as_str() {
	"P"  =>("Pounded Yam / Edikaiko soup",3200_0),
	"F"  =>("Fried Rice & Chicken", 3000_0),
	"A"  =>("Amala & Ewedu soup",2500_0),
	"E"  =>("Eba & Egusi soup",2000_0),
	"W"  =>("White Rice & Stem",2500_0),
	      _=> {
	      	println!("Sorry,that food code is not valid.");
	      	println!("Please try again.");
	      	   return;
	      }
};


println!("Great choice! ");
println!("Please enter the quantity:");

let mut quantity_input =String::new();
io::stdin().read_line(&mut quantity_input);

let quantity: f64 = match quantity_input.trim().parse() {
	Ok (value)  => value,
	Err(_) => {
		println!("Sorry, that is not a valid quantity.");
		return;
	}
};
 let total: f64 = (price as f64) * quantity;
  println!("\n================================ YOUR ORDER =============");
  println!("Food: {}", food_name);
  println!("Quantity: {}", quantity);
  println!("Total: N{ :.2}",total);

  if total >10000.0 {
  	let discount =total * 0.05;
  	let final_amount = total - discount;

  	println!("5% Discount: N{:.2}",discount);
  	println!("Amount to Pay:N{:.2}",final_amount);
  }else {
  	println!("No discount applied.");
  	println!("Amount to Pay: N{:.2}",total);
  }
  println!("====================================================");
  println!("Thank you  for placing order");

  println!("Bon Appetite");
  println!("Thank you for choosing GAB`S RESTAURANT 🥰");
  }

