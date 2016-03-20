# GoGame42

[![travis-badge][]][travis] [![docs-badge][]][docs]

[travis-badge]: https://travis-ci.org/gbersac/gomoku_42.svg?style=flat-square
[travis]: https://travis-ci.org/gbersac/gomoku_42
[docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
[docs]: https://gbersac.github.io/gomoku_42/gomoku

A gomoku game with an **artificial intelligence**. The goban has a width of 19 tiles. Developped using the [rust programming language](https://www.rust-lang.org/).

#### ScreenShot:
![Screen Shot][display-screenshot]

[display-screenshot]: https://raw.githubusercontent.com/gbersac/gomoku_42/gh-pages/screenshot.apng

#### How to play:
```shell
# Between human:
cargo run --release -- --layers 4 --friend human --player human
# Human with ai:
cargo run --release -- --layers 4 --friend human --player ai
```

#### Cargo'git-Dependencies:
```shell
                      sdl2 glfw glutin
                         \   |   /
opengl_graphics graphics (window)
              \ /            |
              (2d)        (core)
                \        /
clap    chrono    piston
     \     |    /
        gomoku
```

#### Directory-Tree:
```shell
.
|__ Cargo.lock
|__ Cargo.toml
|__ README.md
|__ main.rs
|__ one_test.rs
\__ src
    |__ cli.yml
    |__ cmd_option.rs
    |__ one_test.rs
    |__ bench.rs
    |__ display
    │   |__ console.rs
    │   |__ draw.rs
    │   |__ mod.rs
    │   \__ mouse.rs
    |__ ai
    |   |__ decision.rs
    |   |__ heuristic.rs
    |   |__ mod.rs
    |   |__ move_to_evaluate.rs
    |   |__ test_decision.rs
    |   |__ test_move_to_evaluate.rs
    |   \__ turn.rs
    \__ board
        |__ fn_str.rs
        |__ go_board.rs
        |__ mod.rs
        |__ parse.rs
        |__ team.rs
        |__ test_capture.rs
        |__ test_free_threes.rs
        |__ test_win.rs
        \__ tile.rs
```
