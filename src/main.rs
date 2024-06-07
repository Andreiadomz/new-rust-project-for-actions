/*run all functions in lib.rs */

use trust::add;
use trust::divide;
use trust::multiply;
use trust::substract;

fn main() {
    println!("Hello, world!");
    println!("1 + 2 = {}", add(1, 2));
    println!("1 - 2 = {}", substract(1, 2));
    println!("1 * 2 = {}", multiply(1, 2));
    println!("1 / 2 = {}", divide(1, 2));
}
