use crate::struct_proc as sp;
use std::fs;
use std::io;

/*
------------------------------------------------------------------------------------------------------------------------
Function read_info: -input:         a process id
                    -output:        a ProcessInfo containing information about this process if they are getable; an Error 
                                    else
                    -description:   read information about this process (if it exist) through /proc/{PID}/comm
                                    and split it in the different "slots" of a new ProcessInfo
------------------------------------------------------------------------------------------------------------------------
*/
pub fn read_info(pid: usize) -> Result<sp::ProcessInfo, io::Error> {
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

    Ok(sp::ProcessInfo{pid, name, state, ppid, uid, gid, threads, vm_size, vm_rss, cmdline,})
}

/*
------------------------------------------------------------------------------------------------------------------------
Function read_all_info: -input:         a process id
                        -output:        a FullProcessInfo containing information about this process if they are getable; 
                                        an Error else
                        -description:   read all the information about this process (if it exist) through /proc/{PID}/...
                                        and split it in the different "slots" of a new ProcessInfo
------------------------------------------------------------------------------------------------------------------------
*/
pub fn read_all_info(pid: usize) -> Result<sp::FullProcessInfo, io::Error> {
    let name = fs::read_to_string(format!("/proc/{}/comm", pid))?.trim().to_string();
    let status = fs::read_to_string(format!("/proc/{}/status", pid))?;
    let mut state = String::from("None");
    let mut ppid = 0;
    let mut uid = 0;
    let mut gid = 0;
    let mut threads = 0;
    let mut vm_size = 0;
    let mut vm_rss = 0;
    let mut vm_data = 0;
    let mut vm_stack = 0;
    let mut vm_exe = 0;
    let mut vm_lib = 0;
    let mut vm_swap = 0;
    let mut vm_locked = 0;
    let mut vm_hwm = 0;
    let mut vm_peak = 0;
    
    for line in status.lines() {
        let mut parts = line.split_whitespace();
        match parts.next().unwrap_or("") {
            "State:" => { state = parts.next().unwrap_or("None").to_string(); }
            "PPid:" => { ppid = parts.next().unwrap_or("0").parse().unwrap_or(0); }
            "Uid:" => { uid = parts.next().unwrap_or("0").parse().unwrap_or(0); }
            "Gid:" => { gid = parts.next().unwrap_or("0").parse().unwrap_or(0); }
            "Threads:" => { threads = parts.next().unwrap_or("0").parse().unwrap_or(0); }
            "VmSize:" => { vm_size = parts.next().unwrap_or("0").parse().unwrap_or(0); }
            "VmRSS:" => { vm_rss = parts.next().unwrap_or("0").parse().unwrap_or(0); }
            "VmData:" => { vm_data = parts.next().unwrap_or("0").parse().unwrap_or(0); }
            "VmStk:" => { vm_stack = parts.next().unwrap_or("0").parse().unwrap_or(0); }
            "VmExe:" => { vm_exe = parts.next().unwrap_or("0").parse().unwrap_or(0); }
            "VmLib:" => { vm_lib = parts.next().unwrap_or("0").parse().unwrap_or(0); }
            "VmSwap:" => { vm_swap = parts.next().unwrap_or("0").parse().unwrap_or(0); }
            "VmLck:" => { vm_locked = parts.next().unwrap_or("0").parse().unwrap_or(0); }
            "VmHWM:" => { vm_hwm = parts.next().unwrap_or("0").parse().unwrap_or(0); }
            "VmPeak:" => { vm_peak = parts.next().unwrap_or("0").parse().unwrap_or(0); }
            _ => {}
        }
    }
    
    let stat = fs::read_to_string(format!("/proc/{}/stat", pid))?;
    let stat_parts: Vec<&str> = stat.split_whitespace().collect();
    let utime = stat_parts.get(13).and_then(|s| s.parse().ok()).unwrap_or(0);
    let sttime = stat_parts.get(14).and_then(|s| s.parse().ok()).unwrap_or(0);
    let prio = stat_parts.get(17).and_then(|s| s.parse().ok()).unwrap_or(0);
    let nice = stat_parts.get(18).and_then(|s| s.parse().ok()).unwrap_or(0);
    let stime = stat_parts.get(21).and_then(|s| s.parse().ok()).unwrap_or(0);
    
    let uptime = fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|s| s.split_whitespace().next().and_then(|t| t.parse::<f64>().ok()))
        .unwrap_or(0.0) as u64;
    
    let cmdline = fs::read_to_string(format!("/proc/{}/cmdline", pid))
        .unwrap_or_default()
        .replace('\0', " ")
        .trim()
        .to_string();
    
    let io_result = fs::read_to_string(format!("/proc/{}/io", pid));
    let mut read_bytes = None;
    let mut write_bytes = None;
    let mut read_count = None;
    let mut write_count = None;
    let mut cancelled_write_bytes = None;
    
    if let Ok(io_content) = io_result {
        for line in io_content.lines() {
            let mut parts = line.split_whitespace();
            match parts.next().unwrap_or("") {
                "read_bytes:" => { read_bytes = parts.next().and_then(|s| s.parse().ok()); }
                "write_bytes:" => { write_bytes = parts.next().and_then(|s| s.parse().ok()); }
                "syscr:" => { read_count = parts.next().and_then(|s| s.parse().ok()); }
                "syscw:" => { write_count = parts.next().and_then(|s| s.parse().ok()); }
                "cancelled_write_bytes:" => { cancelled_write_bytes = parts.next().and_then(|s| s.parse().ok()); }
                _ => {}
            }
        }
    }
    
    let fd_count = fs::read_dir(format!("/proc/{}/fd", pid))
        .map(|entries| entries.count())
        .unwrap_or(0);
    
    let open_files = fs::read_dir(format!("/proc/{}/fd", pid))
        .map(|entries| {
            entries.filter_map(|e| e.ok())
                .filter_map(|e| fs::read_link(e.path()).ok())
                .filter_map(|p| p.to_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_else(|_| Vec::new());
    
    let cwd = fs::read_link(format!("/proc/{}/cwd", pid))
        .ok()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .unwrap_or_else(|| String::from("N/A"));
    
    let exe = fs::read_link(format!("/proc/{}/exe", pid))
        .ok()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .unwrap_or_else(|| String::from("N/A"));
    
    let root = fs::read_link(format!("/proc/{}/root", pid))
        .ok()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .unwrap_or_else(|| String::from("N/A"));
    
    let limits_content = fs::read_to_string(format!("/proc/{}/limits", pid)).unwrap_or_default();
    let mut limits_map = std::collections::HashMap::new();
    for line in limits_content.lines().skip(1) { // Skip header
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let name = parts[0..parts.len()-2].join(" ");
            let soft = parts[parts.len()-2];
            limits_map.insert(name, soft.to_string());
        }
    }
    
    let mxcpu_time = limits_map.get("Max cpu time").cloned().unwrap_or_else(|| String::from("unlimited"));
    let mxfile_size = limits_map.get("Max file size").cloned().unwrap_or_else(|| String::from("unlimited"));
    let mxdata_size = limits_map.get("Max data size").cloned().unwrap_or_else(|| String::from("unlimited"));
    let mxstack_size = limits_map.get("Max stack size").cloned().unwrap_or_else(|| String::from("unlimited"));
    let mxcore_file_size = limits_map.get("Max core file size").cloned().unwrap_or_else(|| String::from("unlimited"));
    let mxresident_set = limits_map.get("Max resident set").cloned().unwrap_or_else(|| String::from("unlimited"));
    let mxprocesses = limits_map.get("Max processes").cloned().unwrap_or_else(|| String::from("unlimited"));
    let mxopen_files = limits_map.get("Max open files").cloned().unwrap_or_else(|| String::from("unlimited"));
    let mxlocked_memory = limits_map.get("Max locked memory").cloned().unwrap_or_else(|| String::from("unlimited"));
    let mxaddress_space = limits_map.get("Max address space").cloned().unwrap_or_else(|| String::from("unlimited"));
    let mxfile_locks = limits_map.get("Max file locks").cloned().unwrap_or_else(|| String::from("unlimited"));
    let mxpending_signals = limits_map.get("Max pending signals").cloned().unwrap_or_else(|| String::from("unlimited"));
    let mxmsgqueue_size = limits_map.get("Max msgqueue size").cloned().unwrap_or_else(|| String::from("unlimited"));
    let mxnice_prio = limits_map.get("Max nice prio").cloned().unwrap_or_else(|| String::from("0"));
    let mxrealtime_prio = limits_map.get("Max realtime prio").cloned().unwrap_or_else(|| String::from("0"));
    let mxrealtime_timeout = limits_map.get("Max realtime timeout").cloned().unwrap_or_else(|| String::from("unlimited"));
    
    let tcp_connections = Vec::new(); // TODO: parser /proc/net/tcp
    let udp_connections = Vec::new(); // TODO: parser /proc/net/udp
    let unix_sockets = Vec::new(); // TODO: parser /proc/net/unix
    
    let policy = String::from("SCHED_OTHER"); // Par défaut, à parser depuis /proc/[pid]/sched
    let rt_prio = 0; // À parser depuis /proc/[pid]/stat
    
    let environment = fs::read_to_string(format!("/proc/{}/environ", pid))
        .map(|content| {
            content.split('\0')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_else(|_| Vec::new());
    
    let numa_maps = fs::read_to_string(format!("/proc/{}/numa_maps", pid))
        .map(|content| content.lines().map(|s| s.to_string()).collect())
        .unwrap_or_else(|_| Vec::new());
    
    let cgroups = fs::read_to_string(format!("/proc/{}/cgroup", pid))
        .map(|content| content.lines().map(|s| s.to_string()).collect())
        .unwrap_or_else(|_| Vec::new());
    
    let syscall = fs::read_to_string(format!("/proc/{}/syscall", pid))
        .ok()
        .map(|s| s.trim().to_string());
    

    let wchan = fs::read_to_string(format!("/proc/{}/wchan", pid))
        .ok()
        .map(|s| s.trim().to_string());
    
    Ok(sp::FullProcessInfo { pid, name, cmdline, state, ppid, threads, uid, gid, utime, stime, prio, nice, vm_size, vm_rss, vm_data, vm_stack, vm_exe, vm_lib, vm_swap, vm_locked,
        vm_hwm, vm_peak, read_bytes, write_bytes, read_count, write_count, cancelled_write_bytes, fd_count, open_files, cwd, exe, root, mxcpu_time, mxfile_size, mxdata_size, mxstack_size, mxcore_file_size,
        mxresident_set, mxprocesses, mxopen_files, mxlocked_memory, mxaddress_space, mxfile_locks, mxpending_signals, mxmsgqueue_size, mxnice_prio, mxrealtime_prio, mxrealtime_timeout,
        tcp_connections, udp_connections, unix_sockets, policy, rt_prio, environment, numa_maps, cgroups, syscall, wchan, sttime, uptime})
}

/*
------------------------------------------------------------------------------------------------------------------------
Function get_pids_info: -input:         /
                        -output:        a vector of "Process" containing the pid and name of all process
                        -description:   get the pid of all processes (with "/proc") then for every process,
                                        find is name in "/proc/{PID}/comm" and push both information in a vector 
------------------------------------------------------------------------------------------------------------------------
*/
fn get_pids_info() -> Result<Vec<sp::Process>, io::Error> {
    let mut pids: Vec<sp::Process> = Vec::new();

    let contents = fs::read_dir("/proc")?;

    for instance in contents {
        let instance = instance?;
        let instance = instance.file_name().into_string().unwrap();

        if let Ok(pid) = instance.parse::<usize>() {
            let comm_path = format!("/proc/{}/comm", pid);
            if let Ok(name) = fs::read_to_string(comm_path) {
                pids.push(sp::Process {
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
            let v: Vec<sp::Process> = Vec::new();
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
pub fn pinfo(pid: usize, json: bool, file: bool, all: bool) -> Result<(), Box<dyn std::error::Error>> {
    let proc = match get_pids_info() {
        Ok(p) => p,
        Err(_) => {
            println!("No process running?!");
            let v: Vec<sp::Process> = Vec::new();
            v
        }
    };
    let is_in = proc.iter().any(|p| p.pid == pid);
    if !is_in {
        println!("===== PID id not reconized =====");
    } else {
        if all{
            let info = read_all_info(pid)?;
            display_all(info, json, file)?;
        }
        else{
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
    }
    Ok(())
}

/*
------------------------------------------------------------------------------------------------------------------------
Function display_all:   -input:         FullProcessInfo of the wanted process and options as booleans (json and file)
                        -output:        Result type (did it succed or not)
                        -description:   display all the information available for the wanted process in different
                                        categories in txt mode according to the different options (--file and --json)
------------------------------------------------------------------------------------------------------------------------
*/
fn display_all(info: sp::FullProcessInfo, json: bool, file: bool) -> Result<(), io::Error> {
    if json {
        let output = serde_json::to_string_pretty(&info)?;
        if file{
            println!("===== Creating the file =====");
            fs::write(format!("./processes_{}_all_info.json", info.pid), output)?;
            println!("===== Creation completed =====");
        }else{
            println!("{}", output);
        }
    }else{
        let mut output = String::new();
        output.push_str("\n--- Basic Information ---\n");
        output.push_str(&format!("PID: {}\n", info.pid));
        output.push_str(&format!("Name: {}\n", info.name));
        output.push_str(&format!("Command: {}\n", info.cmdline));
        output.push_str("\n--- Status ---\n");
        output.push_str(&format!("State: {}\n", info.state));
        output.push_str(&format!("Parent PID: {}\n", info.ppid));
        output.push_str(&format!("UID: {}, GID: {}\n", info.uid, info.gid));
        output.push_str(&format!("Threads: {}\n", info.threads));
        output.push_str(&format!("Priority: {}\n", info.prio));
        output.push_str(&format!("Nice: {}\n", info.nice));
        output.push_str("\n--- CPU Times ---\n");
        output.push_str(&format!("User time: {} ticks\n", info.utime));
        output.push_str(&format!("System time: {} ticks\n", info.stime));
        output.push_str(&format!("Start time: {} ticks\n", info.sttime));
        output.push_str(&format!("Uptime: {} seconds\n", info.uptime));
        output.push_str("\n--- Memory ---\n");
        output.push_str(&format!("VmSize: {} kB\n", info.vm_size));
        output.push_str(&format!("VmRSS: {} kB\n", info.vm_rss));
        output.push_str(&format!("VmData: {} kB\n", info.vm_data));
        output.push_str(&format!("VmStack: {} kB\n", info.vm_stack));
        output.push_str(&format!("VmExe: {} kB\n", info.vm_exe));
        output.push_str(&format!("VmLib: {} kB\n", info.vm_lib));
        output.push_str(&format!("VmSwap: {} kB\n", info.vm_swap));
        output.push_str(&format!("VmLocked: {} kB\n", info.vm_locked));
        output.push_str(&format!("VmHWM (Peak RSS): {} kB\n", info.vm_hwm));
        output.push_str(&format!("VmPeak: {} kB\n", info.vm_peak));
        output.push_str("\n--- I/O Statistics ---\n");
        if let Some(rb) = info.read_bytes{
            output.push_str(&format!("Read bytes: {}\n", rb));
        }else{
            output.push_str("Read bytes: N/A (no permission)\n");
        }
        if let Some(wb) = info.write_bytes{
            output.push_str(&format!("Write bytes: {}\n", wb));
        }else{
            output.push_str("Write bytes: N/A (no permission)\n");
        }
        if let Some(rc) = info.read_count{output.push_str(&format!("Read syscalls: {}\n", rc));}
        if let Some(wc) = info.write_count{output.push_str(&format!("Write syscalls: {}\n", wc));}
        if let Some(cwb) = info.cancelled_write_bytes{output.push_str(&format!("Cancelled write bytes: {}\n", cwb));}
        output.push_str("\n--- Files ---\n");
        output.push_str(&format!("Open file descriptors: {}\n", info.fd_count));
        output.push_str(&format!("Current working directory: {}\n", info.cwd));
        output.push_str(&format!("Executable: {}\n", info.exe));
        output.push_str(&format!("Root directory: {}\n", info.root));
        if !info.open_files.is_empty(){
            output.push_str("Open files (first 10):\n");
            for (i, file) in info.open_files.iter().take(10).enumerate(){
                output.push_str(&format!("  [{}] {}\n", i, file));
            }
            if info.open_files.len() > 10{
                output.push_str(&format!("  ... and {} more\n", info.open_files.len() - 10));
            }
        }
        output.push_str("\n--- Resource Limits ---\n");
        output.push_str(&format!("Max CPU time: {}\n", info.mxcpu_time));
        output.push_str(&format!("Max file size: {}\n", info.mxfile_size));
        output.push_str(&format!("Max data size: {}\n", info.mxdata_size));
        output.push_str(&format!("Max stack size: {}\n", info.mxstack_size));
        output.push_str(&format!("Max core file size: {}\n", info.mxcore_file_size));
        output.push_str(&format!("Max resident set: {}\n", info.mxresident_set));
        output.push_str(&format!("Max processes: {}\n", info.mxprocesses));
        output.push_str(&format!("Max open files: {}\n", info.mxopen_files));
        output.push_str(&format!("Max locked memory: {}\n", info.mxlocked_memory));
        output.push_str(&format!("Max address space: {}\n", info.mxaddress_space));
        output.push_str(&format!("Max file locks: {}\n", info.mxfile_locks));
        output.push_str(&format!("Max pending signals: {}\n", info.mxpending_signals));
        output.push_str(&format!("Max msgqueue size: {}\n", info.mxmsgqueue_size));
        output.push_str(&format!("Max nice priority: {}\n", info.mxnice_prio));
        output.push_str(&format!("Max realtime priority: {}\n", info.mxrealtime_prio));
        output.push_str(&format!("Max realtime timeout: {}\n", info.mxrealtime_timeout));
        output.push_str("\n--- Scheduling ---\n");
        output.push_str(&format!("Policy: {}\n", info.policy));
        output.push_str(&format!("RT priority: {}\n", info.rt_prio));
        output.push_str("\n--- Network ---\n");
        if !info.tcp_connections.is_empty(){
            output.push_str(&format!("TCP connections: {}\n", info.tcp_connections.len()));
        }else{
            output.push_str("TCP connections: none\n");
        }
        if !info.udp_connections.is_empty(){
            output.push_str(&format!("UDP connections: {}\n", info.udp_connections.len()));
        }else{
            output.push_str("UDP connections: none\n");
        }
        if !info.unix_sockets.is_empty(){
            output.push_str(&format!("Unix sockets: {}\n", info.unix_sockets.len()));
        }else{
            output.push_str("Unix sockets: none\n");
        }
        output.push_str("\n--- Environment Variables ---\n");
        if !info.environment.is_empty(){
            output.push_str(&format!("Count: {}\n", info.environment.len()));
            output.push_str("First 5:\n");
            for (i, env) in info.environment.iter().take(5).enumerate(){
                output.push_str(&format!("  [{}] {}\n", i, env));
            }
            if info.environment.len() > 5{
                output.push_str(&format!("  ... and {} more\n", info.environment.len() - 5));
            }
        }else{
            output.push_str("No environment variables available\n");
        }
        output.push_str("\n--- Control Groups ---\n");
        if !info.cgroups.is_empty(){
            for cgroup in &info.cgroups{
                output.push_str(&format!("  {}\n", cgroup));
            }
        }else{
            output.push_str("No cgroup information available\n");
        }
        output.push_str("\n--- NUMA Maps ---\n");
        if !info.numa_maps.is_empty(){
            output.push_str(&format!("Count: {} entries\n", info.numa_maps.len()));
        }else{
            output.push_str("No NUMA maps available\n");
        }
        output.push_str("\n--- Misc ---\n");
        if let Some(syscall) = &info.syscall{
            output.push_str(&format!("Current syscall: {}\n", syscall));
        }else{
            output.push_str("Current syscall: N/A\n");
        }
        if let Some(wchan) = &info.wchan{
            output.push_str(&format!("Wait channel: {}\n", wchan));
        }else{
            output.push_str("Wait channel: N/A\n");
        }
        if file{
            println!("===== Creating the file =====");
            fs::write(format!("./processes_{}_all_info.txt", info.pid), output)?;
            println!("===== Creation completed =====");
        }else{
            print!("{}", output);
        }
    }
    Ok(())
}