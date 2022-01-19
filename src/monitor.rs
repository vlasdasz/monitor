use std::time::Duration;

use sysinfo::{System, SystemExt};
use tokio::time::interval;

use crate::info::Info;

pub struct Monitor {
    system: System,
    data:   Vec<Info>,
}

impl Monitor {
    fn on_get(&mut self) {
        self.data.push(Info::current(&mut self.system));
    }

    fn on_store(&mut self) {}

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
