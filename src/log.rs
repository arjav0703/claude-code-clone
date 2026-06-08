#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ({
        use std::fs::File;
        use std::io::Write;

        let now = std::time::SystemTime::now();
        let datetime: chrono::DateTime<chrono::Local> = now.into();

        let mut file = File::options()
        .append(true)
        .create(true)
        .open("log.txt").expect("Unable to open log file");

        file.write_all( format!("{} - {}\n", datetime.format("%Y-%m-%d %H:%M:%S"), format!($($arg)*)).as_bytes()).expect("Unable to write to log file");
    })
}
