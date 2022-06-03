use clap::{Args, Command};

#[derive(Args)]
pub struct Arguments {
    /// The shell to generate completions for
    #[clap(arg_enum)]
    shell: clap_complete_command::Shell,
}

pub fn execute(cmd: &mut Command<'_>, args: &Arguments) {
    args.shell.generate(cmd, &mut std::io::stdout());
}
