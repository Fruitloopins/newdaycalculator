use std::io;

fn main() {
    println!("EarthMC New Day Calculator");
    println!(" ");

    loop {
        println!("Enter day of last new day: ");
            let mut last_new_day_input = String::new();
            io::stdin().read_line(&mut last_new_day_input).expect("Not a valid string");
            let last_new_day_input: i64 = last_new_day_input.trim().parse().expect("Not a valid number");

        println!("Enter current average TPS: ");
            let mut tps = String::new();
            io::stdin().read_line(&mut tps).expect("Not a valid string");
            let tps: i64 = tps.trim().parse().expect("Not a valid number");

        println!("Enter the current day: ");
            let mut current_day_input = String::new();
            io::stdin().read_line(&mut current_day_input).expect("Not a valid string");
            let current_day_input: i64 = current_day_input.trim().parse().expect("Not a valid number");
        println!("-------------------------------");

            //Calculations for what tick previous new day was on, what the new day's tick will be and what the current tick is.
            let last_new_day: i64 = last_new_day_input * 24000;
                println!("last_new_day = {}", last_new_day);

            let next_new_day = last_new_day + 1728000;
                println!("next_new_day = {}", next_new_day);

            let current_day = current_day_input * 24000;
                println!("current_day = {}", current_day);

            //Calculations for percent tps has slowed down the server's tick speed to be applied to time remaining until next new day.
            let percent_slow: i64 = tps / 20 * 100;
                println!("percent_slow = {}", percent_slow);

            let tps_percent_slowdown: i64 = 100 - percent_slow;
                println!("tps_percent_slowdown = {}", tps_percent_slowdown);

            //Calculations of how much time is actually left, time remaining in ticks from current tick to next new day tick,
            //adding ticks to that based off of tps slowdown and actually calculating the time in understandable hours.
            let time_remaining = next_new_day - current_day;
                println!("time_remaining = {}", time_remaining);

            let percent_increase = time_remaining * tps_percent_slowdown / 100;
                println!("percent_increase = {}", percent_increase);

            let time_until_new_day = time_remaining + percent_increase;
                println!("time_until_new_day = {}", time_until_new_day);

            let hours_remaining = time_until_new_day / 20 / 60 /60;
                println!("hours_remaining = {}", hours_remaining);

        println!("-------------------------------");
        println!(" ");
        println!("Hours remaining until new day: {}",hours_remaining);
        println!(" ");
        println!("This number is an estimate but will be more accurate the closer to next new day it is.");
        println!("It will not be accurate if the server has restarted since the previous new day.");
    };
}