use std::time::Duration;
use std::io::Write;

fn main() {
    today();
}

fn today() {
    loop {
        use time::OffsetDateTime;
        
        let now = OffsetDateTime::now_local().unwrap();
        let hour = now.hour();
        let minute = now.minute();
        let seconds = now.second();
        let today = now.date();
        
        print!("\x1B[2J\x1B[H");
        std::io::stdout().flush().unwrap();

        println!("Date: {today}");
        println!("Time: {hour}:{minute}:{seconds}");

        std::thread::sleep(Duration::from_secs(1));
    }
}