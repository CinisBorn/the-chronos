fn main() {
    today();
}

fn today() {
    use time::OffsetDateTime;
    
    let now = OffsetDateTime::now_local().unwrap();
    let hour = now.hour();
    let minute = now.minute();
    let seconds = now.minute();
    let today = now.date();
    
    println!("Date: {today}");
    println!("Time: {hour}:{minute}:{seconds}");
}