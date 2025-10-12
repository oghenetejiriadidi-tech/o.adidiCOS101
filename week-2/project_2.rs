fn main() {
   let t:f64 = 450_000.0;
   let m:f64 =  1_500_000.0;
   let h:f64 = 750_000.0;
   let d:f64 = 2_850_000_000.0;
   let a:f64 = 250_000.0;
	
  let s = t + m + h + d + a;
  println!("Sum is {}", s);
  let av = s/10.0;
  println!("Average is {}", av)
   }