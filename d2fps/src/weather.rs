use crate::InstanceSync;
use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
use d2interface as d2;
use num::{MulTrunc, WrappingAdd, WrappingFrom};

#[derive(Default, Clone, Copy)]
pub(crate) struct Particle {
  target_pos: d2::ScreenPos<i32>,
  delta: d2::ScreenPos<i32>,
}

pub(crate) unsafe fn update_weather(
  rng: &mut d2::Rng,
  env_shift: d2::ScreenPos<i32>,
  sync_instance: &mut InstanceSync,
) {
  static SNOW_SIN_ANGLE: AtomicU32 = AtomicU32::new(0);

  let particles = (*sync_instance.accessor.env_effects).particles;
  if (*particles).last_active_idx == 0 {
    for ex in &mut sync_instance.weather_particles {
      *ex = Particle::default();
    }
    return;
  }

  let angle = *sync_instance.accessor.weather_angle;
  let is_snowing = sync_instance.accessor.is_snowing();
  let view_size = sync_instance.accessor.viewport_size();
  let fract = d2::FI16::wfrom(1) - sync_instance.unit_movement_fract;

  let angle_sin = sync_instance.accessor.sin(angle) as f64;
  let angle_cos = sync_instance.accessor.cos(angle) as f64;
  let speed_mod = if is_snowing {
    angle_cos.abs().max(0.25)
  } else {
    *sync_instance.accessor.rain_speed as f64 * 0.15000000596046448 + 0.8500000238418579
  };

  let mut ptr = (*particles).data.as_ptr();
  let mut i = 0;
  while i <= (*particles).last_active_idx {
    let particle = &mut *ptr;
    if particle.active.bool() {
      let ex = match sync_instance.weather_particles.get_mut(i as usize) {
        Some(ex) => ex,
        None => {
          sync_instance
            .weather_particles
            .resize_with(i as usize + 1, Particle::default);
          &mut sync_instance.weather_particles[i as usize]
        }
      };

      if ex.delta.x != 0 || ex.delta.y != 0 {
        particle.pos = ex.target_pos;
      }

      particle.pos = particle.pos.wadd(env_shift);
      particle.pos.x = particle
        .pos
        .x
        .map(|x| (x.wrapping_add(view_size.width as i32) % view_size.width as i32).wrapping_abs());

      if particle.pos.y >= particle.end_y_pos || particle.pos.y < -20 {
        particle.at_end = true.into();
        particle.speed = 0;
        ex.delta = d2::Pos::default();
      }

      if particle.at_end.bool() {
        let (remaining, remove) = particle.frames_remaining.overflowing_sub(1);
        particle.frames_remaining = remaining;
        if remove {
          sync_instance
            .accessor
            .env_array_remove(particles.cast(), (i << 16) as u32);
          if (*particles).active_count < (*sync_instance.accessor.max_weather_particles) {
            sync_instance.accessor.gen_weather_particle(rng);
          }
        }
      }

      let particle = &mut *ptr;
      if !particle.at_end.bool() {
        let speed = particle.speed as f64 * speed_mod;
        let delta = d2::ScreenPos::new(
          d2::Measure::new((angle_cos * speed) as i32),
          d2::Measure::new((angle_sin * speed) as i32),
        );
        let mut pos = particle.pos.wadd(delta);

        if is_snowing {
          pos.x = pos.x.wadd(d2::Measure::new(
            (sync_instance
              .accessor
              .sin(d2::FU8::from_repr(SNOW_SIN_ANGLE.load(Relaxed)) + particle.angle)
              * 2.0) as i32,
          ));
        }

        ex.delta = particle.pos - pos;
        ex.target_pos = pos;
        particle.pos = pos + ex.delta.mul_trunc(fract);
        particle.pos.x = particle.pos.x.map(|x| {
          (x.wrapping_add(view_size.width as i32) % view_size.width as i32).wrapping_abs()
        });
      }
    }
    i += 1;
    ptr = ptr.offset(1);
  }

  SNOW_SIN_ANGLE.fetch_add(17, Relaxed);
}

pub(crate) unsafe fn apply_weather_delta(
  env_shift: d2::ScreenPos<i32>,
  sync_instance: &mut InstanceSync,
) {
  let particles = (*sync_instance.accessor.env_effects).particles;
  let fract = d2::FI16::wfrom(1) - sync_instance.unit_movement_fract;
  let view_size = sync_instance.accessor.viewport_size();

  let mut ptr = (*particles).data.as_ptr();
  let mut i = 0;
  while i <= (*particles).last_active_idx {
    let particle = &mut *ptr;
    if particle.active.bool() && !particle.at_end.bool() {
      particle.pos += env_shift;

      let Some(ex) = sync_instance.weather_particles.get_mut(i as usize) else {
        i += 1;
        ptr = ptr.offset(1);
        continue;
      };
      if ex.delta.x != 0 || ex.delta.y != 0 {
        ex.target_pos = ex.target_pos.wadd(env_shift);
        particle.pos = ex.target_pos + ex.delta.mul_trunc(fract);
        particle.pos.x = particle.pos.x.map(|x| {
          (x.wrapping_add(view_size.width as i32) % view_size.width as i32).wrapping_abs()
        });
      }
    }

    i += 1;
    ptr = ptr.offset(1);
  }
}