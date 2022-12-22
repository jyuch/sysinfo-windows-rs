use sysinfo::{NetworkExt, ProcessExt, System, SystemExt};

fn main() {
    let sys = System::new_all();

    println!("=> disks:");
    for disk in sys.disks() {
        println!("{:?}", disk);
    }

    println!("=> users:");
    for user in sys.users() {
        println!("{:?}", user);
    }
}
