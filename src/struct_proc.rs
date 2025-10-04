/*
----------------------------------------------------------------------------------------
File used to store the different structures representing the information about processes
depending on the command and its option(s).
----------------------------------------------------------------------------------------
*/

use serde::Serialize;

macro_rules! pub_struct {
    ($name:ident { $($field:ident: $t:ty,)* }) => {
        #[derive(Serialize)]
        pub struct $name {
            $(pub $field: $t,)*
        }
    }
}

pub_struct!(Process{
    pid: usize,
    name: String,
}); // Used for list

pub_struct!(ProcessInfo{
    pid: usize,
    name: String,
    state: String,
    ppid: usize,
    uid: u32,
    gid: u32,
    threads: usize,
    vm_size: usize,
    vm_rss: usize,
    cmdline: String,
}); // Used for the other commands

pub_struct!(FullProcessInfo{
        pid: usize,
        name: String,
        cmdline: String,
        state: String,
        ppid: usize,
        threads: usize,
        uid: u32,
        gid: u32,
        utime: u64,
        stime: u64,
        prio: i32,
        nice: i32,
        vm_size: usize,
        vm_rss: usize,
        vm_data: usize,
        vm_stack: usize,
        vm_exe: usize,
        vm_lib: usize,
        vm_swap: usize,
        vm_locked: usize,
        vm_hwm: usize,
        vm_peak: usize,
        read_bytes: Option<u64>,
        write_bytes: Option<u64>,
        read_count: Option<u64>,
        write_count: Option<u64>,
        cancelled_write_bytes: Option<u64>,
        fd_count: usize,
        open_files: Vec<String>,
        cwd: String,
        exe: String,
        root: String,
        mxcpu_time: String,
        mxfile_size: String,
        mxdata_size: String,
        mxstack_size: String,
        mxcore_file_size: String,
        mxresident_set: String,
        mxprocesses: String,
        mxopen_files: String,
        mxlocked_memory: String,
        mxaddress_space: String,
        mxfile_locks: String,
        mxpending_signals: String,
        mxmsgqueue_size: String,
        mxnice_prio: String,
        mxrealtime_prio: String,
        mxrealtime_timeout: String,
        tcp_connections: Vec<String>,
        udp_connections: Vec<String>,
        unix_sockets: Vec<String>,
        policy: String,
        rt_prio: u32,
        environment: Vec<String>,
        numa_maps: Vec<String>,
        cgroups: Vec<String>,
        syscall: Option<String>,
        wchan: Option<String>,
        sttime: u64,
        uptime: u64,
    }
); // Used for --all option
