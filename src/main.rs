pub fn climb_stairs(n: i32) -> i32 {
    if n < 2{
        return n;
    }
    let mut f_minus_1:i32 = 1;
    let mut f_minus_2:i32 = 1;
    let mut fib = (1,1);
    
    for _ in 0..n{
        
        let mut f:i32 = fib.0 + fib.1;
        fib.1 = fib.0;
        fib.0 = f;
        // f_minus_2 = f_minus_1;
        // f_minus_1 = f;
    }
    
    // println!("Hello World! value is {}",f_minus_1);
    return fib.1;
}

fn main() {
    let n = 32;
    let res = climb_stairs(n);
    println!("There are {} distinct ways to climb {} steps",n,res);
}
