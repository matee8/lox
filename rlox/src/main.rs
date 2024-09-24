use rlox::{
    chunk::{Chunk, OpCode},
    vm::Vm,
};

fn main() {
    let mut c = Chunk::new();
    c.write_constant(1.2, 123);
    c.write_constant(3.4, 123);
    c.write_opcode(OpCode::Add, 123);
    c.write_constant(5.6, 123);
    c.write_opcode(OpCode::Divide, 123);
    c.write_opcode(OpCode::Negate, 123);
    c.write_opcode(OpCode::Return, 123);
    println!("{c:?}");
    let mut vm = Vm::new(&c);
    vm.interpret().unwrap();
}
