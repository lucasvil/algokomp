use std::cmp;
use std::time::{Instant, Duration};
pub const INFINITY: i32 = i32::MAX-1;

fn main(){
    let mut n = 1;
    let a = 5;
    let b = 6;
    let c = 7;

    let mut now;
    let mut elapsed = Duration::new(0,0);
    while elapsed.as_micros() < 5000 {
        now = Instant::now();
        coins(n,a,b,c);
        elapsed = now.elapsed();
        println!("{} {}", n, elapsed.as_micros());
        //n+=1;
        n=n*2;
    }
}

fn coins(n:i32, a:i32, b:i32, c:i32)->i32{
    if n < 0{
        INFINITY
    }else if n == 0 {
        0
    }else{
        min(n, 1+coins(n-a,a,b,c), 1+coins(n-b,a,b,c), 1+coins(n-c,a,b,c))
    }
}

fn min(a:i32, b:i32, c:i32, d:i32)->i32{
    let min1 = cmp::min(a, b);
    let min2 = cmp::min(c, d);
    cmp::min(min1, min2)
}