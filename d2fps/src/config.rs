use crate::{
  features::{AtomicFeatures, Features},
  util::AtomicRatio,
  GAME_FPS,
};
use std::{env, fs};

pub(crate) struct Config {
  pub fps: AtomicRatio,
  pub bg_fps: AtomicRatio,
  pub features: AtomicFeatures,
}
impl Config {
  pub const fn new() -> Self {
    Self {
      fps: AtomicRatio::ZERO,
      bg_fps: AtomicRatio::new(GAME_FPS),
      features: AtomicFeatures::ALL,
    }
  }

  pub fn load_config(&self) {
    if let Ok(file) = fs::read_to_string("d2fps.ini") {
      for (i, line) in file
        .lines()
        .enumerate()
        .map(|(i, x)| (i + 1, x.trim_start()))
        .filter(|&(_, x)| !(x.is_empty() || x.as_bytes()[0] == b'#'))
      {
        if let Some((k, v)) = line.split_once('=') {
          let v = v.split_once('#').map_or(v, |(x, _)| x);
          let v = v.trim();
          match k.trim_end() {
            "fps" => match v.parse() {
              Ok(v) => self.fps.store_relaxed(v),
              Err(_) => log!("Error parsing d2fps.ini: line `{i}`: unknown value `{v}`"),
            },
            "bg-fps" => match v.parse() {
              Ok(v) => self.bg_fps.store_relaxed(v),
              Err(_) => log!("Error parsing d2fps.ini: line `{i}`: unknown value `{v}`"),
            },
            "menu-fps" => match v.parse() {
              Ok(v) => self.features.set_relaxed(Features::MenuFps, v),
              Err(_) => log!("Error parsing d2fps.ini: line `{i}`: unknown value `{v}`"),
            },
            "game-fps" => match v.parse() {
              Ok(v) => self.features.set_relaxed(Features::GameFps, v),
              Err(_) => log!("Error parsing d2fps.ini: line `{i}`: unknown value `{v}`"),
            },
            "motion-smoothing" => match v.parse() {
              Ok(v) => self.features.set_relaxed(Features::MotionSmoothing, v),
              Err(_) => log!("Error parsing d2fps.ini: line `{i}`: unknown value `{v}`"),
            },

            k => {
              log!("Error parsing d2fps.ini: line `{i}`: unknown key `{k}`");
            }
          }
        } else {
          log!("Error parsing d2fps.ini: line `{i}`: key with no value `{line}`");
        }
      }
    }

    for arg in env::args_os() {
      if let Some(arg) = arg.to_str() {
        if let Some((k, v)) = arg.split_once('=') {
          match k {
            "-fps" => match v.parse() {
              Ok(v) => self.fps.store_relaxed(v),
              Err(_) => log!("Error parsing argument `fps`: unknown value `{v}`"),
            },
            "-bg-fps" => match v.parse() {
              Ok(v) => self.bg_fps.store_relaxed(v),
              Err(_) => log!("Error parsing argument `bg-fps`: unknown value `{v}`"),
            },
            "-fmenu-fps" => match v.parse() {
              Ok(v) => self.features.set_relaxed(Features::MenuFps, v),
              Err(_) => log!("Error parsing argument `pmenu-fps`: unknown value `{v}`"),
            },
            "-fgame-fps" => match v.parse() {
              Ok(v) => self.features.set_relaxed(Features::GameFps, v),
              Err(_) => log!("Error parsing argument `pgame-fps`: unknown value `{v}`"),
            },
            "-fmotion-smoothing" => match v.parse() {
              Ok(v) => self.features.set_relaxed(Features::MotionSmoothing, v),
              Err(_) => log!("Error parsing argument `pmotion-smoothing`: unknown value `{v}`"),
            },
            _ => {}
          }
        }
      }
    }

    log!(
      "Loaded config:\n  fps: {}\n  bg-fps: {}\n  features: {}",
      self.fps.load_relaxed(),
      self.bg_fps.load_relaxed(),
      self.features.load_relaxed(),
    );
  }
}
