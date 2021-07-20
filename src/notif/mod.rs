use notify_rust::Notification;


pub fn send_notification(title: &str, message: &str) {
    if let Err(e) = Notification::new()
        .summary(title)
        .body(message)
        .show() {
        eprintln!("Something went wrong sending a notification:\n{:?}", e)
    }
}