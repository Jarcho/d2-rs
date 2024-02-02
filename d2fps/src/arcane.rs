use crate::Rng;
use crate::{hooks::GameAccessor, INSTANCE};
use num::M2d;
use rand::distributions::{Distribution, Uniform};

struct Star {
  x: f32,
  y: u32,
  speed: f32,
  color: u8,
}

pub(crate) struct ArcaneBg {
  stars: Vec<Star>,
  size: M2d<u32>,
  x_sampler: Uniform<f32>,
  y_sampler: Uniform<u32>,
  speed_sampler: Uniform<f32>,
  color_sampler: Uniform<u8>,
  colors: [u8; 11],
}
impl ArcaneBg {
  pub unsafe fn new() -> Self {
    Self {
      stars: Vec::new(),
      size: M2d::new(0, 0),
      x_sampler: Uniform::new(0.0, 1.0),
      y_sampler: Uniform::new(0, 1),
      speed_sampler: Uniform::new(
        INSTANCE.perf_freq.per_ms_to_per_tick(1.0 / 40.0),
        INSTANCE.perf_freq.per_ms_to_per_tick(5.0 / 40.0),
      ),
      color_sampler: Uniform::new(0, 11),
      colors: [0; 11],
    }
  }

  pub unsafe fn draw(&mut self, rng: &mut Rng, accessor: &GameAccessor, ticks: u64) {
    let size = accessor.viewport_size();
    let shift = *accessor.viewport_shift;
    if size != self.size {
      if self.size.x == 0 {
        let white = accessor.find_closest_color(0xff, 0xff, 0xff);
        self.colors = [
          accessor.find_closest_color(0x70, 0x70, 0x70),
          accessor.find_closest_color(0x90, 0x90, 0x90),
          accessor.find_closest_color(0xa0, 0xa0, 0xa0),
          accessor.find_closest_color(0xc8, 0xc8, 0xa0),
          accessor.find_closest_color(0xff, 0xd0, 0x80),
          accessor.find_closest_color(0xf0, 0xe4, 0xff),
          accessor.find_closest_color(0xf8, 0xf8, 0xe0),
          white,
          white,
          white,
          white,
        ];
      }

      self.size = size;
      self.x_sampler = Uniform::new(0.0, size.x as f32);
      self.y_sampler = Uniform::new(0, size.y);
      let count = (size.x * size.y / 2048) + ((size.x + size.y) / 32) + 0x40;
      self.stars.resize_with(count as usize, || Star {
        x: self.x_sampler.sample(rng),
        y: self.y_sampler.sample(rng),
        speed: self.speed_sampler.sample(rng),
        color: self.colors[self.color_sampler.sample(rng) as usize],
      });
    }

    for star in &mut self.stars {
      accessor.draw_line(
        star.x as i32 + shift,
        star.y as i32,
        star.x as i32 + shift,
        star.y as i32,
        star.color,
        0xff,
      );
      star.x -= star.speed * ticks as f32;
      if star.x < 0.0 {
        *star = Star {
          x: size.x as f32,
          y: self.y_sampler.sample(rng),
          speed: self.speed_sampler.sample(rng),
          color: self.colors[self.color_sampler.sample(rng) as usize],
        };
      }
    }
  }
}
