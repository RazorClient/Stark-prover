pub mod element; 
pub use element::FieldElement;


// fn main() {
//     const MODULUS: u64 = 7;

//     let a = FieldElement::<MODULUS>::new(3);
//     let b = FieldElement::<MODULUS>::new(5);

//     // Addition
//     let sum = a + b;
//     println!("Sum: {}", sum.value()); // Output: 1

//     // Subtraction
//     let diff = a - b;
//     println!("Difference: {}", diff.value()); // Output: 5

//     // Multiplication
//     let product = a * b;
//     println!("Product: {}", product.value()); // Output: 1

//     // Division
//     let quotient = a / b;
//     println!("Quotient: {}", quotient.value()); // Output: 2

//     // Power
//     let pow_result = a.pow(3);
//     println!("3^3 mod 7: {}", pow_result.value()); // Output: 6

//     // Inverse
//     let inv = b.inverse();
//     println!("Inverse of 5 mod 7: {}", inv.value()); // Output: 3

//     // Negation
//     let neg = -a;
//     println!("Negation of 3 mod 7: {}", neg.value()); // Output: 4
// }
