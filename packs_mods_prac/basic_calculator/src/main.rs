mod math;

pub use crate::math::*;

fn main() {
    let add_result = add(12, 24);
    println!("12 + 24 = {add_result}");

    let sub_result = sub(12, 24);
    println!("12 - 24 = {sub_result}");

    let mul_result = mul(12, 24);
    println!("12 * 24 = {mul_result}");

    let div_result = div(24.0, 12.0);
    println!("24 / 12 = {div_result}");

    let squared_number = algebra::square(12);
    println!("12 * 12 = {squared_number}");
}
