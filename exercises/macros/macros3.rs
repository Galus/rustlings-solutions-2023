// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a hint.

// https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files
#[macro_use] // not sure why the other method didnt work? 
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    // pub(crate) use my_macro;
}

fn main() {
    my_macro!();
}
