use clap::Shell;
use clap::{Arg, SubCommand};
use std::error::Error;
use std::io::BufWriter;

pub const NAME: &str = "completion";

pub fn create() -> clap::App<'static, 'static> {
    let spec_file = Arg::with_name("completion-shell")
        .index(1)
        .possible_values(&["bash", "fish", "elvish", "powershell", "zsh"])
        .help("The shell to generate completions for")
        .required(true);

    SubCommand::with_name(NAME)
        .about("Output completion for a shell of your choice")
        .arg(spec_file)
}

pub fn execute(run_matches: &clap::ArgMatches<'_>) {
    let shell = match run_matches
        .value_of("completion-shell")
        .expect("failed to parse shell")
    {
        "bash" => Shell::Bash,
        "fish" => Shell::Fish,
        "elvish" => Shell::Elvish,
        "powershell" => Shell::PowerShell,
        "zsh" => Shell::Zsh,
        shell => panic!("Unknown shell: {}", shell),
    };

    let mut buf = BufWriter::new(Vec::new());
    create().gen_completions_to(create().get_name(), shell, &mut buf);
    println!(
        "{}",
        buf.into_inner()
            .map_err(Box::from)
            .and_then(|x| String::from_utf8(x).map_err(Box::<dyn Error>::from))
            .expect("failed to generate utf8 completions")
    );
}
