extern crate drain_while;
extern crate easybench;

mod dw;
mod dw2;

fn main() {
    println!("### drain_while ######");
    dw::bench();
    println!("### drain_while_2 ####");
    dw2::bench();
}
