use colored::*;
use sysinfo::Disks;

pub fn run_isolated(bar_width: f64) {
    let disks = Disks::new_with_refreshed_list();

    let colors = ["green", "yellow", "blue", "magenta", "cyan", "red"];
    let gb_divisor = 1_073_741_824.0;

    for (i, disks) in disks.iter().enumerate() {
        let name = disks.name().to_string_lossy();
        if !name.starts_with("/dev/") {
            continue;
        }

        let total_bytes = disks.total_space();
        if total_bytes == 0 {
            continue;
        }

        let available_bytes = disks.available_space();
        let accuired_bytes = total_bytes - available_bytes;

        let barlen_used = (accuired_bytes as f64 / total_bytes as f64) * bar_width;
        let mut barlen_rounded_used = barlen_used.round() as usize;
        if barlen_rounded_used < 1 && accuired_bytes > 0 {
            barlen_rounded_used = 1;
        }

        let barlen_free = bar_width as usize - barlen_rounded_used;

        let mount_name = disks.mount_point().to_string_lossy();
        let line_color = colors[i % colors.len()];

        let used_gb = accuired_bytes as f64 / gb_divisor;
        let total_gb = total_bytes as f64 / gb_divisor;

        println!(
            "{} => {:.2}G/{:.2}G",
            mount_name.color(line_color),
            used_gb,
            total_gb
        );
        println!(
            "[{}{}]",
            "█".repeat(barlen_rounded_used).color(line_color),
            "░".repeat(barlen_free).dimmed()
        )
    }
}
