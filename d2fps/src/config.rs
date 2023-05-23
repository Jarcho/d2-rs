use crate::util::Ratio;
use std::{env, fs};

pub(crate) struct Config {
  pub fps: Option<Ratio>,
  pub bg_fps: Option<Ratio>,
  pub enable_smoothing: bool,
}
impl Config {
  pub const fn new() -> Self {
    Self { fps: None, bg_fps: None, enable_smoothing: true }
  }

  pub fn load_config(&mut self) {
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
              Ok(v) => self.fps = Some(v),
              Err(_) => log!("Error parsing d2fps.ini: line `{i}`, unknown value `{v}`"),
            },
            "bg-fps" => match v.parse() {
              Ok(v) => self.bg_fps = Some(v),
              Err(_) => log!("Error parsing d2fps.ini: line `{i}`, unknown value `{v}`"),
            },
            "enable-smoothing" => match v.parse() {
              Ok(v) => self.enable_smoothing = v,
              Err(_) => log!("Error parsing d2fps.ini: line `{i}`, unknown value `{v}`"),
            },
            k => {
              log!("Error parsing d2fps.ini: line `{i}`, unknown key `{k}`");
            }
          }
        } else {
          log!("Error parsing d2fps.ini: line `{i}`, key with no value `{line}`");
        }
      }
    }

    for arg in env::args_os() {
      if let Some(arg) = arg.to_str() {
        if let Some((k, v)) = arg.split_once('=') {
          match k {
            "-fps" => match v.parse() {
              Ok(v) => self.fps = Some(v),
              Err(_) => log!("Error parsing argument `fps`: unknown value `{v}`"),
            },
            "-bg-fps" => match v.parse() {
              Ok(v) => self.bg_fps = Some(v),
              Err(_) => log!("Error parsing argument `bg-fps`: unknown value `{v}`"),
            },
            "-enable-smoothing" => match v.parse() {
              Ok(v) => self.fps = Some(v),
              Err(_) => log!("Error parsing argument `enable-smoothing`: unknown value `{v}`"),
            },
            _ => {}
          }
        }
      }
    }

    if self.fps.map_or(false, |x| x.num == 0) {
      self.fps = None;
    }
    if self.bg_fps.map_or(false, |x| x.num == 0) {
      self.bg_fps = None;
    }

    log!(
      "Loaded config:\n  fps: {:?}\n  bg-fps: {:?}\n  enable-smoothing: {}",
      self.fps,
      self.bg_fps,
      self.enable_smoothing,
    );
  }
}
