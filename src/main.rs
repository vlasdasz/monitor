use tokio::time::{sleep, Duration};

use sysinfo::{DiskExt, NetworkExt, NetworksExt, ProcessExt, System, SystemExt};


#[tokio::main]
async fn main() {

    let mut sys = System::new_all();

    loop {

        sys.refresh_all();

        let disk = match sys.disks().first() {
            Some(val) => val.available_space() / 1024 / 1024 / 1024,
            None => 0,
        } as u16;

        let mem = (sys.used_memory() / 1024) as u16;

        dbg!(disk);
        dbg!(mem);

        sleep(Duration::from_millis(5000)).await;
    }
}
