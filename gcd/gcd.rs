use std::env;

fn main() {
  let args: Vec<_> = env::args().collect();

  let mut x = match args[1].parse::<i32>() {
    Ok(i) => i,
    Err(e) => {
      -1
    }
  };
  let mut y = match args[2].parse::<i32>() {
    Ok(i) => i,
    Err(e) => {
      -1
    }
  };
  let mut z;

  println!("The greatest common divisor of {} and {} is:", x, y);

  loop {
    z = if x > y {
      x % y
    } else {
      y % x
    };

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
