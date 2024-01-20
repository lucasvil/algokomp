use std::cmp;
pub const INFINITY: i32 = i32::MAX-1;

fn main(){
    let n = read_int();
    let a = read_int();
    let b = read_int();
    let c = read_int();

    let mut mem =vec![-1; (n+1) as usize];

    print!("{}", coins(n,a,b,c, &mut mem));
}

fn coins(n:i32, a:i32, b:i32, c:i32, mem: &mut Vec<i32>)->i32{
    if n < 0{
        INFINITY
    }else if n == 0 {
        0
    }else if mem[n as usize] != -1 {
        mem[n as usize]
    }else{
        mem[n as usize]=min(n, 1+coins(n-a,a,b,c,mem), 1+coins(n-b,a,b,c,mem), 1+coins(n-c,a,b,c,mem));
        mem[n as usize]
    }
}

fn min(a:i32, b:i32, c:i32, d:i32)->i32{
    let min1 = cmp::min(a, b);
    let min2 = cmp::min(c, d);
    cmp::min(min1, min2)
}

fn read_int()->i32{
    use std::io;
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
    line.trim().parse::<i32>().unwrap()
}