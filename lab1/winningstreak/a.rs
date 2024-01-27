use std::io;

fn main(){
    let n = read_num() as i32; // total number of games x
    let k = read_num() as i32; // number of games in a row needed (2 ≤ k ≤ n)
    let p = read_num(); // real number, probability of winning each game (0 ≤ p ≤ 1)

    let mut mem =vec![vec![-1.0; (k+1) as usize]; (n+1) as usize];

    print!("{}", streak(n,k,k,p, &mut mem));
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

fn read_num()-> f64{
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
    line.trim().parse::<f64>().unwrap()
}