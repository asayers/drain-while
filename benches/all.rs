extern crate drain_while;
extern crate easybench;

mod dw;
mod dw2;

fn main() {
    println!("\ndrain_while:");
    dw::bench();
    println!("\ndrain_while_2:");
    dw2::bench();
}
