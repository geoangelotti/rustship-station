use inquire::Text;

mod menu;
use menu::menu;

mod station;
use station::*;

fn main() {
    let mut station = Station::new();
    let mut station_log = vec![];
    loop {
        // main game loop!
        let days_left = station.days_left();
        if days_left < 1 {
            println!("(END-TRANSMISSION)");
            break;
        }
        println!("{days_left} UNTIL FINAL TRANSMISSION");
        station_log.push(Text::new("Enter your log:").prompt().unwrap());
        match menu(&["NEW DAY".into(), "STATUS".into(), "POWERDOWN".into()]).as_str() {
            "NEW DAY" => {
                // menu system defined after here}
                station.new_day();
                match menu(&["REPAIR".into(), "SCIENCE".into()]).as_str() {
                    "REPAIR" => {
                        repair(menu(&station.broken_sections()), &mut station);
                        continue;
                    }
                    "SCIENCE" => {
                        science(menu(&station.working_sections()), &mut station);
                        continue;
                    }
                    &_ => panic!(),
                }
            }
            "STATUS" => station.status(),
            "POWERDOWN" => break,
            &_ => panic!("test"),
        }
    }
    dbg!(station_log);
}
