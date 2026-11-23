//! `src/utils/pid.ts`: the name of the parent process.

pub fn process_name(pid: u32) -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
        let start = stat.find('(')?;
        let end = stat.find(')')?;
        return Some(stat[start + 1..end].to_string());
    }
    #[cfg(target_os = "macos")]
    {
        let out = std::process::Command::new("ps")
            .args(["-o", "comm=", &pid.to_string()])
            .output()
            .ok()?;
        if !out.stderr.is_empty() {
            return None;
        }
        return Some(String::from_utf8_lossy(&out.stdout).trim().to_string());
    }
    #[cfg(windows)]
    {
        let root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".into());
        let out = std::process::Command::new(format!("{root}\\System32\\tasklist.exe"))
            .args(["/FI", &format!("PID eq {pid}"), "/FO", "CSV", "/NH"])
            .output()
            .ok()?;
        let s = String::from_utf8_lossy(&out.stdout);
        let line = s.lines().next()?;
        let mut parts = line.split("\",\"");
        let name = parts.next()?.trim_start_matches('"').to_string();
        let p = parts.next()?;
        return (p == pid.to_string()).then_some(name);
    }
    #[allow(unreachable_code)]
    None
}

pub fn parent_pid() -> u32 {
    #[cfg(unix)]
    {
        std::os::unix::process::parent_id()
    }
    #[cfg(not(unix))]
    {
        0
    }
}

/// `pid.isPowerShell()`.
pub fn is_powershell() -> bool {
    let n = process_name(parent_pid()).unwrap_or_default();
    n.contains("powershell") || n.contains("pwsh")
}
