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
      for line in file
        .lines()
        .map(|x| x.trim_start())
        .filter(|x| x.is_empty() || x.as_bytes()[0] == b'#')
      {
        let Some((k, v)) = line.split_once('=') else {
          panic!();
        };
        let v = v.split_once('#').map_or(v, |(x, _)| x);
        let v = v.trim();
        match k {
          "fps" => self.fps = Some(v.parse().unwrap()),
          "bg-fps" => self.bg_fps = Some(v.parse().unwrap()),
          "enable-smoothing" => self.enable_smoothing = v.parse().unwrap(),
          _ => panic!(),
        }
      }
    }

    for arg in env::args_os() {
      if let Some(arg) = arg.to_str() {
        if let Some((k, v)) = arg.split_once('=') {
          match k {
            "-fps" => self.fps = Some(v.parse().unwrap()),
            "-bg-fps" => self.bg_fps = Some(v.parse().unwrap()),
            "-enable-smoothing" => self.enable_smoothing = v.parse().unwrap(),
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
  }
}
