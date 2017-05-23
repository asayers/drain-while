extern crate drain_while;
extern crate easybench;

mod dw;

fn main() {
    println!("### drain_while ######");
    dw::bench();
}
