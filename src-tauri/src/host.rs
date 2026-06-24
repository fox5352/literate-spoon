use std::fs;

const WIN_PATH: &str = r"C:\Windows\System32\drivers\etc\hosts";
const LINUX_PATH: &str = r"/etc/hosts";

const BLOCK_START: &str = "# === BLOCKER START ===";
const BLOCK_END: &str = "# === BLOCKER END ===";

const HOST_FILE_START: &str = "# ====================================================="; // after
const HOST_FILE_END: &str = "# blacklist"; // before

pub fn parse_host_file(content: &str) -> Result<Vec<(String, String)>, std::io::Error> {
    let t = if let (Some(start), Some(end)) =
        (content.find(HOST_FILE_START), content.find(HOST_FILE_END))
    {
        Ok(content[start + HOST_FILE_START.len()..end].to_string())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Failed to parse host file",
        ))
    }?;

    let lines = t.lines();

    let mut entries = Vec::new();

    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.len() >= 2 {
            let ip = parts[0].to_string();
            let domain = parts[1].to_string();
            entries.push((ip, domain));
        }
    }

    Ok(entries)
}

fn flush_dns() -> Result<(), std::io::Error> {
    match std::env::consts::OS {
        "windows" => {
            let t = std::process::Command::new("ipconfig")
                .args(&["/flushdns"])
                .output()
                .map_err(|e| {
                    eprintln!("{}", e);
                    std::io::Error::new(std::io::ErrorKind::Other, "Failed to flush dns")
                })?;

            Ok(())
        }
        "linux" => Ok(()),
        // "macos" => Ok(LINUX),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Platform not supported",
        )),
    }?;
    Ok(())
}

pub fn remove_blacklist() -> Result<(), std::io::Error> {
    let path = match std::env::consts::OS {
        "windows" => Ok(WIN_PATH),
        "linux" => Ok(LINUX_PATH),
        // "macos" => Ok(LINUX),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Platform not supported",
        )),
    }?;

    let file_content = fs::read_to_string(path)?;
    if let (Some(start), Some(end)) = (file_content.find(BLOCK_START), file_content.find(BLOCK_END))
    {
        let end_idx = end + BLOCK_END.len();
        let mut cleaned = file_content[..start].to_string();
        cleaned.push_str(&file_content[end_idx..]);
        fs::write(path, cleaned)?;
        flush_dns()?;
    }

    Ok(())
}

pub fn inject_blacklist(domains: &[(String, String)]) -> Result<(), std::io::Error> {
    remove_blacklist()?;

    let path = match std::env::consts::OS {
        "windows" => Ok(WIN_PATH),
        "linux" => Ok(LINUX_PATH),
        // "macos" => Ok(LINUX),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Platform not supported",
        )),
    }?;

    let mut block = String::new();

    block.push_str(BLOCK_START);
    block.push('\n');

    for (ip, domain) in domains {
        block.push_str(&format!("{} {}\n", ip, domain));
    }

    block.push_str(BLOCK_END);
    block.push('\n');

    let mut file_content = fs::read_to_string(path)?;
    // add the blocklist at the end of the file
    file_content.push('\n');
    file_content.push_str(&block);
    fs::write(path, file_content)?;
    flush_dns()?;
    Ok(())
}
