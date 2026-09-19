// Rust program to find the roots of a quadratic equation

use std::io;

fn main(){
    println!("Enter value for a:");
    let mut input1 =String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f64=input1.trim().parse().expect("Failed to read input1");

    println!("Enter value for b:");
    let mut input1 =String::new();
    io::stdin().read_line(&mut input1).expect("Failed to input");
    let b:f64=input1.trim().parse().expect("Failed to read input1");

    println!("Enter value for c:");
    let mut input1 =String::new();
    io::stdin().read_line(&mut input1).expect("Failed to input1");
    let c:f64=input1().trim().parse().expect("Failed to input1");

    let d=b*b - 4.0*a*c;
     if d ==0.0{
        let root1 =(-b + d.sqrt())/(2.0*a);
        let root2 =(-b -d.sqrt())/(2.0*a);
        println!("d is greater than zero:two distinct real roots");
        println!("Root 1:{}",root1);
       println!("Root 2 ={}",root2);
     } else if d==0.0{
        let root =-b/(2.0*a);
        println!("d is equal to zer: exactlyone real root");
        println!("Root: {}",root);
     } else {
         println!("d is less than zero:no real roots");
     }

    }