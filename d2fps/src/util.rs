use core::{
  fmt,
  mem::{size_of, zeroed},
  num::{NonZeroU32, NonZeroU64},
  ptr::null_mut,
  str::FromStr,
};
use gcd::Gcd;
use std::{
  ffi::{OsStr, OsString},
  fs,
  os::windows::prelude::{OsStrExt, OsStringExt},
};
use windows_sys::{
  w,
  Win32::{
    Devices::Display::{
      DisplayConfigGetDeviceInfo, GetDisplayConfigBufferSizes, QueryDisplayConfig,
      DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME, DISPLAYCONFIG_SOURCE_DEVICE_NAME,
      QDC_ONLY_ACTIVE_PATHS,
    },
    Foundation::{ERROR_INSUFFICIENT_BUFFER, ERROR_SUCCESS, HMODULE},
    Graphics::Gdi::{GetMonitorInfoW, HMONITOR, MONITORINFOEXW},
    Storage::FileSystem::{
      GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW, VS_FIXEDFILEINFO,
    },
    System::{
      LibraryLoader::{FreeLibrary, LoadLibraryW},
      Performance::QueryPerformanceFrequency,
      ProcessStatus::{EnumProcessModules, GetModuleFileNameExW},
      Threading::GetCurrentProcess,
    },
    UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR},
  },
};
use xxhash_rust::xxh3::xxh3_64;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ratio {
  pub num: u32,
  pub den: NonZeroU32,
}
impl Ratio {
  pub const fn new(num: u32, den: NonZeroU32) -> Self {
    Self { num, den }
  }

  pub fn reduced(self) -> Self {
    let d = self.num.gcd_binary(self.den.get());
    NonZeroU32::new(self.den.get() / d).map_or(self, |den| Self { num: self.num / d, den })
  }
}
impl FromStr for Ratio {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.split_once('/') {
      Some((n, d)) => {
        let num: u32 = n.parse().map_err(|_| ())?;
        let den: u32 = d.parse().map_err(|_| ())?;
        let den = NonZeroU32::new(den).ok_or(())?;
        Ok(Ratio { num, den }.reduced())
      }
      None => Ok(Ratio {
        num: s.parse().map_err(|_| ())?,
        den: NonZeroU32::new(1).ok_or(())?,
      }),
    }
  }
}
impl fmt::Display for Ratio {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.num == 0 || self.den.get() == 1 {
      self.num.fmt(f)
    } else {
      write!(f, "{}/{}", self.num, self.den)
    }
  }
}

#[derive(Clone, Copy)]
pub struct PerfFreq {
  for_s: NonZeroU64,
  for_ms: NonZeroU64,
}
impl PerfFreq {
  pub const fn uninit() -> Self {
    Self {
      for_s: unsafe { NonZeroU64::new_unchecked(1000) },
      for_ms: unsafe { NonZeroU64::new_unchecked(1) },
    }
  }

  pub fn init(&mut self) -> bool {
    let mut freq = 0i64;
    if unsafe { QueryPerformanceFrequency(&mut freq) } == 0 || freq < 1000 {
      return false;
    }
    self.for_s = NonZeroU64::new(freq as u64).unwrap();
    self.for_ms = NonZeroU64::new(freq as u64 / 1000).unwrap();

    true
  }

  pub fn s_to_sample(&self, s: u64) -> u64 {
    s * self.for_s.get()
  }

  pub fn sample_to_ms(&self, sample: u64) -> u64 {
    sample / self.for_ms.get()
  }

  pub fn ms_to_sample(&self, ms: u64) -> u64 {
    ms * self.for_ms.get()
  }
}

pub struct Module(pub HMODULE);
impl Module {
  pub unsafe fn new(name: *const u16) -> Result<Self, ()> {
    let x = LoadLibraryW(name);
    if x == 0 {
      Err(())
    } else {
      Ok(Module(x))
    }
  }
}
impl Drop for Module {
  fn drop(&mut self) {
    unsafe {
      FreeLibrary(self.0);
    }
  }
}

unsafe fn wcs_iter(s: *const u16) -> impl Iterator<Item = u16> {
  struct I(*const u16);
  impl Iterator for I {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
      unsafe {
        let x = *self.0;
        if x == 0 {
          None
        } else {
          self.0 = self.0.offset(1);
          Some(x)
        }
      }
    }
  }
  I(s)
}

