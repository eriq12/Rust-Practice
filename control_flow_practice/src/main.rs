fn main() {
    let condition = true;
    // terinary statement
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // if, else if, and else work similar to c family, save for requirements for ()
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 2, 3, or 4");
    }

    // there is also loop, which will have program run indefinitely
    // loop {
    //      println!("Again!");
    //  }
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // loops can have labels to specify fo continue and break, label needs to be preceeded by a single quotation mark '
    let mut count = 0;
    'counting_up : loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while loops, similar to c family, but same difference as if, else if, and else statements
    let mut count_down = 3;
    while count_down > 0 {
        println!("{count_down}");
        count_down -= 1;
    }
    println!("LIFTOFF!!!");

    // for loops seems to be equivalent to for each loops
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is {element}");
    }

    // for alternative, (<num1>..<num2>) seems to produce an array, num1 inclusive, num2 exclusive?
    for num in (1..4).rev(){
        println!("{num}!");
    }
    println!("LEFTOFF!!!");
}
