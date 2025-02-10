use winapi::um::{
    processthreadsapi::OpenProcess,
    sysinfoapi::{GetSystemInfo, SYSTEM_INFO},
    winbase::SetProcessAffinityMask,
    winnt::PROCESS_SET_INFORMATION,
};

fn get_cpu_core_count() -> u32 {
    unsafe {
        let mut sys_info: SYSTEM_INFO = std::mem::zeroed();
        GetSystemInfo(&mut sys_info);
        sys_info.dwNumberOfProcessors
    }
}

fn set_process_affinity(process_id: u32, affinity_mask: u32) -> bool {
    unsafe {
        // get process handle
        let process_handle = OpenProcess(PROCESS_SET_INFORMATION, 0, process_id);
        if process_handle.is_null() {
            println!("Failed to open process {process_id}.");
            return false;
        }

        // set process affinity
        let result = SetProcessAffinityMask(process_handle, affinity_mask);
        return result == 0;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let process_id = args[0].parse::<u32>()?;
    let affinity_mask = args[1].parse::<u32>()?;
    if affinity_mask >= 1 << get_cpu_core_count() {
        println!("Invalid affinity mask.");
        return Ok(());
    }

    if set_process_affinity(process_id, affinity_mask) {
        println!("Process affinity set successfully.");
    } else {
        println!("Failed to set process affinity.");
    }
    Ok(())
}
