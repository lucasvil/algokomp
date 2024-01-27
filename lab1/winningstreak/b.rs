use std::time::{Instant, Duration};

fn main(){
    let mut n:i32 = 4; // total number of games x
    let mut k:i32 = n/2; // number of games in a row needed (2 ≤ k ≤ n)
    let p:f64 = 0.99; // real number, probability of winning each game (0 ≤ p ≤ 1)

    let mut now;
    let mut elapsed = Duration::new(0,0);
    while elapsed.as_millis() < 1000 {
        now = Instant::now();

        let mut mem =vec![vec![-1.0; (k+1) as usize]; (n+1) as usize];
        streak(n,k,k,p, &mut mem);

        elapsed = now.elapsed();
        println!("{} {}", n, elapsed.as_millis());

        n+=1;
        //n=n*2;
        k=n/2
    }
}

// answer given by streak(n, k)
fn streak(n:i32, y:i32, k:i32, p:f64, mem: &mut Vec<Vec<f64>>)->f64{
    if y == 0 {
        1.0
    }else if (n == 0) && (y > 0){
        0.0
    }else if mem[n as usize][y as usize] != -1.0 {
        mem[n as usize][y as usize]
    }else{
        mem[n as usize][y as usize]=p*streak(n-1,y-1,k,p, mem) + (1.0-p)*streak(n-1,k,k,p, mem);
        mem[n as usize][y as usize]
    }
}
