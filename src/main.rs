mod brainfuck;

use brainfuck::interpreter::Interpreter;

fn main() {
    let mut interpreter = Interpreter::new();
    interpreter.load_program_from_file("program.bf");
    interpreter.run();
}
