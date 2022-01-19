use tokio::time::{sleep, Duration};

use sysinfo::{DiskExt, NetworkExt, NetworksExt, ProcessExt, System, SystemExt};


#[tokio::main]
async fn main() {

    let mut sys = System::new_all();

    loop {

        // First we update all information of our `System` struct.
        sys.refresh_all();

        // We display all disks' information:
        println!("=> disks:");
        for disk in sys.disks() {
            println!("{:?}", disk);
            println!("{:?} GB", disk.total_space()  / 1024 / 1024 / 1024);
            println!("{:?} GB", disk.available_space()  / 1024 / 1024 / 1024);
        }

        println!("total memory: {} MB", sys.total_memory() / 1024);
        println!("used memory : {} MB", sys.used_memory() / 1024);
        println!("total swap  : {} MB", sys.total_swap() / 1024);
        println!("used swap   : {} MB", sys.used_swap() / 1024);

        sleep(Duration::from_millis(5000)).await;
    }
}
