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
        let minutes = now.minute() as u64;
        let seconds = now.second() as u64;
        let today = now.date();

        print!("\x1B[2J\x1B[H");
        std::io::stdout().flush().unwrap();

        println!(" \x1B[33mGeneral Information:\x1B[0m\n");
        println!(" Current Date: {today}");
        println!(" Current Time: {hour}:{minutes}:{seconds}\n");
        
        let elapsed_minutes = calculate_elapsed_time(hour, minutes, seconds).0;
        let elapsed_seconds = calculate_elapsed_time(hour, minutes, seconds).1;
        
        remaining_time(hour, minutes, seconds);
        available_time(hour, elapsed_minutes, elapsed_seconds);
        
        std::thread::sleep(Duration::from_secs(1));
    }
}

/// Subtracts sleeping time.
///
/// Assumes the user wakes up at 6 AM. The time from midnight to 06:00:00
/// is subtracted from the current elapsed time, returning the available time in a day.
fn available_time(hour: u64, minutes: u64, seconds: u64) {
    let sleep_hours: u32 = 8;
    
    let available_hours: u32 = 24 - sleep_hours;
    let available_minutes: u32 = available_hours * 60;
    let available_seconds: u32 = available_hours * 3_600;
    
    let wake_hour = 6;
    let wake_minutes = 0;
    let wake_seconds = 0;

    let wake_available_hour = hour.saturating_sub(wake_hour);
    let wake_available_min = wake_hour * 60 + wake_minutes;
    let wake_available_sec = wake_hour * 3600 + wake_minutes * 60 + wake_seconds;
    
    let elapsed_available_min = minutes.saturating_sub(wake_available_min);
    let elapsed_available_sec = seconds.saturating_sub(wake_available_sec);

    println!(" \x1B[33mAvailable Time Counting Sleep Time:\x1B[0m\n");
    println!(" Hours: {wake_available_hour}/{available_hours}");
    println!(" Minutes: {elapsed_available_min}/{available_minutes}");
    println!(" Seconds: {elapsed_available_sec}/{available_seconds}");
}

fn remaining_time(hour: u64, minutes: u64, seconds: u64) {
    let total_hours  : u32 = 24;
    let total_minutes: u32 = 1_440;
    let total_seconds: u32 = 86_400;

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
    let elapsed_minutes = calculate_elapsed_time(hour, minutes, seconds).0;
    let elapsed_seconds = calculate_elapsed_time(hour, minutes, seconds).1;

    println!(" \x1B[33mRemaining Time Comparation:\x1B[0m\n");
    println!(" Hours: {hour}/{total_hours}");
    println!(" Minutes: {elapsed_minutes}/{total_minutes}");
    println!(" Seconds: {elapsed_seconds}/{total_seconds}\n");
}

fn calculate_elapsed_time(hour: u64, minutes: u64, seconds: u64) -> (u64, u64) {
    let elapsed_minutes = hour * 60 + minutes;
    let elapsed_seconds = hour * 3600 + minutes * 60 + seconds;

    (elapsed_minutes, elapsed_seconds)
}