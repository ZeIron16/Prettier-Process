use std::thread;
use std::time::Duration;
use std::fs;
use std::io;
use libc::{sysconf, _SC_CLK_TCK};
use chrono::Local;
use crate::proc::{read_info};

/*
------------------------------------------------------------------------------------------------------------------------
Function get_cpu_info:  -input:         a pid
                        -output:        the cpu usage for the given process
                        -description:   read cpu information in "/proc/{}/stat" and return it
------------------------------------------------------------------------------------------------------------------------
*/
fn get_cpu_info(pid: usize)-> Result<u64, io::Error>{
    let content = fs::read_to_string(format!("/proc/{}/stat", pid))?;
    let split: Vec<&str> = content.split_whitespace().collect();
    let user_t = split.get(13).unwrap_or(&"0").parse::<u64>().unwrap_or(0);
    let sys_t = split.get(14).unwrap_or(&"0").parse::<u64>().unwrap_or(0);
    let total = user_t + sys_t;

    Ok(total)
}

/*
------------------------------------------------------------------------------------------------------------------------
Function start: -input:         a pid and option as bolean (json)
                -output:        Result type (did it succed or not)
                -description:   start the live monitoring of the given process by reading its information (read_info)
                                every second and displaying the result depending on the option
------------------------------------------------------------------------------------------------------------------------
*/
pub fn start(pid: usize, json: bool)-> Result<(), Box<dyn std::error::Error>>{
    let mut prev_cpu = get_cpu_info(pid)?;
    let mut time = std::time::Instant::now();

    let _info = match read_info(pid){
        Ok(p) => p,
        Err(_) => {println!("===== PID not found ====="); return Ok(());}
    };
    println!("===== Live Monitor =====");
    loop{
        thread::sleep(Duration::from_secs(1));

        let info = match read_info(pid){
            Ok(p) => p,
            Err(_) => {println!("===== Process terminated ====="); return Ok(());}
        };
        let cpu = match get_cpu_info(pid){
            Ok(c) => c,
            Err(_) => {println!("===== Process terminated ====="); return Ok(());}
        };
        
        let delta_t = (std::time::Instant::now() - time).as_secs_f64();
        let tick = unsafe{sysconf(_SC_CLK_TCK)};
        let usage = ((cpu-prev_cpu) as f64 /tick as f64/delta_t)*100.0;

        if json{
            let output = serde_json::json!({
                "pid": pid,
                "name": info.name,
                "state": info.state,
                "cpu_percent": format!("{:.2}", usage),
                "memory_rss_kb": info.vm_rss,
                "memory_virtual_kb": info.vm_size,
                "timestamp": Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
            });
            println!("{}", serde_json::to_string_pretty(&output)?); 
        }
        else{
            println!("---- Process Status ----\n PID: {} | Name: {} | State: {}", pid, info.name, info.state);
            println!("--- Ressources Usage ---\n CPU Usage: {:.2}% | Memory RSS:  {} kB ({} MB) | Memory Virt: {} kB ({} MB)", usage, info.vm_rss, info.vm_rss / 1024, info.vm_size, info.vm_size / 1024);
            println!("--------- Time ---------\n {}", Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
            println!("\nPress Ctrl+C to stop\n");
        }

        prev_cpu = cpu;
        time = std::time::Instant::now();
    }
}