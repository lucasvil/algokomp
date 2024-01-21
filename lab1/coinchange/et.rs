use std::cmp;
use std::time::{Instant, Duration};

fn main(){
    let mut n = 1;
    let a = 5;
    let b = 6;
    let c = 7;

    let mut now;
    let mut elapsed = Duration::new(0,0);

    while n < 30000 {
        now = Instant::now();
        coins(n,a,b,c);
        elapsed = now.elapsed();
        println!("{} {}", n, elapsed.as_micros());
        n+=1;
        //n=n*2;
    }
}

fn coins(n:i32, a:i32, b:i32, c:i32)->i32{
    let mut n_coins =vec![n; (n+1) as usize];

    n_coins[0] = 0;

    for amt in 1..(n+1) {
        for coin in [1,a,b,c] {
            if coin <= amt{
                n_coins[amt as usize] = cmp::min(n_coins[amt as usize], 1+n_coins[(amt-coin) as usize])
            }
        }
    }
    n_coins[n as usize]
}