extern crate systemstat;
use systemstat::{System, Platform};
use std::{thread,env,time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();
    let sys = System::new();
    for i in args{
        match i.as_str() {
            "battery" => {
                match sys.battery_life(){
                    Ok(battery) =>
                        println!("{}",
                               battery.remaining_capacity*100.0,
                            ),
                    Err(x) => println!("Battery: error: {}", x)
                }
            },
            "power" => {
                match sys.on_ac_power() {
                    Ok(power) => match power{
                        true => println!("charging"),
                        false => println!("discharging")
                    },
                    Err(x) => println!("AC power: error: {}", x)
                }            
            },
            "memory" => {
                match sys.memory(){
                    Ok(mem) => {
                        let used = mem.total.as_u64()-mem.free.as_u64();
                        let percent = (used as f64 / mem.total.as_u64() as f64) * 100.0;
                        println!(
                        "{}",
                         percent
                        )
                    },
                    Err(x) => println!("Memory: error: {}", x)
                }
            },
            "temp" => {
                match sys.cpu_temp() {
                    Ok(cpu_temp) => println!("{}", cpu_temp),
                    Err(x) => println!("{}", x)
                }
            },
            "load" => {
                match sys.load_average() {
                    Ok(loadavg) => println!("{}", loadavg.one),
                    Err(x) => println!("Load average: error: {}", x)
                }
            },
            "cpu" => {
                match sys.cpu_load_aggregate() {
                    Ok(cpu)=> {
                        thread::sleep(Duration::from_secs(1));
                        let cpu = cpu.done().unwrap();
                        println!("{}",
                            100.0 - cpu.idle * 100.0
                        );
                    },
                    Err(x) => println!("CPU load: error: {}", x)
                }
            }
            _ => print!(""),
        }
    }
}