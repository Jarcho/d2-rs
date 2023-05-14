use crossbeam_channel::{unbounded, Receiver, RecvTimeoutError, Sender};
use once_cell::sync::OnceCell;
use std::{
  fs,
  io::{BufWriter, Write as _},
  thread,
  time::Duration,
};

pub struct Logger {
  send: Sender<String>,
  recv: Receiver<String>,
}
impl Logger {
  fn new() -> Self {
    let (send, recv) = unbounded::<String>();
    let (ret_send, ret_recv) = unbounded();

    ret_send.send(String::with_capacity(1024)).unwrap();
    ret_send.send(String::with_capacity(1024)).unwrap();
    ret_send.send(String::with_capacity(1024)).unwrap();
    ret_send.send(String::with_capacity(1024)).unwrap();

    let _ = thread::Builder::new()
      .name("d2fps_logger".to_owned())
      .stack_size(4096)
      .spawn(move || {
        match fs::OpenOptions::new()
          .create(true)
          .truncate(true)
          .write(true)
          .open("d2fps.log")
        {
          Ok(file) => {
            let mut file = BufWriter::new(file);
            loop {
              if file.buffer().is_empty() {
                let Ok(msg) = recv.recv() else {
                  break;
                };
                if msg.is_empty() {
                  return;
                } else {
                  let _ = writeln!(file, "{msg}");
                }
                let _ = ret_send.send(msg);
              } else {
                match recv.recv_timeout(Duration::from_secs(1)) {
                  Ok(msg) => {
                    if msg.is_empty() {
                      return;
                    } else {
                      let _ = writeln!(file, "{msg}");
                    }
                    let _ = ret_send.send(msg);
                  }
                  Err(RecvTimeoutError::Timeout) => {
                    let _ = file.flush();
                  }
                  Err(RecvTimeoutError::Disconnected) => break,
                };
              }
            }
          }
          Err(_) => {
            for msg in recv {
              if msg.is_empty() {
                return;
              }
              let _ = ret_send.send(msg);
            }
          }
        }
      });

    Self { send, recv: ret_recv }
  }

  fn log(&self, f: impl FnOnce(&mut String)) {
    let mut buf = match self.recv.try_recv() {
      Ok(mut buf) => {
        buf.clear();
        buf
      }
      Err(_) => String::with_capacity(1024),
    };
    f(&mut buf);
    let _ = self.send.send(buf);
  }
}

static LOGGER: OnceCell<Logger> = OnceCell::new();
fn logger() -> &'static Logger {
  LOGGER.get_or_init(Logger::new)
}

pub fn log(f: impl FnOnce(&mut String)) {
  logger().log(f);
}

pub fn shutdown() {
  if let Some(log) = LOGGER.get() {
    log.send.send(String::new()).unwrap();
    // Just hope this is long enough...
    thread::sleep(Duration::from_millis(100));
  }
}
