use serde::Serialize;
use std::fs;
use std::io;

#[derive(Serialize)]
struct Process {
    pid: usize,
    name: String,
} // Used for list

#[derive(Serialize)]
pub struct ProcessInfo {
    pid: usize,
    pub name: String,
    pub state: String,
    ppid: usize,
    uid: u32,
    gid: u32,
    threads: usize,
    pub vm_size: usize,
    pub vm_rss: usize,
    cmdline: String,
} // Used for the other commands

/*
------------------------------------------------------------------------------------------------------------------------
Function read_info: -input:         a process id
                    -output:        a ProcessInfo containing information about this process if they are getable; an Error else
                    -description:   read information about this process (if it exist) through /proc/{PID}/comm
                                    and split it in the different "slots" of a new ProcessInfo
------------------------------------------------------------------------------------------------------------------------
*/
pub fn read_info(pid: usize) -> Result<ProcessInfo, io::Error> {
    let content = format!("/proc/{}/comm", pid);
    let name = fs::read_to_string(content)?.trim().to_string();
    let status = fs::read_to_string(format!("/proc/{}/status", pid))?;

    let mut state = String::from("None");
    let mut ppid = 0;
    let mut uid = 0;
    let mut gid = 0;
    let mut threads = 0;
    let mut vm_size = 0;
    let mut vm_rss = 0;
    for line in status.lines() {
        let mut now = line.split_whitespace();
        match now.next().unwrap_or("") {
            "State:" => {state = now.next().unwrap_or("None").to_string();}
            "PPid:" => {ppid = now.next().unwrap_or("0").parse().unwrap_or(0);}
            "Uid:" => {uid = now.next().unwrap_or("0").parse().unwrap_or(0);}
            "Gid:" => {gid = now.next().unwrap_or("0").parse().unwrap_or(0);}
            "Threads:" => {threads = now.next().unwrap_or("0").parse().unwrap_or(0);}
            "VmSize:" => {vm_size = now.next().unwrap_or("0").parse().unwrap_or(0);}
            "VmRSS:" => {vm_rss = now.next().unwrap_or("0").parse().unwrap_or(0);}
            _ => {}
        }
    }
    let cmdline_path = format!("/proc/{}/cmdline", pid);
    let cmdline = fs::read_to_string(cmdline_path).unwrap_or_default().replace('\0', " ").trim().to_string();

    Ok(ProcessInfo{pid, name, state, ppid, uid, gid, threads, vm_size, vm_rss, cmdline,})
}

/*
------------------------------------------------------------------------------------------------------------------------
Function get_pids_info: -input:         /
                        -output:        a vector of "Process" containing the pid and name of all process
                        -description:   get the pid of all processes (with "/proc") then for every process,
                                        find is name in "/proc/{PID}/comm" and push both information in a vector 
------------------------------------------------------------------------------------------------------------------------
*/
fn get_pids_info() -> Result<Vec<Process>, io::Error> {
    let mut pids: Vec<Process> = Vec::new();

    let contents = fs::read_dir("/proc")?;

    for instance in contents {
        let instance = instance?;
        let instance = instance.file_name().into_string().unwrap();

        if let Ok(pid) = instance.parse::<usize>() {
            let comm_path = format!("/proc/{}/comm", pid);
            if let Ok(name) = fs::read_to_string(comm_path) {
                pids.push(Process {
                    pid,
                    name: name.trim().to_string(),
                });
            }
        }
    }
    Ok(pids)
}

/*
------------------------------------------------------------------------------------------------------------------------
Function list_proc: -input:         options as booleans (json and file)
                    -output:        Result type (did it succed or not)
                    -description:   call get_pids_info and display the result dependig on the options
------------------------------------------------------------------------------------------------------------------------
*/
pub fn list_proc(json: bool, file: bool) -> Result<(), Box<dyn std::error::Error>> {
    let proc = match get_pids_info() {
        Ok(p) => p,
        Err(_) => {
            println!("No process running?!");
            let v: Vec<Process> = Vec::new();
            v
        }
    };

    if json {
        let output = serde_json::to_string_pretty(&proc)?;
        if !file {
            println!("{}", output);
        } else {
            println!("===== Creating the file =====");
            fs::write("./processes.json", output)?;
            println!("===== Creation completed =====");
        }
    } else {
        if !file {
            for p in &proc {
                println!("PID: {} - {}", p.pid, p.name);
            }
        } else {
            println!("===== Creating the file =====");
            let mut output = String::new();
            for p in &proc {
                output.push_str(&format!("PID: {} - {}\n", p.pid, p.name));
            }
            fs::write("./processes.txt", output)?;
            println!("===== Creation completed =====");
        }
    }
    Ok(())
}

/*
------------------------------------------------------------------------------------------------------------------------
Function pinfo: -input:         pid of the wanted process and options as booleans (json and file)
                -output:        Result type (did it succed or not)
                -description:   call get_pids_info to check if the wanted process exist, then call read_info on
                                its pid and display the result depending on the options
------------------------------------------------------------------------------------------------------------------------
*/
pub fn pinfo(pid: usize, json: bool, file: bool) -> Result<(), Box<dyn std::error::Error>> {
    let proc = match get_pids_info() {
        Ok(p) => p,
        Err(_) => {
            println!("No process running?!");
            let v: Vec<Process> = Vec::new();
            v
        }
    };
    let is_in = proc.iter().any(|p| p.pid == pid);
    if !is_in {
        println!("===== PID id not reconized =====");
    } else {
        let info = read_info(pid)?;
        if json {
            let output = serde_json::to_string_pretty(&info)?;
            if file {
                println!("===== Creating the file =====");
                fs::write(format!("./processes_{pid}_info.json"), output)?;
                println!("===== Creation completed =====");
            } else {
                println!("{}", output);
            }
        } else {
            if !file {
                println!("PID: {}", info.pid);
                println!("Name: {}", info.name);
                println!("State: {}", info.state);
                println!("Parent PID: {}", info.ppid);
                println!("UID: {}, GID: {}", info.uid, info.gid);
                println!("Threads: {}", info.threads);
                println!("Memory (VmRSS): {} kB", info.vm_rss);
                println!("Command: {}", info.cmdline);
            } else {
                println!("===== Creating the file =====");
                let output = "PID: {info.pid}\nName: {info.name}\nState: {info.state}\nParent PID: {info.ppid}\nUID: {info.uid}, GID: {info.gid}\nThreads: {info.threads}\nVmSize: {info.vm_size}\nMemory (VmRSS): {info.vm_rss} kB\nCommand: {info.cmdline}";
                fs::write(format!("./processes_{pid}_info.txt"), output)?;
                println!("===== Creation completed =====");
            }
        }
    }
    Ok(())
}