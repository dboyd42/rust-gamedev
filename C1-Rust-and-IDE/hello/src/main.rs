#![warn(clippy::all, clippy::pedantic)]
fn main() {
    // println!("Hello, world!");
    let my_list = [ "One", "Two", "Three" ];
    // for i in 0..3 {
    for item in &my_list {
        println!("{}", item);
    }
}
