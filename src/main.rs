#![allow(unused_features)]
#![feature(test)]

#[macro_use]
extern crate clap;
extern crate chrono;

mod board;
mod display;
mod ia;
mod cmd_option;
#[cfg(test)]
mod one_test;
#[cfg(test)]
mod bench;

use clap::App;
use display::{Console};
use cmd_option::CmdOption;

fn main() {
    // options
    let yml = load_yaml!("cli.yml");
    let m = App::from_yaml(yml).get_matches();
    let opts = CmdOption::parse(&m);

    let mut game: Console = Console::new(
        opts.init_map,
        opts.layers,
        (opts.player, opts.friend),
        opts.info,
        opts.debug_map,
        opts.human_help,
    );

    game.start();
}
