fn main(){
	let p:f64=210_000.00;
	let r:f64=5.00;
	let t:f64=3.00;
	let a=p*(1.0+(r/100.0)).powf(t);
	println!("Amount is {}", a);
	
}
