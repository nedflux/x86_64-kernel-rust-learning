use crate::println;
use crate::shell::MAX_COMMAND_ARGS_LEN;

pub fn cowsay(args: [Option<&str>; MAX_COMMAND_ARGS_LEN]) {
    println!("
   ({})
     \\   ^__^
      \\  (><)\\_______
         (__)\\       )\\/\\
             ||----w |
             ||     ||
        ", args[0].unwrap_or("MOO"));
}
pub fn whoami() {
    println!("Hey, my name is Alexey");
}
pub fn help() {
    println!("
x86-64 rust kernel.
commands:

    cowsay [ARGS]: Print a cow and exit.

    help: Print this help and exit.

    whoami: Print my name.

    aboutme: Print some information about me.

    clear: clear the screen.
        ")
}

pub fn aboutme() {
    println!("
Dev: Alexey K.

What is this?: Written in rust kernel for x86_64 arch.

Why did I choose this architecture?:\n The only argument is that there\n is more training material for this architecture,\n otherwise I would prefer RISC-V,\n which I will implement in the next project

My goals: Understand the Rust operating system development base.

My real OS: Gentoo linux hardened x86_64.
        ")
}
