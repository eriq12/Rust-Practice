pub fn fib(n:u32) -> u32{
    let mut value_one : u32 = 0;
    let mut value_two : u32 = 1;
    let mut index = 1;
    while index < n {
        let value_next : u32 = value_one + value_two;
        index += 1;
        if index % 2 == 1 {
            value_two = value_next;
        } else {
            value_one = value_next;
        }
    }
    return if n % 2 == 1 { value_two } else { value_one }
}