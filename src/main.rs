use std::fs::OpenOptions;
use std::io::Write;

use cod::Key;

const HELP: &str = include_str!("help.txt");
const CONFIG_FILE: &str = "tuilemap.cfg";

#[derive(Default)]
struct Map {
    tiles: Vec<Vec<char>>,
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        Self {
            tiles: vec![vec![' '; width]; height],
        }
    }

    fn reset(&mut self, width: usize, height: usize) {
        self.tiles = vec![vec![' '; width]; height];
    }

    fn export(&self) -> String {
        self.tiles
            .iter()
            .map(String::from_iter)
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn draw(&self, x: usize, y: usize, mode: &Mode, dx: usize, dy: usize) {
        if let Mode::Visual { x: fx, y: fy } = mode {
            let (min, max) = if *fx > dx { (dx, *fx) } else { (*fx, dx) };
            let x_range = min..=max;
            let (min, max) = if *fy > dy { (dy, *fy) } else { (*fy, dy) };
            let y_range = min..=max;

            let sx = x;
            let sy = y;

            let width = self.tiles[0].len();
            let height = self.tiles.len();
            for y in 0..height {
                for x in 0..width {
                    if x_range.contains(&x) && y_range.contains(&y) && !(x == dx && y == dy) {
                        cod::color::fg(0);
                        cod::color::bg(7);
                    } else {
                        cod::color::de();
                    }

                    cod::pixel(self.tiles[y][x], (x + sx) as u32, (y + sy) as u32);
                }

                cod::color::de();
            }
        } else {
            cod::blit(self.export(), x as u32, y as u32);
        }

        cod::goto::pos((x + dx) as u32, (y + dy) as u32);
    }

    fn set(&mut self, x: usize, y: usize, tile: char) {
        self.tiles
            .get_mut(y)
            .and_then(|row| row.get_mut(x))
            .map(|ch| *ch = tile);
    }

    fn write(&mut self, x: usize, y: usize, tile: char, mode: &mut Mode) {
        if let Mode::Visual { x: fx, y: fy } = mode {
            let dx = x;
            let dy = y;

            let (min, max) = if *fx > dx { (dx, *fx) } else { (*fx, dx) };
            let x_range = min..=max;
            let (min, max) = if *fy > dy { (dy, *fy) } else { (*fy, dy) };
            let y_range = min..=max;
            for y in y_range {
                for x in x_range.clone() {
                    self.set(x, y, tile);
                }
            }

            *mode = Mode::Normal;
        } else {
            self.set(x, y, tile);
        }
    }
}

