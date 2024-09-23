use rlox::chunk::{Chunk, OpCode};

fn main() {
    let mut c = Chunk::new();
    c.write_opcode(OpCode::OpReturn, 123);
    c.write_constant(1.2, 123);
    println!("{c:?}");
}
