use anyhow::Result;
use std::process::Command;

pub fn notify(title: &str, message: &str) -> Result<()> {
    if Command::new("terminal-notifier")
        .args(["-title", title, "-message", message])
        .status()
        .is_ok_and(|status| status.success())
    {
        return Ok(());
    }

    let script = format!("display notification {:?} with title {:?}", message, title);
    let _ = Command::new("osascript").args(["-e", &script]).status();
    Ok(())
}

pub fn play_sound() -> Result<()> {
    let candidates = [
        "/System/Library/Sounds/Glass.aiff",
        "/System/Library/Sounds/Ping.aiff",
    ];
    for sound in candidates {
        if Command::new("afplay")
            .arg(sound)
            .status()
            .is_ok_and(|status| status.success())
        {
            return Ok(());
        }
    }
    println!("Sound unavailable; Pomolife will continue silently.");
    Ok(())
}
