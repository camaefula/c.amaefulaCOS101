struct Laptop{      
       brand:String,
       quantity:i32,
       price_each:f64,                      //underscore used as space
}

fn main() {
    //initialize a structure
println!("Alaba_Electronics");

    let a1 = Laptop {
        brand:String::from("HP"),
        quantity:10,
        price_each:650_000.00                                   //underscore used as space
    };
    let a2 = Laptop {
        brand:String::from("IBM"),
        quantity:6,
        price_each:755_000.00                  //underscore used as space
};
let a3 = Laptop {
    brand:String::from("Toshiba"),
    quantity:10,
    price_each:550_000.00                  //underscore used as space
};
let a4 = Laptop {
    brand:String::from("Dell"),
    quantity:4,
    price_each:850_000.00           //underscore used as space          
};
      // Declare all 'let' be lowercase
      let laptop_to_purchase:i32=3;                
// total cost of purchasing 3 laptops from each brand
   let hp_cost = a1.cost(laptop_to_purchase);
   let ibm_cost =a2.cost(laptop_to_purchase);
   let toshiba_cost = a3.cost(laptop_to_purchase);
   let dell_cost = a4.cost(laptop_to_purchase);

//calculate overall cost across all four brands
   let overall_total = hp_cost + ibm_cost + toshiba_cost + dell_cost;

   println!("Cost for 3 HP Laptop: {}",hp_cost);
   println!("Cost for 3 IBM Laptop: {}",ibm_cost);
   println!("Cost for 3 Toshiba Laptop: {}",toshiba_cost);
   println!("Cost for 3 Dell Laptop: {}",dell_cost);
}

impl Laptop{
// A method to calculate the cost if a customer buys 'num_items'
    fn cost(&self,num_items:i32)->f64 {
        //use the . operator to fetch the value of a field via the self keyword
        self.price_each * (num_items as f64)
    } 
}
