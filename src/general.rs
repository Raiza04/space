use colored::*;
use sysinfo::Disks;

pub fn run_general(total_width: f64) {
    let disks = Disks::new_with_refreshed_list();

    let mut system_total_bytes = 0;
    let mut system_free_bytes = 0;

    for disk in &disks {
        let name = disk.name().to_string_lossy();

        if !name.starts_with("/dev/") {
            continue;
        }

        let current_total_bytes = disk.total_space();
        if current_total_bytes == 0 {
            continue;
        }

        let curren_free_bytes = disk.available_space();

        system_total_bytes += current_total_bytes;
        system_free_bytes += curren_free_bytes;
    }

    let mut combined_bar = String::new();
    let mut legend_line: Vec<String> = Vec::new();
    let colors = ["green", "yellow", "blue", "magenta", "cyan", "red"];

    //Byte to Gb
    let gb_divisor = 1_073_741_824.0;

    let mut printed_blocks = 0;

    for (i, disk) in disks.iter().enumerate() {
        let name = disk.name().to_string_lossy();
        if !name.starts_with("/dev/") {
            continue;
        }

        let current_total = disk.total_space();
        if current_total == 0 {
            continue;
        }

        let color_name = colors[i % colors.len()];
        let current_free = disk.available_space();
        let used_space = current_total - current_free;

        let gb = used_space as f64 / gb_divisor;

        let bar_len = (used_space as f64 / system_total_bytes as f64) * total_width;
        let mut bar_len_rounded = bar_len.round() as usize;

        if bar_len_rounded == 0 && used_space > 0 {
            bar_len_rounded = 1;
        }

        combined_bar.push_str(&format!(
            "{}",
            "█".repeat(bar_len_rounded).color(color_name)
        ));

        legend_line.push(
            format!("{} - {:.2} G", disk.mount_point().to_string_lossy(), gb)
                .color(color_name)
                .to_string(),
        );

        printed_blocks += bar_len_rounded;
    }

    let bar_free_len = total_width as usize - printed_blocks;

    combined_bar.push_str(&"░".repeat(bar_free_len).dimmed().to_string());

    let system_free_gb = system_free_bytes as f64 / gb_divisor;

    legend_line.push(format!("Free - {:.2} G", system_free_gb));

    println!("[{combined_bar}]");

    for word in legend_line {
        println!("{word}");
    }
}