pub unsafe fn monitor_refresh_rate(mon: HMONITOR) -> Option<Ratio> {
  let mut info = zeroed::<MONITORINFOEXW>();
  info.monitorInfo.cbSize = size_of::<MONITORINFOEXW>() as u32;
  if GetMonitorInfoW(mon, &mut info.monitorInfo) == 0 {
    return None;
  }

  let mut path_count: u32 = 0;
  let mut mode_count: u32 = 0;
  let mut paths = Vec::new();
  let mut modes = Vec::new();

  loop {
    if GetDisplayConfigBufferSizes(QDC_ONLY_ACTIVE_PATHS, &mut path_count, &mut mode_count)
      != ERROR_SUCCESS
    {
      return None;
    }

    paths.reserve_exact(path_count as usize);
    modes.reserve_exact(mode_count as usize);

    match QueryDisplayConfig(
      QDC_ONLY_ACTIVE_PATHS,
      &mut path_count,
      paths.as_mut_ptr(),
      &mut mode_count,
      modes.as_mut_ptr(),
      null_mut(),
    ) {
      ERROR_SUCCESS => break,
      ERROR_INSUFFICIENT_BUFFER => continue,
      _ => return None,
    }
  }

  paths.set_len(path_count as usize);
  modes.set_len(mode_count as usize);

  paths
    .iter()
    .find(|p| {
      let mut name = zeroed::<DISPLAYCONFIG_SOURCE_DEVICE_NAME>();
      name.header.r#type = DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME;
      name.header.size = size_of::<DISPLAYCONFIG_SOURCE_DEVICE_NAME>() as u32;
      name.header.adapterId = p.sourceInfo.adapterId;
      name.header.id = p.sourceInfo.id;

      DisplayConfigGetDeviceInfo(&mut name.header) as u32 == ERROR_SUCCESS
        && wcs_iter(info.szDevice.as_ptr()).eq(wcs_iter(name.viewGdiDeviceName.as_ptr()))
    })
    .and_then(|p| {
      let den = NonZeroU32::new(p.targetInfo.refreshRate.Denominator)?;
      if p.targetInfo.refreshRate.Numerator == 0 {
        None
      } else {
        Some(Ratio::new(p.targetInfo.refreshRate.Numerator, den))
      }
    })
}

pub fn message_box_error(msg: &str) {
  let mut msg: Vec<u16> = OsStr::new(&msg).encode_wide().collect();
  msg.push(0);
  unsafe {
    MessageBoxW(0, msg.as_ptr(), w!("D2fps Error"), MB_ICONERROR);
  }
}

pub fn log_loaded_modules() {
  let process = unsafe { GetCurrentProcess() };
  let mut modules = [0; 256];
  let mut size = 0;
  if unsafe {
    EnumProcessModules(
      process,
      modules.as_mut_ptr(),
      (modules.len() * size_of::<HMODULE>()) as u32,
      &mut size,
    )
  } == 0
  {
    return;
  }

  log!("Loaded modules:");
  let mut buf = [0; 260];
  for &module in &modules[..size as usize / size_of::<HMODULE>()] {
    let len = unsafe { GetModuleFileNameExW(process, module, buf.as_mut_ptr(), 260) };
    if len != 0 {
      if let Some(name) = OsString::from_wide(&buf[..len as usize]).to_str() {
        if !name
          .get(.."C:\\Windows\\".len())
          .map_or(false, |s| s.eq_ignore_ascii_case("C:\\Windows\\"))
        {
          if let Ok(version) = unsafe { read_file_version(buf.as_ptr()) } {
            log!("  {name} (v{version})");
          } else {
            log!("  {name}");
          }
        }
      }
    }
  }
}

pub fn hash_module_file(module: HMODULE) -> Option<u64> {
  if module == 0 {
    return None;
  }
  let process = unsafe { GetCurrentProcess() };
  let mut buf = [0; 260];
  let len = unsafe { GetModuleFileNameExW(process, module, buf.as_mut_ptr(), 260) };
  if len != 0 {
    if let Some(name) = OsString::from_wide(&buf[..len as usize]).to_str() {
      if let Ok(buf) = fs::read(name) {
        return Some(xxh3_64(&buf));
      }
    }
  }
  None
}

pub struct FileVersion {
  pub ms: u32,
  pub ls: u32,
}
impl fmt::Display for FileVersion {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}.{}.{}.{}",
      self.ms >> 16,
      self.ms & 0xFFFF,
      self.ls >> 16,
      self.ls & 0xFFFF
    )
  }
}

pub unsafe fn read_file_version(file: *const u16) -> Result<FileVersion, ()> {
  let len = GetFileVersionInfoSizeW(file, null_mut());
  let mut buf = Vec::<u8>::with_capacity(len as usize);

  if GetFileVersionInfoW(file, 0, len, buf.as_mut_ptr().cast()) == 0 {
    return Err(());
  }
  buf.set_len(len as usize);

  let mut len = 0u32;
  let mut out = null_mut::<u8>();
  if VerQueryValueW(
    buf.as_mut_ptr().cast(),
    w!("\\"),
    (&mut out as *mut *mut u8).cast(),
    &mut len,
  ) == 0
    || (len as usize) < size_of::<VS_FIXEDFILEINFO>()
  {
    return Err(());
  }

  let info = &*out.cast::<VS_FIXEDFILEINFO>();
  Ok(FileVersion { ls: info.dwFileVersionLS, ms: info.dwFileVersionMS })
}
