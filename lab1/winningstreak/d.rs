use std::time::{Instant, Duration};

fn main(){
    let mut n:i32 = 4; // total number of games x
    let mut k:i32 = n/2; // number of games in a row needed (2 ≤ k ≤ n)
    let p:f64 = 0.99; // real number, probability of winning each game (0 ≤ p ≤ 1)

    let mut now;
    let mut elapsed = Duration::new(0,0);
    while n <= 20000 {
        now = Instant::now();

        let mut mem =vec![-1.0; (n+1) as usize];
        streak(n,k,p, &mut mem);

        elapsed = now.elapsed();
        println!("{} {}", n, elapsed.as_micros());

        n+=1;
        //n=n*2;
        k=n/2
    }
}

// answer given by streak(n, k)
fn streak(x:i32, k:i32, p:f64, mem: &mut Vec<f64>)->f64{
    if x < k {
        0.0
    }else if x==k{
        f64::powf(p,k as f64)
    }else if mem[x as usize] != -1.0 {
        mem[x as usize]
    }else{
        mem[x as usize]=streak(x-1,k,p,mem)+f64::powf(p,k as f64)*(1.0-p)*(1.0-streak(x-k-1,k,p,mem));
        mem[x as usize]
    }
}