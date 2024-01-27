use std::io;

fn main(){
    let n = read_num() as i32; // total number of games x
    let k = read_num() as i32; // number of games in a row needed (2 ≤ k ≤ n)
    let p = read_num(); // real number, probability of winning each game (0 ≤ p ≤ 1)

    let mut mem =vec![-1.0; (n+1) as usize];

    print!("{}", streak(n,k,p, &mut mem));
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

fn read_num()-> f64{
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
    line.trim().parse::<f64>().unwrap()
}