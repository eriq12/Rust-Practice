// set constant with type
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // mut allows variable mutability (can change value), type is constant (type declarative)
    let mut x : i8 = 5;
    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");
    let spaces : String = String::from("    "); // can do "    ".to_string();
    let amt_spaces: usize = spaces.len(); // usize is hardware dependent (64 if 64-bit arch, 32 on 32-bit)
    println!("{spaces}There were {amt_spaces} spaces before this sentence.");
    println!("There are {THREE_HOURS_IN_SECONDS} seconds in three hours");
    // some types:
    // integer types: signed(i) or unsigned(u)
    //   size/length: 8, 16, 32, 64, 128, size(arch dependent like stated earlier)
    // floating-point types: f32, f64
    // character type: char
    // compound types
    let tup : (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The values are {x} {y} {z}");
}
