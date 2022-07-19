// Bring complex type into scope.
use num::complex::Complex;

fn main() {
  // Using literal syntax.
  let a = Complex { re: 2.1, im: -1.2 };

  // Most types implement a new() method.
  // Rust does not have constructors.
  let b = Complex::new(11.1, 22.2);

  let result = a + b;
  // Access fields using dot notation.
  println!("{} + {}𝑖", result.re, result.im)
}
