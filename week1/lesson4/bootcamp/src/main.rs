fn main() {
    println!("My nice welcome message");
    fizz_buzz(false);
    fizz2();
}

fn fizz2(){
    let mut num_fizzbuzzes = 0;
    for value in 1..302 {
        match (value % 3, value % 5) {
            (0,0) => num_fizzbuzzes += 1, 
            (0, _) => (),
            (_, 0) => (),
            _ => (),
        }
    }
    println!("{} fizzbuzzes", num_fizzbuzzes);
}

fn fizz_buzz(should_print: bool) {
    let mut num_fizzbuzzes = 0;
    for value in 1..302 {
        if value % 5 == 0 && value % 3 == 0 {
            if should_print {
                println!("fizz buzz {}", value);
            }

            num_fizzbuzzes += 1;
        } else if value % 5 == 0 {
            if should_print {
                println!("buzz");
            }
        } else if value % 3 == 0 {
            if should_print {
                println!("fizz");
            }
        }
    }
    println!("{} fizzbuzzes occured", num_fizzbuzzes);
}
