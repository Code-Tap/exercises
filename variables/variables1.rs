// variables1.rs
// Make me compile! Execute the command `rustlings hint variables1` if you want a hint :)

// About this `I AM NOT DONE` thing:
// We sometimes encourage you to keep trying things on a given exercise,
// even after you already figured it out. If you got everything working and
// feel ready for the next exercise, remove the `I AM NOT DONE` comment below.

fn main() {
    let mut x = 5;
    const y: u16 = 6;

    println!("y has the value {}", y);
    println!("x has the value {}", x);
    x = 10;
    println!("x has the value {}", x);


    let spaces = "   ";
    let spaces = spaces.len(); // shaddow function
    println!("There are {} spaces", spaces)
}
