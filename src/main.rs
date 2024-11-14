fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("Hello World!");
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
    let info = os_info::get();
    println!("{} days", 31);
    // // Print full information:
    // println!("OS information: {info}");

    // // Print information separately:
    println!("Type: {}", info.os_type());
    // println!("Version: {}", info.version());
    // println!("Bitness: {}", info.bitness());
    match info.architecture() {
        Some(arch) => println!("Architecture: {}", arch),
        None => println!("Architecture: None"),
    }
}

pub fn public_function() {
    println!("called rary's `public_function()`");
    private_function()
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
