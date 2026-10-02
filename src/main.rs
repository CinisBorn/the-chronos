use std::time::Duration;
use std::io::Write;

fn main() {
    today();
}

fn today() {
    loop {
        use time::OffsetDateTime;
        
        let now = OffsetDateTime::now_local().unwrap();
        let hour = now.hour() as u64;
        let minute = now.minute() as u64;
        let seconds = now.second() as u64;
        let today = now.date();

        let total_hours   = 24;
        let total_minutes = 1_440;
        let total_seconds = 86_400;

        // --- Minutes elapsed time conversion ---
        //
        // Converts the current hours into minutes.
        // This gives the number of complete minutes that have passed since midnight.
        // Then adds the current minutes.
        // 
        // --- Seconds elapsed time conversion ---
        //
        // Finds how many seconds are in one hour:
        // 60 minutes * 60 seconds = 3,600 seconds.
        //
        // Converts the current hours into seconds:
        // hour * 3,600.
        //
        // Converts the current minutes into seconds:
        // minute * 60.
        //
        // Adds the current seconds to the total.
        let elapsed_minutes = hour * 60 + minute;
        let elapsed_seconds = hour * 3600 + minute * 60 + seconds;
        
        print!("\x1B[2J\x1B[H");
        std::io::stdout().flush().unwrap();

        println!("Current Date: {today}");
        println!("Current Time: {hour}:{minute}:{seconds}\n");
        
        println!("Hours: {hour}/{total_hours}");
        println!("Seconds: {elapsed_seconds}/{total_seconds}");
        println!("Minutes: {elapsed_minutes}/{total_minutes}");
        
        std::thread::sleep(Duration::from_secs(1));
    }
}