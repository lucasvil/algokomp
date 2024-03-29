use std::cmp;

fn main(){
    let n = read_int();
    let a = read_int();
    let b = read_int();
    let c = read_int();

    print!("{}", coins(n,a,b,c));
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

fn read_int()->i32{
    use std::io;
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
    line.trim().parse::<i32>().unwrap()
}