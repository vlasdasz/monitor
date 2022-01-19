use std::{
    fs::{File, OpenOptions},
    io::Write,
    time::Duration,
};

use sysinfo::{System, SystemExt};
use tokio::{time::interval, signal::ctrl_c};

use crate::info::Info;

pub struct Monitor {
    system: System,
    data:   Vec<Info>,
}

impl Monitor {
    fn on_get(&mut self) {
        self.data.push(Info::current(&mut self.system));
    }

    fn on_store(&mut self) {

        let mut file = self.open_file();

        file.write_all(b"Hello, world!\n").unwrap();




        let ptr = self.data.as_ptr();

        let sok = ptr as *const u8;

        //let kak = sok as &[u8];

      //  self.data.encode()

     //   let orr = [0; self.data.len()];

        dbg!(self.data.len());

        //self.data.

        self.data.clear();
    }

    fn open_file(&self) -> File {
        OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("my-file-spika.txt")
            .unwrap()
    }

    pub async fn start_monitoring(&mut self) {
        let mut get = interval(Duration::from_millis(500));
        let mut store = interval(Duration::from_secs(10));

        loop {
            tokio::select! {
                _ = get.tick() => {
                    self.on_get()
                }
                _ = store.tick() => {
                    self.on_store()
                }
            }
        }
    }
}

impl Default for Monitor {
    fn default() -> Self {
        Self {
            system: System::new_all(),
            data:   vec![],
        }
    }
}
