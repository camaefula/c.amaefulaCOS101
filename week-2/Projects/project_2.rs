fn main() {
	let toshiba:f64 = 450_000.00;
	let mac:f64= 1500_000.00;
	let hp:f64= 750_000.00;
	let dell:f64=2_850_000.00;
	let acer:f64=250_000.00;
	let qtytoshiba=2.0;
	let qtymac:f64=1.0;
	let qtyhp:f64=3.0;
	let qtydell:f64=3.0;
	let qtyacer:f64=1.0;
	let qty:f64=qtytoshiba+qtymac+qtyhp+qtyacer+qtydell;

	let s:f64 = (toshiba*qtytoshiba) +(mac*qtymac) +(hp*qtyhp) +(dell*qtydell) +(acer*qtyacer);
	let _a:f64 = s/qty;
	println!("The total sum is {} and the total amount is {}", s,_a)
}
