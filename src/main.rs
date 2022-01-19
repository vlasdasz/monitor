use tokio::time::{sleep, Duration};

use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};


#[tokio::main]
async fn main() {

    let mut sys = System::new_all();

    loop {

        // First we update all information of our `System` struct.
        sys.refresh_all();

        println!("total memory: {} KB", sys.total_memory());
        println!("used memory : {} KB", sys.used_memory());
        println!("total swap  : {} KB", sys.total_swap());
        println!("used swap   : {} KB", sys.used_swap());

        sleep(Duration::from_millis(5000)).await;
    }
}
