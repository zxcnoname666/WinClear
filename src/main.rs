use std::{fs, mem, path::PathBuf, process};
use windows::{core::{s, PCSTR}, Win32::System::Registry::{RegCloseKey, RegCreateKeyA, RegDeleteKeyA, RegSetValueExA, HKEY, HKEY_CURRENT_USER, REG_SZ}};

mod core;

fn main() {
    for arg in std::env::args() {
        if arg.starts_with("/run") {
            core::run();
            process::exit(1);
        }
    }

    loop {
        println!("Write command... // Write \"help\" to get commands");

        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => {
                line = line.replace('\n', "").trim().to_lowercase();
            }
            Err(err) => {
                println!("Couldn't read the line: {err}");
            }
        };
        
        process_command(&line);
    }
}

fn process_command(command: &str) {
    match command {

        "help" => {
            println!("Command list:");
            println!("| help - Get a list of commands");
            println!("| install - Install a start-up for automatic clearing logs");
            println!("| uninstall - Remove a start-up for automatic clearing logs");
            println!("| run - Run the backup script");
            println!("| quit - Close the app");
        }

        "install" => {
            let appdata = match std::env::var("LOCALAPPDATA") {
                Ok(res) => res,
                Err(err) => {
                    println!("Err: {}", err);
                    return;
                }
            };
            
            let mut service_file_path = PathBuf::from(appdata);
            service_file_path.push("WinClear");

            if service_file_path.exists() {
                if let Err(err) = fs::remove_dir_all(&service_file_path) {
                    println!("Error when deleting a exists directory: {err}");
                }
            }

            if let Err(err) = fs::create_dir_all(&service_file_path) {
                println!("Error when creating a directory: {err}");
                return;
            }

            let current_path = match std::env::current_exe() {
                Ok(path) => path,
                Err(err) => {
                    println!("Error when getting the location of the current file: {err}");
                    return;
                }
            };
            
            let exec_file_path = service_file_path.join("WinClear.exe");
            if let Err(err) = fs::copy(current_path, &exec_file_path) {
                println!("Error when copying a file: {err}");
                return;
            }

            let exec_file_path_str = exec_file_path.to_str().unwrap_or_default();
            println!("File copied to {}", exec_file_path_str);


            let root_str = c"Software\\Microsoft\\Windows\\CurrentVersion\\Run";
            let ptr = root_str.as_ptr();
            let pcstr = PCSTR::from_raw(ptr as *mut u8);
            
            let mut reg_key: HKEY = unsafe { mem::zeroed() };
            let reg_status = unsafe { RegCreateKeyA(
                HKEY_CURRENT_USER,
                pcstr,
                &mut reg_key
            ) };
        
            if !reg_status.is_ok() {
                println!("Create reg error: {:?}", reg_status);
                return;
            }
        
            let reg_status = unsafe { RegSetValueExA(
                reg_key,
                s!("WinClear"),
                0,
                REG_SZ,
                Some(format!("\"{}\" /run", exec_file_path_str).as_bytes())
            ) };
        
            if !reg_status.is_ok() {
                println!("Create reg \"URL Protocol\" error: {:?}", reg_status);
                return;
            }

            let reg_status = unsafe { RegCloseKey(reg_key) };
            if !reg_status.is_ok() {
                println!("Close reg error: {:?}", reg_status);
                return;
            }

            println!("Startup created");
        }

        "uninstall" => {
            let root_str = c"Software\\Microsoft\\Windows\\CurrentVersion\\Run\\WinClear";
            let ptr = root_str.as_ptr();
            let pcstr = PCSTR::from_raw(ptr as *const u8);
            
            let reg2 = unsafe { RegDeleteKeyA(HKEY_CURRENT_USER, pcstr) };
            if reg2.is_err() {
                println!("Removed reg err: {:?}", reg2);
            }

            let appdata = match std::env::var("LOCALAPPDATA") {
                Ok(res) => res,
                Err(err) => {
                    println!("Err: {}", err);
                    return;
                }
            };
            
            let mut service_file_path = PathBuf::from(appdata);
            service_file_path.push("WinClear");
            
            if service_file_path.exists() {
                if let Err(err) = fs::remove_dir_all(service_file_path) {
                    println!("Error when deleting a exists directory: {err}");
                }
            }

            println!("Startup deleted");
        }

        "run" => {
            core::run();
        }

        "quit" => {
            process::exit(0x0100);
        }

        _ => {
            println!("Unknown command: {command}");
        }
    }
}