// Run it passing "/sys" and "/sys/dev/char/1:3" to it
//
// cargo run --example sysfs_uevent /sys "/sys/dev/char/1:3"
//

use kobject_uevent::UEvent;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 3 {
        eprintln!("{} <mount point> <sysfs path>", args.first().unwrap());
        panic!("Wrong arguments");
    }

    let mount_point = &args[1];
    let sysfs_path = &args[2];

    let uevent = UEvent::from_sysfs_path(sysfs_path, mount_point).unwrap();

    println!("{:#?}", uevent);
}
