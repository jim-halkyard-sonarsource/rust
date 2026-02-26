use clap::Parser;
use std::thread;
use std::time::Duration;

// Conditional imports: Only compile 'fs' and 'Command' where they are used
#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "macos")]
use std::process::Command;

use sysinfo::System;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Number of snapshots to run
    #[arg(short, long)]
    runs: Option<u32>,

    /// Polling interval in seconds, default is 5
    #[arg(short, long, default_value_t = 5)]
    interval: u64,
}

fn main() {
    let args = Args::parse();
    let mut count = 0;
    let mut sys = System::new_all();

    loop {
        println!("--- Snapshot {} ---", count + 1);

        // Refresh system data
        sys.refresh_cpu();
        // A short sleep is required for the first iteration to calculate CPU delta
        thread::sleep(Duration::from_millis(200)); 
        sys.refresh_cpu();
        sys.refresh_memory();

        // 1. CPU Usage
        println!("CPU Usage:      {:.2}%", sys.global_cpu_info().cpu_usage());

        // 2. Human-Readable RAM
        let used = format_bytes(sys.used_memory());
        let total = format_bytes(sys.total_memory());
        println!("Memory:         {} / {} used", used, total);

        // 3. Unified Power Reporting
        report_power();

        count += 1;
        if let Some(max) = args.runs {
            if count >= max { break; }
        }

        thread::sleep(Duration::from_secs(args.interval));
        println!();
    }
}

/// Converts bytes into human-readable units (B, KB, MB, GB)
fn format_bytes(bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut count = 0;
    let mut f_bytes = bytes as f64;

    while f_bytes >= 1024.0 && count < units.len() - 1 {
        f_bytes /= 1024.0;
        count += 1;
    }
    format!("{:.2} {}", f_bytes, units[count])
}

/// Platform-agnostic power reporting logic
fn report_power() {
    let mut source = "Unknown".to_string();
    let mut percentage = "N/A".to_string();

    #[cfg(target_os = "linux")]
    {
        // Linux: Read from sysfs
        source = fs::read_to_string("/sys/class/power_supply/AC/online")
            .map(|s| if s.trim() == "1" { "AC".to_string() } else { "Battery".to_string() })
            .unwrap_or_else(|_| "Unknown".to_string());
        
        if let Ok(cap) = fs::read_to_string("/sys/class/power_supply/BAT0/capacity") {
            percentage = format!("{}%", cap.trim());
        }
    }

    #[cfg(target_os = "macos")]
    {
        // macOS: We'll use the 'pmset' command which is the most reliable way 
        // to get power info on a Mac without heavy FFI bindings.
        let output = Command::new("pmset")
            .arg("-g")
            .arg("batt")
            .output();

        if let Ok(out) = output {
            let s = String::from_utf8_lossy(&out.stdout);
            // Parse for 'AC Power' or 'Battery Power'
            source = if s.contains("AC Power") { "AC".to_string() } else { "Battery".to_string() };
            
            // Parse for percentage (e.g., "95%")
            if let Some(idx) = s.find('%') {
                let start = s[..idx].rfind(|c: char| c.is_whitespace()).unwrap_or(0);
                percentage = s[start..idx+1].trim().to_string();
            }
        }
    }

    println!("Power Source:   {}", source);
    println!("Charge:         {}", percentage);
}

#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_cpu_usage_calculation() {
        // T1: Simulate a CPU that was 50% busy
        let start = CpuSnapshot { 
            user: 100, nice: 0, system: 100, idle: 200, 
            iowait: 0, irq: 0, softirq: 0 
        }; // Total = 400
        
        let end = CpuSnapshot { 
            user: 200, nice: 0, system: 200, idle: 400, 
            iowait: 0, irq: 0, softirq: 0 
        }; // Total = 800 (Delta Total = 400, Delta Idle = 200)

        let usage = calculate_usage(&start, &end);
        assert_eq!(usage, 50.0, "CPU usage should be exactly 50%");
    }

    #[test]
    fn test_zero_delta_handling() {
        // T2: Ensure we don't divide by zero if called too quickly
        let start = CpuSnapshot { user: 10, nice: 0, system: 0, idle: 10, iowait: 0, irq: 0, softirq: 0 };
        let usage = calculate_usage(&start, &start);
        assert_eq!(usage, 0.0, "Zero delta should return 0% usage, not a crash");
    }
    
    #[test]
    fn test_format_bytes() {
        // T3: Make sure we are formatting data correctly
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
        assert_eq!(format_bytes(1073741824), "1.00 GB");
    }
}
