use std::env;
use std::process;

fn main() {
  // Get console arguments
  let args: Vec<_> = env::args().collect();

  // Give info if input is not long enough
  if args.len() < 3 {
    println!("Provide the script with two integers!");
    process::exit(0);
  };
  
  // Assign console input to variables
  let x: i64 = args[1].parse().unwrap_or(0);
  let y: i64 = args[2].parse().unwrap_or(0);
  
  // Initialize the variables for the algorithm
  let mut larger;
  let mut smaller;
  let mut z;

  // Let input be in arbitrary order
  if x > y {
    larger = x;
    smaller = y;
  } else {
    larger = y;
    smaller = x;
  }

  // First print
  println!("The greatest common divisor of {} and {} is:", x, y);

  loop {
    // First things first. Get the remainder of larger / smaller
    z = larger % smaller;

    // Print the remainder to console with the calculation that produced it
    println!("{} mod {} = {}", larger, smaller, z);

    // Case of coprime integers
    if z == 1 {
      println!("----------------------------");
      
      println!("{}", 1);

      println!("The two numbers are coprime.");
      break;
    }

    // GCD found!
    if z == 0 {
      println!("----------------------------");
      
      println!("{}", smaller);

      println!("proof of x % z: {} % {} = {}", x, smaller, x % smaller);
      println!("proof of y % z: {} % {} = {}", y, smaller, y % smaller);
      break;
    }

    // If GCD not found this iteration, shift values accordingly.
    larger = smaller;
    smaller = z;
  }
}
