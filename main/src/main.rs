use core::blockchain::Blockchain;
fn main() {
    let mut chain = Blockchain::new();
    chain.add_block("a transfer to b 1 coin".to_string());
    chain.add_block("b transfer to c 1 coin".to_string());

    for block in chain.blocks {
        println! {"+++++++++++++++++++++"}
        println!("{:?}", block);
    }
}
