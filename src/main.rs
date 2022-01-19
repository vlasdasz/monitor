use sysinfo::{DiskExt, NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use tokio::time::{interval, sleep, Duration};

#[derive(Debug)]
struct Info {
    //ram:  u16,
    disk: u32,
}

impl Info {
    pub fn current(sys: &mut System) -> Self {
        sys.refresh_all();

        let disk = match sys.disks().first() {
            Some(val) => val.available_space() / 1024 / 1024,
            None => 0,
        } as _;

        //let ram = ((sys.total_memory() - sys.used_memory()) / 1024) as u16;

        Self { /*ram,*/ disk }
    }
}

fn on_get(sys: &mut System) {
    dbg!(Info::current(sys));
}

fn on_store() {

}

#[tokio::main]
async fn main() {
    let mut sys = System::new_all();

    let mut get = interval(Duration::from_millis(500));
    let mut store = interval(Duration::from_secs(10));

    loop {
        tokio::select! {
            _ = get.tick() => {
                on_get(&mut sys)
            }
            _ = store.tick() => {
                on_store()
            }
        }
    }
}
