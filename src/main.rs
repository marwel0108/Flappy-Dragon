use bracket_lib::prelude::*;
use bracket_color::prelude::*;

#[derive(Debug)]
enum GameMode {
    Menu,
    Playing,
    End
}

#[derive(Debug)]
struct State {
    game_mode: GameMode,
    text_position_x: u32,
    text_position_y: u32,
}

impl State {

    fn new() -> Self {
        State { 
            game_mode: GameMode::Menu,
            text_position_y: 0,
            text_position_x: 0
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        println!("{:?}", self);
        ctx.print(self.text_position_x,self.text_position_y, "Now you are playing!");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Escape => self.game_mode = GameMode::Menu,
                VirtualKeyCode::Up => {
                    if self.text_position_y <= 1 {
                        self.text_position_y = ctx.get_char_size().1 - 1;
                    } else {
                        self.text_position_y -= 1;
                    }
                },
                VirtualKeyCode::Down => {
                    if self.text_position_y >= ctx.get_char_size().1 - 1  {
                        self.text_position_y = 0;
                    } else {
                        self.text_position_y += 1;
                    }
                },
                VirtualKeyCode::Right => {
                    if self.text_position_x >= ctx.get_char_size().0 - 1 {
                        self.text_position_x = 0;
                    } else {
                        self.text_position_x += 1;
                    }
                },
                VirtualKeyCode::Left => {
                    if self.text_position_x <= 1 {
                        self.text_position_x = ctx.get_char_size().0 - 1;
                    } else {
                        self.text_position_x -= 1;
                    }
                }
                _ => {}
            }
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        let height = ctx.get_char_size().1 / 4;
        ctx.cls();
        ctx.print_centered(height, "Welcome to Flappy Dragon!!!");
        ctx.print_color_centered(height * 2, RGBA::from_u8(78,204,69, 255), RGBA::from_u8(0, 0, 0, 0), "Press P to play");
        ctx.print_color_centered(height * 3, RGBA::from_u8(204, 14, 46, 255), RGBA::from_u8(0, 0, 0, 0), "Press Q to quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quit(),
                _ => {}
            }
        }
    }

    fn end(&mut self, ctx: &mut BTerm) {
        todo!()
    }

    fn restart(&mut self) {
        self.text_position_y = 0;
        self.game_mode = GameMode::Playing;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        println!("{:?}", self.game_mode);
        match self.game_mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => ctx.print(25, 25, "End of the game") 
        }
    }
}

fn main() -> BError {
    
    let context: BTermBuilder = BTermBuilder::simple80x50();

    let window: BTerm = context.build()?;

    main_loop(window, State::new())
}
