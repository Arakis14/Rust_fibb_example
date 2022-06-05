fn calculate_fib(z: u8) {
    let mut count = 0;
    let mut x_2 = 0;
    let mut x_1 = 1;
    let mut x = x_1 + x_2;
    while count != z {
        match count {
            0 => println!("{}", x_2),
            1 => println!("{}", x_1),
            _ => println!("{}", x),
        };
        if count > 1 {
            x_2 = x_1;
            x_1 = x;
            x = x_1 + x_2
        }
        count += 1;
    }
}

fn main() {
    println!("Wypisze 20 liczb ciągu fibbanaciego ");
    calculate_fib(20);
}
