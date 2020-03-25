use find_union::*;
use rand::Rng;
use std::io;
use std::time::Instant;
fn main() {
    println!("input the number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Not a number");
    let num = input.trim().parse().unwrap();
    let qf = QuickFindUf::new(num);
    let qu = QuickUnionUf::new(num);
    let wqu = WeighedQu::new(num);
    let pcqu = PathCompressQu::new(num);
    let wpcqu = WQUPC::new(num);
    println!("QuickFindUf:");
    bencher(qf, num);
    println!("QuickUnionUf:");
    bencher(qu, num);
    println!("Weighed_QU:");
    bencher(wqu, num);
    println!("PathCompressQu:");
    bencher(pcqu, num);
    println!("WQUPC:");
    bencher(wpcqu, num);
}
fn bencher<T: Ufmain>(mut uf: T, num: usize) {
    let first_cost: f64;
    let second_cost: f64;
    let mut connecting_count = 0;
    let mut connected_count = 0;
    let now = Instant::now();
    //random union for 'num' times
    for _ in 0..num {
        let p = rand::thread_rng().gen_range(0, num);
        let q = rand::thread_rng().gen_range(0, num);
        if p != q {
            uf.union(p, q);
            connecting_count += 1;
            //println!("Connecting {}-{}", p, q);
        }
    }
    first_cost = now.elapsed().as_nanos() as f64 / 1000000000 as f64;
    let now = Instant::now();
    //random find
    for _ in 0..num {
        let p = rand::thread_rng().gen_range(0, num);
        let q = rand::thread_rng().gen_range(0, num);
        if p != q {
            if uf.connected(p, q) {
                connected_count += 1;
                //println!("{}-{} Connected", p, q);
            }
        }
    }
    //
    //iterate all
    /* for i in 0..num {
            for j in i + 1..num {
                if uf.connected(i, j) {
                    connected_count += 1;
                    //println!("{}-{} Connected", p, q);
                }
            }
    } */
    //
    second_cost = now.elapsed().as_nanos() as f64 / 1000000000 as f64;
    println!(
        "union({}):{}s ,find({}):{}s",
        connecting_count, first_cost, connected_count, second_cost,
    );
}
