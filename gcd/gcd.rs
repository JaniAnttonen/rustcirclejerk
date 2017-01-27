fn main() {
  let mut x = 310;
  let mut y = 5;
  let mut z;

  println!("The greatest common divisor of {} and {} is:", x, y);

  loop {
    z = y % x;
    if z == 0 {
      println!("{}", x);
      break;
    }
    x = y;
    y = z;
  }
}
