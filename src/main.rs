/*
Error handling:

Rust does not have exceptions in the traditional sense; however, it uses the Result<T, E> enum for recoverable errors,
and the panic!() for unrecoverable errors.  The panic macro will cause the program to terminate immediately, while also
relaying to the user what went wrong.

In WSL, use the export RUST_BACKTRACE=1 command in the terminal to see the backtrace of the error message.
In Powershell, use $Env:RUST_BACKTRACE=1 to do the same thing.
*/

use core::panic;

fn main() {
    //panic!("Houston, we have a problem!"); // exit code 101 indcates that the code panicked.
    let countdown = [5, 4, 3, 2, 1, 0];
    for count in countdown {
        println!("T-minus {}", count);
        let x = 1 / count; // This won't end well
    }
}
