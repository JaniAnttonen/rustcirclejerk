use std::env;

fn main() {
  let mut args: Vec<_> = env::args().collect();

  if args.len() < 3 {
    args.push("310".to_string());
    args.push("140".to_string());
  };
  
  let mut x = args[1].parse().unwrap_or(0);;
  let mut y = args[2].parse().unwrap_or(0);;
  let mut z;

  println!("The greatest common divisor of {} and {} is:", x, y);

  loop {
    z = if x > y {
      x % y
    } else {
      y % x
    };

    println!("{}", z);
    if z == 1 {
      println!("{}", 1);
      break;
    }
    if z == 0 {
      println!("{}", x);
      break;
    }

    x = y;
    y = z;
  }
}
