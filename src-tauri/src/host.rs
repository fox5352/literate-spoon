use std::{collections::HashMap, fs};

const WIN: &str = r"C:\Windows\System32\drivers\etc\hosts";
const LINUX: &str = r"/etc/hosts";

const BLOCK_START: &str = "# === BLOCKER START ===";
const BLOCK_END: &str = "# === BLOCKER END ===";

const HOST_FILE_START: &str = "# =====================================================";// after
const HOST_FILE_END: &str = "# blacklist";// before

// let path = match std::env::consts::OS {
//     "windows" => Ok(WIN),
//     "linux" => Ok(LINUX),
//     // "macos" => Ok(LINUX),
//     _ => Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "Platform not supported"))
// }?;

pub fn parse_host_file(content: String) -> Result<Vec<(String, String)>, std::io::Error> {
    let t = if let (Some(start), Some(end)) = (content.find(HOST_FILE_START), content.find(HOST_FILE_END)) { 
        content[start + HOST_FILE_START.len()..end].to_string()
    }else {
        
    }

    Ok(vec![])
}