#[macro_use]
extern crate clap;

mod board;
mod display;
mod ia;
mod cmd_option;

use clap::App;
use display::{Play};
use cmd_option::CmdOption;

fn main() {
    //options
    let yml = load_yaml!("cli.yml");
    let m = App::from_yaml(yml).get_matches();
    let opts = CmdOption::parse(&m);
    println!("{:?}", opts);

    let mut game:Play = Default::default();

    game.start();
    //display::main(board, &mut teams);
}
