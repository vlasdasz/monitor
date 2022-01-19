use sysinfo::{DiskExt, System, SystemExt};

#[derive(Debug)]
pub struct Info {
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

        Self { /* ram, */ disk, }
    }
}
