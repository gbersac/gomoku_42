extern crate std;
extern crate piston;
extern crate opengl_graphics;
extern crate graphics;
#[cfg(feature = "include_sdl2")]
extern crate sdl2_window;
#[cfg(feature = "include_glfw")]
extern crate glfw_window;
#[cfg(feature = "include_glutin")]
extern crate glutin_window;

#[cfg(feature = "include_sdl2")]
use self::sdl2_window::Sdl2Window as Window;
#[cfg(feature = "include_glfw")]
use self::glfw_window::GlfwWindow as Window;
#[cfg(feature = "include_glutin")]
use self::glutin_window::GlutinWindow as Window;

use self::opengl_graphics::GlGraphics;

use self::piston::input::*;
use self::piston::event_loop::*;

use display::mouse::Mouse;
use display::draw;

use board::GoBoard;
use board::Team;
use board::Tile;

use ia::Decision;
use ia::heuristic;

const CASE_WIDTH: graphics::types::Resolution = 40;
const ORANGE: graphics::types::Color = [0.97647065f32, 0.9450981f32, 0.854902f32, 1f32];

#[derive(Debug, PartialEq, Clone)]
pub enum Player {
    Human,
    Ia,
}

impl Player {
	pub fn from_str(s: &str) -> Player {
	    match s {
	        "ia"	=> Player::Ia,
	        "human"	=> Player::Human,
	        _		=> panic!("Player cli option must be either ia, solo or multi")
	    }
	}
}

#[derive(Debug, Clone)]
pub struct Console {
    board: GoBoard,
    event: Mouse,
    player: (Team, Player),
    friend: (Team, Player),
    layer: u32,
    turn: u32, // Player one = true, player two = false.
    win: bool,
    help: bool,
    help_decision: (u32, u32),
    info: bool,
    debug_map: bool,
}

impl Console {

	/// The `new` constructor function returns the interface console.

    pub fn new (
        board: GoBoard,
        layer: u32,
        (player, friend): (Player, Player),
        info: bool,
        debug_map: bool,
        help: bool,
    ) -> Self {
        let size: u32 = board.get_size() as u32;
        let (team_player, team_friend) = Team::new_teams();

		Console {
			board: board,
            event: Mouse::new((CASE_WIDTH * size, CASE_WIDTH * size)),
            player: (team_player, player),
            friend: (team_friend, friend),
            turn: 0,
            layer: layer,
            win: false,
            help: help,
            help_decision: (size/2, size/2),
            info: info,
            debug_map: debug_map,
		}
    }

    /// The `get_size` function returns window size.

    fn get_size (
        &self
    ) -> piston::window::Size {
        let size: graphics::types::Resolution = self.board.get_size (
        ) as graphics::types::Resolution;
        let dimension = self.event.get_dimension();

        piston::window::Size::from([
            dimension.0 / size,
            dimension.1 / size,
        ])
    }

    /// The `get_turn_is_ia` function returns a boolean if a IA must play.

    fn get_turn_is_ia (&self) -> bool {
        match (self.turn % 2 == 0, &self.player, &self.friend) {
            (false, _, &(_, Player::Ia)) => true,
            (true, &(_, Player::Ia), _) => true,
            _ => false,
        }
    }

    /// The `set` function updates the turn and set the human coordinate.

    fn set (&mut self, event: &Event, team: &mut Team) -> (u32, u32) {
        let (x, y) = self.event.get_coordinate();

        if x < self.board.get_size() as u32 && y < self.board.get_size() as u32 {
            if let Some(Button::Mouse(_)) = event.press_args() {
                if self.board.set((x as usize, y as usize), team) {
                    self.turn += 1;
                    if self.help && self.get_turn_is_ia() == false {
                        self.help_decision = self.help_optimal_move();
                    }
                    if self.info {
                        println!("#{} human - {}: {}'s captured, [{}, {}]'s played.", self.turn, team, team.captured(), x, y);
                    }
                    if self.debug_map {
                        println!("{}", self.board);
                    }
                }
            };
            (x, y)
        }
        else {
            (0, 0)
        }
    }

    /// The `set_raw` function updates the turn and set the IA coordinate.

    fn set_raw (&mut self, (x, y): (usize, usize), team: &mut Team) -> (u32, u32) {
        self.board.set((x, y), team);
        self.turn += 1;
        if self.help && self.get_turn_is_ia() == false {
            self.help_decision = self.help_optimal_move();
        }
        if self.info {
            println!("#{} ia - {}: {}'s captured, [{}, {}]'s played.", self.turn, team, team.captured(), x, y);
        }
        if self.debug_map {
            println!("{}", self.board);
        }
        (x as u32, y as u32)
    }

    /// The `is_ia_versus` function returns a boolean if the player one
    /// and two are typed like IA.

    fn is_ia_versus (&self) -> bool {
        match (&self.player, &self.friend) {
            (&(_, Player::Ia), &(_, Player::Ia)) => true,
            _ => false,
        }
    }

    /// The `play` function sets the board with the new tail coordinate.

