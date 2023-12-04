use std::io::stdin;

fn main() {
    println!("\n");

    // 1.
    let mut x = String::new();
    println!("1. Please enter a number of type `u32` to calculate f_x when x =");
    stdin().read_line(&mut x).expect("Didn't Receive Input");
    match x.trim().parse() {
        Ok(k) => println!("x = {}; f_x = {}", k, f_x(k)),
        Err(_) => panic!("\n\n\tThere was an error parsing the number you entered for x !!!\n\n")
    } 
    
    println!("\n-------------------------------------\n");
    
    // 2.
    let mut n = String::new();
    println!("2. Please enter a number of type `u64` to calculate f_n when n =");
    stdin().read_line(&mut n).expect("Didn't Receive Input");
    match n.trim().parse() {
        Ok(k) => println!("n = {}; f_n = {}", k, f_n(k)),
        Err(_) => panic!("\n\n\tThere was an error parsing the number you entered for n !!!\n\n")
    }

    println!("\n-------------------------------------\n");

    // 3.
    let mut m: u64 = 0;
    for i in 1_u64..{
        if calc_pi(&(i + 1)) - calc_pi(&i) >= 10_f64.powf(-8_f64) {
            continue;
        }

        m = i;
        break;
    }

    println!("3. The n and (n+1) for which the 2 neighbouring PIs' difference is less than 10^(-8) are:");
    println!("\tn = {}; PI(n) = {:.11}", m, calc_pi(&m));
    println!("\tn+1 = {}; PI(n+1) = {:.11}", m+1, calc_pi(&(m + 1)));
    println!("\tPI(n+1) - PI(n) = {:.11}", calc_pi(&(m + 1)) - calc_pi(&m));

    println!("\n")
}


fn f_x(x: u32) -> f32 {
    let x_f32 = x as f32;
    if x_f32 < 1_f32 {
        return x_f32.powf(2_f32) * (x_f32.powf(6_f32) + x_f32.sin()).powf(x_f32);
    }

    let exp = (1_f32).exp();
    
    if x_f32 > 1_f32 {
        return (x_f32.powf(6_f32) + x_f32.sin()) * (x_f32.powf(2_f32) + 1_f32).log(exp); 
    }

    (1_f32 + x_f32.sin().powf(2_f32)).log(exp)
}

fn f_n(n: u64) -> f32 {
    let mut sum: f32 = 0_f32;

    for i in 1..=n {
        let i_f32 = i as f32;
        sum += (2_f32 + i_f32).cos() + i_f32.powf(2_f32);
    }

    sum
}


fn calc_pi(n: & u64) -> f64 {
    let mut sum: f64 = 0_f64;

    for i in 1..=*n {
        let i_f64 = i as f64;
        sum += 4_f64 / ((2_f64 * i_f64) * (2_f64 * i_f64 + 1_f64) * (2_f64 * i_f64 + 2_f64));
    }

    3_f64 + sum
}
