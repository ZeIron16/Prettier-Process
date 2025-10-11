use crate::struct_proc as sp;
use crate::proc::{read_info};
use std::fs;

/*
------------------------------------------------------------------------------------------------------------------------
Function statistics:    -input:         options as boleans
                        -output:        Result type (did it succed or not)
                        -description:   get all processes (ProcessInfo) public informations (with read_info) and
                                        push them in a vector; then compute generals information and display the
                                        result depending on the options
------------------------------------------------------------------------------------------------------------------------
*/
pub fn statistics(json: bool, file: bool)-> Result<(), Box<dyn std::error::Error>>{
    let mut pids: Vec<sp::ProcessInfo> = Vec::new();

    let contents = fs::read_dir("/proc")?;

    for instance in contents {
        let instance = instance?;
        let instance = instance.file_name().into_string().unwrap();

        if let Ok(pid) = instance.parse::<usize>() {
            if let Ok(info) = read_info(pid) {
                pids.push(info);
            }
        }
    }

    let mut total = 0;
    let mut run = 0;
    let mut sleep = 0;
    let mut disk_sleep = 0;
    let mut zombie = 0;
    let mut stop = 0;
    let mut idle = 0;
    let mut res_mem = 0;
    let mut vr_mem = 0;

    for p in pids{
        total += 1;
        match p.state.as_str(){
            "R" => run += 1,
            "S" => sleep += 1,
            "D" => disk_sleep += 1,
            "Z" => zombie += 1,
            "T" => stop += 1,
            "I" => idle += 1,
            _ => {}
        }
        res_mem += p.vm_rss;
        vr_mem += p.vm_size;
    }

    if json{
        let stats = serde_json::json!({
            "total_processes": total,
            "running": run,
            "sleeping": sleep,
            "disk_sleeping": disk_sleep,
            "zombie": zombie,
            "stopped": stop,
            "idle": idle,
            "rss_memory_kb": res_mem,
            "virtual_memory_kb": vr_mem
        });
        let output = serde_json::to_string_pretty(&stats)?;
        if !file {
            println!("{}", output);
        } else {
            println!("===== Creating the file =====");
            fs::write("./statistics.json", output)?;
            println!("===== Creation completed =====");
        }
    }
    else{
        let output = format!("===== Statistics =====\n--- Processes Status ---\nTotal processes: {total}\nRunning: {run}\nSleeping: {sleep}\nDisk Sleep: {disk_sleep}\nZombie: {zombie}\nStopped: {stop}\nIdle: {idle}\n--- Memory Usage ---\nTotal RSS Memory: {res_mem} kB ({} MB)\nTotal Virtual Memory: {vr_mem} kB ({} MB)", res_mem / 1024, vr_mem / 1024);
        if !file {
            println!("{}", output);
        } else {
            println!("===== Creating the file =====");
            fs::write("./statistics.txt", output)?;
            println!("===== Creation completed =====");
        }
    }
    Ok(())
}