    fn play (&mut self, event: &Event) -> Option<Tile> {
        let (x, y):(u32, u32) = match (
            self.turn % 2 == 0,
            &mut self.player,
            &mut self.friend
        ) {
            (true, &mut (mut player_team, Player::Ia), &mut (friend_team, _)) => {
                let decision = Decision::get_optimal_move(&mut self.board, &(player_team, friend_team), friend_team, self.layer, heuristic);
                let result = self.set_raw(decision.get_result(), &mut player_team);

                self.player.0 = player_team;
                decision.print_result();
                result
            },
            (false, &mut (player_team, _), &mut (mut friend_team, Player::Ia)) => {
                let decision = Decision::get_optimal_move(&mut self.board, &(player_team, friend_team), player_team, self.layer, heuristic);
                let result = self.set_raw(decision.get_result(), &mut friend_team);

                self.friend.0 = friend_team;
                decision.print_result();
                result
            },
            (true, &mut (mut player_team, Player::Human), &mut (_, _)) => {
                let result = self.set(event, &mut player_team);

                self.player.0 = player_team;
                result
            },
            (false, &mut (_, _), &mut (mut friend_team, Player::Human)) => {
                let result = self.set(event, &mut friend_team);

                self.friend.0 = friend_team;
                result
            },
        };
        self.board.is_win(x as usize, y as usize)
    }

    /// The `help_optimal_move` function returns the recommended coordinate to play.

    fn help_optimal_move (&mut self) -> (u32, u32) {
        let (x, y) = if self.turn % 2 == 0 {
            Decision::get_optimal_move (
                &mut self.board,
                &(self.player.0, self.friend.0),
                self.player.0,
                self.layer,
                heuristic
            ).get_result()
        }
        else {
            Decision::get_optimal_move (
                &mut self.board,
                &(self.player.0, self.friend.0),
                self.friend.0,
                self.layer,
                heuristic
            ).get_result()
        };
        (x as u32, y as u32)
    }

    /// The `input` function listens all mouse event like resize and click.

    fn input (
        &mut self,
        event: &Event,
        limit: u32
    ) {

        if let Some(resize) = event.resize(|w, h| (w as u32, h as u32)) {
            self.event.set_dimension(resize);
        }
        if self.win == false {

            if let Some(coordinate) = event.mouse_cursor(|x, y| {
                (x as u32, y as u32)
            }) {
                if let Some(coordinate) = self.event.check_inside_window (
                    coordinate,
                    limit,
                ) {
                    self.event.set_coordinate(coordinate);
                }

            }
            if let Some(team) = self.play(&event) {
                println!("{} win! Give him a cookie !", team.ennemy());
                self.win = true;
            }
        }
    }

    /// The `draw` function refreshs the window with a new board.

    fn draw (
        &mut self,
        gl: &mut GlGraphics,
        event: &RenderArgs,
        limit: u32
    ) {
        let dimension = self.get_size();
        gl.draw(event.viewport(), |context, g| {
            graphics::clear(ORANGE, g);
            draw::draw_render(&self.board, dimension, limit, (&context, g));

            if self.help
            && self.win == false
            && self.get_turn_is_ia() == false {
                draw::draw_help(&self.board, dimension, self.help_decision, (
                    &context,
                    g
                ));
            }
            if self.event.get_over() {
                let (x, y) = self.event.get_coordinate();

                if x < self.board.get_size() as u32 && y < self.board.get_size() as u32 {
                    draw::draw_over (
                        &self.board,
                        dimension,
                        (x, y),
                        (&context, g)
                    );
                }
            }
        });
    }

    /// The `start` function loops the board.

    pub fn start (
        &mut self,
    ) {
        let opengl = opengl_graphics::OpenGL::V3_2;
        let window: Window = piston::window::WindowSettings::new (
            "Gomoku",
            self.event.get_dimension(),
        ).exit_on_esc(true).opengl(opengl).build().unwrap();
        let ref mut gl = opengl_graphics::GlGraphics::new(opengl);
        let window = std::rc::Rc::new(std::cell::RefCell::new(window));
        let limit: u32 = self.board.get_size() as u32;

        if self.is_ia_versus() {
            for event in window.clone().events() {
                if let Some(render) = event.render_args() {
                    self.input(&event, limit);
                    self.draw(gl, &render, limit);
                    event.update(|_| {});
                }
            }
        } else {
            for event in window.clone().events() {

                self.input(&event, limit);
                if let Some(render) = event.render_args() {
                    self.draw(gl, &render, limit);
                    event.update(|_| {});
                }
            }
        }
    }
}

impl Default for Console {

	/// The `new` constructor function returns the interface console.

    fn default () -> Self {
        let board: GoBoard = Default::default();
        let (team_player, team_friend) = Team::new_teams();
        let size: u32 = board.get_size() as u32;

		Console {
			board: board,
            event: Mouse::new((CASE_WIDTH * size, CASE_WIDTH * size)),
            player: (team_player, Player::Human),
            friend: (team_friend, Player::Ia),
            layer: 3,
            turn: 0,
            win: false,
            help: false,
            help_decision: (size/2, size/2),
            info: true,
            debug_map: false,
		}
    }
}
