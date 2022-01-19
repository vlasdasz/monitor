use tokio::time::{sleep, Duration};
extern crate sys_info;

use sys_info::*;


#[tokio::main]
async fn main() {

    loop {

        println!("os: {} {}", os_type().unwrap(), os_release().unwrap());

        println!("cpu: {} cores, {} MHz", cpu_num().unwrap(), cpu_speed().unwrap());

        println!("proc total: {}", proc_total().unwrap());

        let load = loadavg().unwrap();

        println!("load: {} {} {}", load.one, load.five, load.fifteen);

        let mem = mem_info().unwrap();

        println!("mem: total {} MB, free {} MB, avail {} MB, buffers {} MB, cached {} MB",
                 mem.total / 1024, mem.free / 1024, mem.avail / 1024, mem.buffers / 1024, mem.cached / 1024 );

        #[cfg(not(target_os = "solaris"))] {
            let disk = disk_info().unwrap();
            println!("disk: total {} KB, free {} KB", disk.total, disk.free);
        }

        sleep(Duration::from_millis(5000)).await;
    }
}
