use crate::println;
use crate::shell::MAX_COMMAND_ARGS_LEN;

pub fn cowsays(args: [Option<&str>; MAX_COMMAND_ARGS_LEN]) {
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
