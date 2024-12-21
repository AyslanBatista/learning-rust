const SECONDS_IN_MINUTE: u32 = 60;

fn main() {
    const MINUTES_IN_HOUR: u32 = 60;
    const SECONDS_IN_HOUR: u32 = MINUTES_IN_HOUR * SECONDS_IN_MINUTE;

    let total = 30;
    let total_em_segundos = total * SECONDS_IN_HOUR;

    println!("Trabalhou {} segundos", total_em_segundos);
}
