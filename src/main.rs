use inquire::{Text, ui};
use clap;

fn main() {
    let default = ui::RenderConfig::default();
    let prompt_prefix = ui::Styled::new("$").with_fg(ui::Color::DarkRed);
    let main_config = default.with_prompt_prefix(prompt_prefix);

    loop {
        let input = Text::new("").with_render_config(main_config).prompt();
        match input {
            Ok(input) => {
                println!("input {}", input);
                if input.trim() == "quit" {
                    break;
                }
            }
            Err(e) => println!("get error {}", e)
        }
    }
}
