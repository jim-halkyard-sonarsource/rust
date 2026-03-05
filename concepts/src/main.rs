fn main() {
    //Variables
    
    // Shadowing
    // Declare a variable
    let x = 5;
    // Shaodow it
    let x = x + 1;

    {
        // Declare again, this shadowing the shadow
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    // Back in the outer scope the value is the first shadow
    println!("The value of x is: {x}");

    // Create a string of spaces
    let spaces = "   ";
    // Shadow that string of spaces with a number - different type
    let spaces = spaces.len();
    println!("We have {spaces} spaces");

    // Data types
    //
    // Integers
    //
    // Length	Signed	Unsigned
    // 8-bit	i8		u8
    // 16-bit	i16		u16
    // 32-bit	i32		u32
    // 64-bit	i64		u64
    // 128-bit	i128	u128

    // Default int is i32

    let x = 0xff;
    println!("x is {x}");

    // Floating-point
    // f32 and f64
    // f64 is the default

    let x = 2.1; // f64    
    let y: f32 = 3.2; // f32
    println!("x is {x}, y is {y}");

    // Numeric operations
    // addition
    let sum = 5 + 10;
    println!("sum of 5+10 is {sum}");
    
    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference of 95.5 - 4.3 is {difference}");

    // multiplication
    let product = 4 * 30;
    println!("product of 4 * 20 is {product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -8 / 3; // Results in -2
    let truncfp = -8.0 / 3.0; // Results in 2.666665!!!!

    println!("result of 56.7 / 32.2 is {quotient}");
    println!("truncated of -8 / 3 is {truncated}");
    println!("truncfp of -8.0 / 3.0 is {truncfp}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder of 43 % 5 is {remainder}");
}