fn get_width_height(h: usize) -> Option<(usize, usize)> {
    cod::goto::pos(0, h as u32 + 2);
    cod::clear::line();
    println!("Width: ");
    let width = cod::read::line().and_then(|w| w.parse().ok())?;

    cod::goto::pos(0, h as u32 + 3);
    cod::clear::line();
    cod::goto::up(1);
    cod::clear::line();
    println!("Height: ");
    let height = cod::read::line().and_then(|h| h.parse().ok())?;

    Some((width, height))
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Mode {
    Normal,
    Visual { x: usize, y: usize },
}

#[derive(Default)]
struct Config {
    width: usize,
    height: usize,
    tileset: String,
}

fn read_cfg() -> Config {
    let mut cfg = Config {
        width: 8,
        height: 8,
        tileset: "abcdefgh".to_string(),
    };

    if let Ok(data) = std::fs::read_to_string(CONFIG_FILE) {
        for line in data.lines() {
            let line = line.trim();

            if let Some(width) = line.strip_prefix("width = ") {
                cfg.width = width.parse().unwrap_or(8);
            } else if let Some(height) = line.strip_prefix("height = ") {
                cfg.height = height.parse().unwrap_or(8);
            } else if let Some(tileset) = line.strip_prefix("tileset = ") {
                cfg.tileset = tileset.trim().to_string();
            }
        }
    }

    cfg
}

fn main() {
    let Config { mut width, mut height, mut tileset } = read_cfg();

    let mut x = 0;
    let mut y = 0;
    let mut mode = Mode::Normal;
    let mut map = Map::new(width, height);
    let mut edit = false;

    loop {
        cod::clear::all();

        cod::color::fg(3);
        cod::orth_line('|', width as u32, 0, width as u32, height as u32).unwrap();
        cod::orth_line('-', 0, height as u32, width as u32, height as u32).unwrap();

        cod::pixel(
            match (&mode, edit) {
                (Mode::Visual { .. }, _) => 'V',
                (_, true) => 'E',
                _ => '/',
            },
            width as u32,
            height as u32,
        );

        cod::blit("Tileset: ", 0, height as u32 + 1);
        cod::blit(&tileset, 9, height as u32 + 1);

        cod::color::de();
        map.draw(0, 0, &mode, x, y);

        cod::flush();

        let key = cod::read::key().expect("failed to get key");

        if let Key::Char(ch) = key {
            if edit {
                map.write(x, y, ch, &mut mode);
                x = (width - 1).min(x + 1);
                continue;
            }
        }

        match key {
            Key::Escape => {
                mode = Mode::Normal;
                edit = false;
            }
            Key::Char('i' | 'a') => edit = true,
            Key::Char('v' | 'V') => mode = Mode::Visual { x, y },

            Key::ArrowLeft => x = x.saturating_sub(1),
            Key::ArrowRight => x = (width - 1).min(x + 1),
            Key::ArrowUp => y = y.saturating_sub(1),
            Key::ArrowDown => y = (height - 1).min(y + 1),
            Key::Home | Key::Char('f') => x = 0,
            Key::End | Key::Char('F') => x = width - 1,
            Key::Char('g') => y = 0,
            Key::Char('G') => y = height - 1,
            Key::Backspace => {
                map.set(x, y, ' ');
                x = x.saturating_sub(1);
            }

            Key::Char(' ') => map.write(x, y, ' ', &mut mode),
            Key::Char('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9') => {
                let Key::Char(ch) = key else {
                    unreachable!();
                };

                let idx = match ch.to_digit(10).unwrap() {
                    0 => 9,
                    i => i - 1,
                };

                if let Some(tile) = tileset.chars().nth(idx as usize) {
                    map.set(x, y, tile);
                    map.write(x, y, tile, &mut mode);
                }
            }

            Key::Char('s') => {
                cod::goto::pos(0, height as u32 + 2);
                println!("Filename: ");
                let file = cod::read::line().unwrap();

                if file == "cancel" {
                    continue;
                }

                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .create(true)
                    .open(file)
                    .expect("Failed to open file");

                file.write_all(map.export().as_bytes())
                    .expect("Failed to write to file");
            }

            Key::Char('n') => {
                let (new_width, new_height) = match get_width_height(height) {
                    Some(dims) => dims,
                    None => {
                        cod::color::fg(1);
                        println!("Invalid number");
                        cod::read::key();
                        cod::color::de();
                        continue;
                    }
                };

                width = new_width;
                height = new_height;
                map.reset(width, height);
                x = 0;
                y = 0;
            }

            Key::Char('l') => {
                cod::goto::pos(0, height as u32 + 2);
                println!("Filename: ");
                let file = cod::read::line().unwrap();

                if file == "cancel" {
                    continue;
                }

                let data = std::fs::read_to_string(file).expect("Failed to read file");

                let width = data.split('\n').next().map_or(0, |l| l.len());
                let tiles = data
                    .lines()
                    .map(|l| {
                        l.chars()
                            // pad lines that are too short...
                            .chain(std::iter::repeat(' ').take(width))
                            // ...without becoming too large
                            .take(width)
                            .collect::<Vec<char>>()
                    })
                    .collect::<Vec<_>>();

                map.tiles = tiles;

                x = 0;
                y = 0;
            }

            Key::Char('h') => {
                cod::clear::all();
                cod::color::de();
                cod::blit(HELP, 0, 0);
                cod::goto::bot();
                cod::read::key();
            }

            Key::Char('t') => {
                cod::goto::pos(0, height as u32 + 2);
                println!("New tileset:");
                if let Some(line) = cod::read::line() {
                    tileset = line;
                }
            }

            Key::Char('\x11') | Key::Char('q') => {
                if mode == Mode::Normal {
                    cod::goto::pos(0, height as u32 + 2);
                    print!("Really quit? (y/N) ");
                    cod::flush();
                    if cod::read::line()
                        .map_or(false, |l| l.starts_with(|ch| ch == 'y' || ch == 'Y'))
                    {
                        break;
                    }
                }
            }
            _ => {}
        }
    }

    cod::clear::all();
}
