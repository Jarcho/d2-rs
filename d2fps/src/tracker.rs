use core::mem;
use d2interface::{all_versions::EntityKind, FixedI16, FixedU16, LinearPos};

#[derive(Clone, Copy)]
pub struct UnitId {
  pub kind: EntityKind,
  pub id: u32,
}
impl UnitId {
  pub const fn new(kind: EntityKind, id: u32) -> Self {
    Self { kind, id }
  }

  fn key(self) -> usize {
    // Divides the search space into four, one each of PC's, NPC's, and missiles.
    // There's a segment for objects as well, but they're never tracked.
    //
    // Each entity type starts in it's own segment, then rotates through
    // segments as the unit id increases. The cycles are:
    // PC & Object: PC -> NPC -> Object -> Missile
    // NPC & Missile: NPC -> PC -> Missile -> Object
    (self.id ^ self.kind.0 << 9) as usize
  }
}

#[derive(Clone, Copy)]
pub struct Position {
  pub real: LinearPos<FixedU16>,
  pub delta: LinearPos<FixedI16>,
  pub last_rendered: LinearPos<FixedU16>,
}
impl Position {
  pub fn for_time(&mut self, fract: FixedI16) -> LinearPos<FixedU16> {
    let x = ((self.delta.x.repr() as i64 * fract.repr() as i64) >> 16) as u32;
    let y = ((self.delta.y.repr() as i64 * fract.repr() as i64) >> 16) as u32;
    let x = self.real.x.repr().wrapping_add(x);
    let y = self.real.y.repr().wrapping_add(y);
    let pos = LinearPos::new(FixedU16::from_repr(x), FixedU16::from_repr(y));
    self.last_rendered = pos;
    pos
  }

  fn update_pos(&mut self, pos: LinearPos<FixedU16>, target: LinearPos<u16>) {
    let vx = pos.x.repr().wrapping_sub(self.real.x.repr()) as i32;
    let vy = pos.y.repr().wrapping_sub(self.real.y.repr()) as i32;
    self.real = pos;
    let tx = ((target.x as u32) << 16) | 0x8000;
    let ty = ((target.y as u32) << 16) | 0x8000;
    let lx = tx.wrapping_sub(pos.x.repr()) as i32;
    let ly = ty.wrapping_sub(pos.y.repr()) as i32;

    let dx = if lx < 0 { vx.max(lx) } else { vx.min(lx) };
    let dy = if ly < 0 { vy.max(ly) } else { vy.min(ly) };
    self.delta = LinearPos::new(FixedI16::from_repr(dx), FixedI16::from_repr(dy));
  }

  fn new(pos: LinearPos<FixedU16>) -> Self {
    Self {
      real: pos,
      last_rendered: pos,
      delta: LinearPos::new(FixedI16::default(), FixedI16::default()),
    }
  }
}

#[derive(Clone, Copy)]
struct Entry {
  distance: i16,
  kind: u16,
  id: u32,
}

const UNIT_COUNT: usize = 2048;

pub struct EntityTracker {
  // Tracks which entries have been updated since the last compaction cycle.
  active: [u32; UNIT_COUNT / 32],
  // Fixed-size, round-robin hash table. First array is the keys, second the values.
  entities: [Entry; UNIT_COUNT],
  positions: [Position; UNIT_COUNT],
}
impl EntityTracker {
  pub const fn new() -> Self {
    Self {
      active: [0; UNIT_COUNT / 32],
      entities: [Entry { distance: -1, kind: 0, id: 0 }; UNIT_COUNT],
      positions: [Position {
        real: LinearPos::new(FixedU16::from_repr(0), FixedU16::from_repr(0)),
        delta: LinearPos::new(FixedI16::from_repr(0), FixedI16::from_repr(0)),
        last_rendered: LinearPos::new(FixedU16::from_repr(0), FixedU16::from_repr(0)),
      }; UNIT_COUNT],
    }
  }

  fn probe_idx(&self, id: UnitId) -> (Option<i16>, usize) {
    let mut i = id.key() % UNIT_COUNT;
    let mut delta = 0i16;
    loop {
      let entry = self.entities[i];
      if self.entities[i].distance < delta {
        return (Some(delta), i);
      }
      if entry.kind == id.kind.0 as u16 && entry.id == id.id {
        return (None, i);
      }
      i = (i + 1) % UNIT_COUNT;
      delta += 1
    }
  }

  fn mark_active(&mut self, i: usize) {
    self.active[i >> 5] |= 1u32 << (i & 31);
  }

  fn shift(&mut self, mut i: usize) {
    let mut current = self.entities[i];
    let mut count = 0u16;
    while current.distance >= 0 && count < UNIT_COUNT as u16 {
      i = (i + 1) % UNIT_COUNT;
      count += 1;
      current.distance += 1;
      mem::swap(&mut current, &mut self.entities[i]);
    }

    while count > 0 {
      count -= 1;
      let prev = i.wrapping_sub(1) % UNIT_COUNT;
      self.positions[i] = self.positions[prev];
      i = prev;
    }
  }

  pub fn clear_unused(&mut self) {
    let mut dst = 0usize;
    for i in 0..self.active.len() {
      if self.active[i] == 0 {
        // Save bit scanning when not needed. Note the number of active entries
        // is likely to be low.
        continue;
      }
      for j in 0..32 {
        if self.active[i] & (1u32 << j) == 0 {
          // Skip over inactive entries
          continue;
        }

        let src = (i << 5) | j;
        let mut entry = self.entities[src];

        let mut distance = (src - dst) as i16;
        while distance > entry.distance {
          // While the destination is further back than the current entry can be
          // moved mark the destination as dead.
          self.entities[dst].distance = -1;
          distance -= 1;
          dst += 1;
        }

        if dst != src {
          // Move the current entry back.
          entry.distance -= distance;
          self.entities[dst] = entry;
          self.positions[dst] = self.positions[src];
        }
        dst += 1;
      }
    }

    // Mark any remaining entries as dead.
    for i in dst..UNIT_COUNT {
      self.entities[i].distance = -1;
    }

    self.active.fill(0);
  }

  pub fn get(&mut self, id: UnitId) -> Option<&mut Position> {
    if let (None, i) = self.probe_idx(id) {
      Some(&mut self.positions[i])
    } else {
      None
    }
  }

  pub fn insert_or_update(&mut self, id: UnitId, pos: LinearPos<FixedU16>, target: LinearPos<u16>) {
    match self.probe_idx(id) {
      (None, i) => {
        self.mark_active(i);
        self.positions[i].update_pos(pos, target)
      }
      (Some(distance), i) => {
        self.mark_active(i);
        self.shift(i);
        self.entities[i] = Entry { distance, kind: id.kind.0 as u16, id: id.id };
        self.positions[i] = Position::new(pos);
      }
    }
  }
}

#[test]
fn test_entity_tracker() {
  let mut tracker = EntityTracker::new();
  tracker.insert_or_update(
    UnitId { kind: EntityKind::Pc, id: 0 },
    LinearPos::new(FixedU16::default(), FixedU16::default()),
    LinearPos::new(0, 0),
  );
  tracker.clear_unused();
  assert!(tracker.get(UnitId::new(EntityKind::Pc, 0)).is_some());
  tracker.clear_unused();
  assert!(tracker.get(UnitId::new(EntityKind::Pc, 0)).is_none());
}
