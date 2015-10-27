#[macro_use]
extern crate clap;

mod board;
mod display;
mod ia;
mod cmd_option;

use clap::App;
use display::{Console};
use cmd_option::CmdOption;

fn main() {
    // options
    let yml = load_yaml!("cli.yml");
    let m = App::from_yaml(yml).get_matches();
    let opts = CmdOption::parse(&m);

    let mut game:Console = Console::new (
        Default::default(),
        opts.layers,
        (opts.player, opts.friend),
        opts.human_help,
    );

    game.start();
}
