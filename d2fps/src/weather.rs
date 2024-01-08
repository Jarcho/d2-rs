use crate::{InstanceSync, INSTANCE};
use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
use d2interface as d2;

#[derive(Default, Clone, Copy)]
pub(crate) struct Particle {
  target_pos: d2::ScreenPos<i32>,
  delta: d2::ScreenPos<i32>,
}

pub(crate) unsafe fn update_weather(
  rng: &mut d2::Rng,
  env_shift: d2::IsoPos<i32>,
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
  let fract = d2::FixedI16::from_wrapping(1) - sync_instance.unit_movement_fract;

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

      particle.pos = d2::ScreenPos::new(
        particle.pos.x.wrapping_add(env_shift.x),
        particle.pos.y.wrapping_add(env_shift.y),
      );

      if particle.pos.y >= particle.end_y_pos || particle.pos.y < -20 {
        particle.at_end = true.into();
        particle.speed = 0;
        ex.delta = d2::ScreenPos::default();
      }

      if particle.at_end.bool() {
        if particle.frames_remaining == 0 {
          sync_instance
            .accessor
            .env_array_remove(particles.cast(), (i << 16) as u32);
          if (*particles).active_count < (*sync_instance.accessor.max_weather_particles) {
            sync_instance.accessor.gen_weather_particle(rng);
          }
        } else {
          particle.frames_remaining -= u32::from(INSTANCE.client_updated.load(Relaxed));
        }
      }

      let particle = &mut *ptr;
      if !particle.at_end.bool() {
        let speed = particle.speed as f64 * speed_mod;
        let delta_x = (angle_cos * speed) as i32;
        let delta_y = (angle_sin * speed) as i32;
        let x = particle.pos.x.wrapping_add(delta_x);
        let y = particle.pos.y.wrapping_add(delta_y);

        if is_snowing {
          particle.pos.x = particle.pos.x.wrapping_add(
            (sync_instance
              .accessor
              .sin(d2::FixedU8::from_repr(SNOW_SIN_ANGLE.load(Relaxed)) + particle.angle)
              as f64
              * 2.0) as i32,
          );
        }

        let x = (x.wrapping_add(view_size.width as i32) % view_size.width as i32).wrapping_abs();
        ex.target_pos = d2::ScreenPos::new(x, y);
        ex.delta = d2::ScreenPos::new(
          particle.pos.x.wrapping_sub(x),
          particle.pos.y.wrapping_sub(y),
        );
        particle.pos = d2::ScreenPos::new(
          ex.target_pos.x + ((ex.delta.x * fract.repr()) >> 16),
          ex.target_pos.y + ((ex.delta.y * fract.repr()) >> 16),
        );
      }
    }
    i += 1;
    ptr = ptr.offset(1);
  }

  SNOW_SIN_ANGLE.fetch_add(17, Relaxed);
}

pub(crate) unsafe fn apply_weather_delta(
  env_shift: d2::IsoPos<i32>,
  sync_instance: &mut InstanceSync,
) {
  let particles = (*sync_instance.accessor.env_effects).particles;
  let fract = d2::FixedI16::from_wrapping(1) - sync_instance.unit_movement_fract;

  let mut ptr = (*particles).data.as_ptr();
  let mut i = 0;
  while i <= (*particles).last_active_idx {
    let particle = &mut *ptr;
    if particle.active.bool() && !particle.at_end.bool() {
      particle.pos = d2::ScreenPos::new(
        particle.pos.x.wrapping_add(env_shift.x),
        particle.pos.y.wrapping_add(env_shift.y),
      );

      let Some(ex) = sync_instance.weather_particles.get_mut(i as usize) else {
        i += 1;
        ptr = ptr.offset(1);
        continue;
      };
      if ex.delta.x != 0 || ex.delta.y != 0 {
        ex.target_pos = d2::ScreenPos::new(
          ex.target_pos.x.wrapping_add(env_shift.x),
          ex.target_pos.y.wrapping_add(env_shift.y),
        );

        particle.pos = d2::ScreenPos::new(
          ex.target_pos.x + ((ex.delta.x * fract.repr()) >> 16),
          ex.target_pos.y + ((ex.delta.y * fract.repr()) >> 16),
        );
      }
    }

    i += 1;
    ptr = ptr.offset(1);
  }
}
