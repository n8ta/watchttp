use std::process::Command;

pub fn send_notification(title: &str, message: &str) {
    println!("title: {}, message: {}", title, message);
    Command::new("/Users/n8ta/.rbenv/shims/terminal-notifier")
        .arg("-message")
        .arg(format!("\"{}\"", message))
        .arg("-title")
        .arg(format!("\"{}\"", title))
        .spawn()
        .expect("Failed to send notification");
}