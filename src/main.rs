use sysinfo::{
    System,
};

fn main() {
    let mut sys = System::new_all();

    sys.refresh_all();

    println!("=> system:");

    let free = sys.total_memory() - sys.free_memory();
    let free_formatted = free as f64 / 1073741824.0;
    println!("Available memory : {:.2} GB", free_formatted);

    // Display system information:
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());

    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());
}
