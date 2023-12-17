use sdl2::event::Event;
use sdl2::video::WindowContext;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use std::time::{Instant, Duration};
use sdl2::render::{Texture, TextureCreator};
use std::path::Path;

struct Character {
    current_animation_idx: usize,
    animations: Vec<Animation>,
    current_dest_rect: usize,
    dest_rects: Vec<Rect>,
}

impl Character {
    fn new(animations: Vec<Animation>, dest_rects: Vec<Rect>) -> Self {
        Self {
            current_animation_idx: 1,
            animations,
            current_dest_rect: 1,
            dest_rects,
        }
    }
}

struct Animation {
    frames: Vec<Rect>,
    current_frame: usize,
    frame_duration: Duration,
    last_instant: Instant,
    texture_idx: usize,
}

impl Animation {
    fn new(frames: Vec<Rect>, frame_duration: Duration, texture_idx: usize) -> Self {
        Self {
            frames,
            current_frame: 0,
            frame_duration,
            last_instant: Instant::now(),
            texture_idx,
        }
    }
    fn advance(&mut self) {
        if self.last_instant.elapsed() >= self.frame_duration {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
            self.last_instant = Instant::now();
        }
    }
    fn get_current_frame(&mut self) -> Rect {
        self.frames[self.current_frame]
    }
}

struct State<'a> {
    characters: Vec<Character>,
    textures: Vec<Texture<'a>>,
}

fn main() -> Result<(), String> {
    Ok(run()?) 
}

fn vertical_animation_from_texture_idx(texture_idx: usize, width: i32, height: i32, n_frames: i32, frame_duration: Duration  ) -> Animation {
    let mut frames_vec: Vec<Rect> = Vec::new(); 
    for i in 0..n_frames {
        frames_vec.push(Rect::new(0, height * i, width as u32, height as u32));
    }
    Animation::new(frames_vec, frame_duration, texture_idx)
}

fn get_texture<'a>(path: &Path, texture_creator: &'a TextureCreator<WindowContext>) -> Texture<'a> {
    return texture_creator.load_texture(path).unwrap();
}

fn insert_texture<'a>(texture: Texture<'a>, texture_vec: &mut Vec<Texture<'a>>) -> usize {
    texture_vec.push(texture);
    return texture_vec.len() -1;
}

fn run() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let screen_width = 1920;
    let screen_height = 1080;
    let window = video_subsystem.window("Dwell. The game.", screen_width, screen_height)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut textures: Vec<Texture> = Vec::new();

    // BACKGROUND
    let background_texture_idx = insert_texture(get_texture(Path::new("assets/background.png"), &texture_creator), &mut textures);

    // CREATING BLUE WITCH -- setup values
    
    let blue_witch_scl_fac = 10.5;

    // IDLE animation
    let blue_witch_idle_texture_idx = insert_texture(get_texture(Path::new("assets/Blue_witch/B_witch_idle.png"), &texture_creator), &mut textures);
    let blue_witch_idle_dest_rect = Rect::new(
        ((screen_width / 15) as f32 - ( 32.0 * blue_witch_scl_fac / 2.0 )) as i32,            // x
        ((screen_height as f32 / 1.5) as f32 - ( 48.0 * blue_witch_scl_fac / 2.0 )) as i32,   // y
        (32 as f32 * blue_witch_scl_fac) as u32,                                              // w
        (48 as f32 * blue_witch_scl_fac) as u32,                                              // h
    );                                             

    let one_twelfth_sec = Duration::new(0, 1_000_000_000 / 12);
    let blue_witch_idle_animation = vertical_animation_from_texture_idx(blue_witch_idle_texture_idx, 32, 48, 6, one_twelfth_sec);

    // CHARGING UP animation
    let blue_witch_charge_texture_idx = insert_texture(get_texture(Path::new("assets/Blue_witch/B_witch_charge.png"), &texture_creator), &mut textures);

    let blue_witch_charge_dest_rect = Rect::new(
        ((screen_width / 15) as f32 - ( 48.0 * blue_witch_scl_fac / 2.0 )) as i32,           // x
        ((screen_height as f32 / 1.5) as f32 - ( 48.0 * blue_witch_scl_fac / 2.0 )) as i32,  // y
        (48 as f32 * blue_witch_scl_fac) as u32,                                             // w
        (48 as f32 * blue_witch_scl_fac) as u32);                                            // h

    let blue_witch_charge_animation = vertical_animation_from_texture_idx(blue_witch_charge_texture_idx, 48, 48, 5, one_twelfth_sec);

    // RUNNING animation (not really needed i guess)
    let blue_witch_run_texture_idx = insert_texture(get_texture(Path::new("assets/Blue_witch/B_witch_run.png"), &texture_creator), &mut textures);

    let blue_witch_run_dest_rect = Rect::new(
        ((screen_width / 15) as f32 - ( 32.0 * blue_witch_scl_fac / 2.0 )) as i32,
        ((screen_height as f32 / 1.5) as f32 - ( 48.0 * blue_witch_scl_fac / 2.0)) as i32,
        (32 as f32 * blue_witch_scl_fac) as u32,
        (48 as f32 * blue_witch_scl_fac) as u32,
    );

    let blue_witch_run_animation = vertical_animation_from_texture_idx(blue_witch_run_texture_idx, 32, 48, 8, one_twelfth_sec);

    // Putting it all together
    let mut blue_witch_animations: Vec<Animation> = Vec::new();
    blue_witch_animations.push(blue_witch_idle_animation);
    blue_witch_animations.push(blue_witch_charge_animation);
    blue_witch_animations.push(blue_witch_run_animation);
    let mut blue_witch_dest_rects: Vec<Rect> = Vec::new();
    blue_witch_dest_rects.push(blue_witch_idle_dest_rect);
    blue_witch_dest_rects.push(blue_witch_charge_dest_rect);
    blue_witch_dest_rects.push(blue_witch_run_dest_rect);
    let blue_witch_character = Character::new(blue_witch_animations, blue_witch_dest_rects); 

    let mut characters: Vec<Character> = Vec::new();
    characters.push(blue_witch_character); 

    let mut state = State {
        characters,
        textures,
    };

    'let_there_be_light: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Option::Some(Keycode::Escape), .. } => break 'let_there_be_light,
                Event::KeyDown {keycode: Option::Some(Keycode::A), .. } => {
                    state.characters[0].current_dest_rect = 0;
                    state.characters[0].current_animation_idx = 0;
                },
                Event::KeyDown {keycode: Option::Some(Keycode::B), .. } => {
                    state.characters[0].current_dest_rect = 1;
                    state.characters[0].current_animation_idx = 1;
                },
                Event::KeyDown {keycode: Option::Some(Keycode::C), .. } => {
                    state.characters[0].current_dest_rect = 2;
                    state.characters[0].current_animation_idx = 2;
                }
                _ => {}
            }
        }  
        canvas.clear();
        canvas.copy(&state.textures[background_texture_idx], None, None).unwrap();
        for character in state.characters.iter_mut() {
            canvas.copy(&state.textures[character.animations[character.current_animation_idx].texture_idx], character.animations[character.current_animation_idx].get_current_frame(), character.dest_rects[character.current_dest_rect])?;
            character.animations[character.current_animation_idx].advance();
        }
        canvas.present();
    }

    Ok(())
}

//TODO write a function to create characters
//  so that we can think about adding more animations for the witch to do on certain key presses
//
//TODO we should have a gamestate, and then when we go to draw in the loop, we check what gamestate
//we currently have in order to draw the right things
