fn main() {
    hello_world_function();
    let mut x = five();
    another_function(x);
    x = twentyfive();
    statements(x);
}

// basic format of a function definition
fn hello_world_function(){
    println!("Hello, world!");
}


// function definition with parameters
fn another_function(x:i32){
    println!("The function was given a param x of value {x}");
}

// statement and expression
fn statements(z:i32){
    // the following statement would be invalid:
    // let y = (let x = 6);
    // as the right hand side of the assignment statement cannot be a statement itself, it requires an expression
    // for reference, function definitions are also statements

    // the block (denotated by the collection surrounded by {})
    // the last expression (x + 1) is what the block evaluates to
    let y = {
        let x = z;
        x + 1
    };

    println!("The value of y is: {y}");
}

// return values
fn five() -> i32 {
    // like stated earlier, the last expression will be returned, note as return the expression needs to NOT have a semicolon (';') at the end of the line
    5
}

// return values, cont.
fn twentyfive() -> i32 {
    // like many in the c family and more, you can do "return {value};"
    return 25;
}