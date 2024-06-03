use inquire::{Text, ui, self};
use clap::{Command, Subcommand, Parser, arg};

#[derive(Debug, Parser)]
#[command(multicall = true, arg_required_else_help(true))]
struct Interactive {
    /// some help
    #[command(subcommand)]
    APPLET: Applets,
}

#[derive(Debug, Parser)]
#[command(about)]
struct RunCmd {
    /// cmd lines
    cmdline: String,
}

#[derive(Debug, Subcommand)]
enum Applets {
    /// run commands
    Exec(RunCmd),
    /// exit
    Quit,
}

fn main() {
    let default = ui::RenderConfig::default();
    let prompt_prefix = ui::Styled::new("$").with_fg(ui::Color::DarkRed);
    let main_config = default.with_prompt_prefix(prompt_prefix);

    loop {
        let input_result = Text::new("").with_render_config(main_config).with_placeholder("test").prompt();
        let input = match input_result {
            Ok(input) => input,
            Err(e) => {
                println!("get error {}", e);
                continue;
            }
        };

        let args = shlex::split(&input);
        if let Some(args) = args {
            let cli = Interactive::try_parse_from(args);
            let cli = match cli {
                Ok(cli) => cli,
                Err(e) => {
                    println!("get err while parse {}", e);
                    continue;
                }
            };
            match cli.APPLET {
                Applets::Exec(cmd) => println!("cmd {}", cmd.cmdline),
                Applets::Quit => break,
            }
        } else {
            println!("split error")
        }
    }
}

