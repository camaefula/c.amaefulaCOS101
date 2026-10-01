fn main() {
 let  fullname = "Chibudun John Umeh";
 let  department = "Computer Science";
 let  uni = "Pan-Atlantic University";

 let mut school = "School of Science".to_string();
 // push string
 school.push_str(" and Technology") ;
 // check length
 println!("The Length my fullname is: {}",fullname.len());
 println!("I am a student of {} Department",department);
 println!("{}",school);
 println!("{}",uni);  
}
