use std::io;
/** 
 * @param n the index to find
 * @param nm1 the current fibonacci number
 * @param nm2 the previous fibonacci number
 * @param index the index which we are currently iterating
*/
fn get_nth_fibonacci(n: i128, nm1: i128, nm2: i128, index: i128 ) -> i128 {
    if index == n {
        nm1 + nm2
    } else {
        get_nth_fibonacci(n, nm2 + nm1, nm1, index + 1)
    }
}

fn main() {
    println!("\rWhat index of the fibonacci sequence would you like to find?");
    let mut nth_index = String::new();
    io::stdin().read_line(&mut nth_index).expect("\rSomething went wrong\r");
    let test = &nth_index.trim().parse::<i128>();
    match test {
        Ok(ok) => println!("\rThe number is: {}\r", get_nth_fibonacci(*ok, 1, 0, 0)),
        Err(e) => println!("\rEnter an integer: {}\r", e)
    }
    println!();
    
}
