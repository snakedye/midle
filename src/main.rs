mod wayland;

use wayland_client::{Display, GlobalManager, Main};
use std::process::{Command};
use crate::wayland::idle::{
    org_kde_kwin_idle::OrgKdeKwinIdle,
    org_kde_kwin_idle_timeout::Event
};
use wayland_client::protocol::{ wl_seat, wl_seat::WlSeat };

struct Globals {
    seats: Vec<Main<WlSeat>>,
    idle_manager: Option<Main<OrgKdeKwinIdle>>,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub name: Option<String>,
    pub command: Option<String>,
    pub timeout: u32,
}

impl Config {
    pub fn new() -> Config {
        { Config {
            name: None,
            command: None,
            timeout: 1 << 31
        } }
    }
    pub fn filter(self, seat: &Main<WlSeat>, idle_manager: &Main<OrgKdeKwinIdle>) {
        let idle_timeout = idle_manager
            .get_idle_timeout(seat, self.timeout);
        let mut command = if let Some(command) = &self.command {
            let mut string = command.split_whitespace();
            let mut command = Command::new(string.next().unwrap());
            command.args(string.collect::<Vec<&str>>());
            Some(command)
        } else { None };
        idle_timeout.quick_assign(move |_, event, _| match event {
            Event::Idle => if let Some(command) = &mut command {
                command.spawn().expect("Error");
            }
            Event::Resumed => {}
        });
    }
}

fn main() {
    let display = Display::connect_to_env().unwrap();

    let mut event_queue = display.create_event_queue();

    let attached_display = (*display).clone().attach(event_queue.token());

    let mut globals: Globals = { Globals {
        seats: Vec::new(),
        idle_manager: None
    } };

    let mut configuration: Vec<Config> = Vec::new();

    let mut args = std::env::args();
    args.next();
    let mut collect = false;
    let mut fields = (false, false, false);
    loop {
        let len = configuration.len();
        match args.next() {
            Some(flag) => match flag.as_str() {
                "-c" | "--command" => { 
                    collect = true;
                    fields = (false, false, false); 
                    configuration.push(Config::new());
                }
                "-h" => { 
                    print!("Usage: midle [option]\n\n");
                    println!("    -c [timeout] [command] [seat_name] : the timeout is in second");
                    std::process::exit(0);
                }
                _ => if collect {
                    if !fields.0 {
                        fields.0 = true;
                        configuration[len-1].timeout = flag.parse::<u32>().unwrap() * 1000;
                    } else if !fields.1 {
                        fields.1 = true;
                        configuration[len-1].command = Some(flag);
                    } else if !fields.2 {
                        fields.2 = true;
                        configuration[len-1].command = Some(flag);
                    }
                }
            },
            None => break,
        }
    }

    GlobalManager::new_with_cb(
        &attached_display,
        wayland_client::global_filter!(
            [
                WlSeat,
                7,
                |seat: Main<WlSeat>, mut globals: DispatchData| {
                    globals.get::<Globals>().unwrap().seats.push(seat);
                }
            ],
            [
                OrgKdeKwinIdle,
                1,
                |idle: Main<OrgKdeKwinIdle>, mut globals: DispatchData| {
                    globals.get::<Globals>().unwrap().idle_manager = Some(idle);
                }
            ]
        ),
    );

    event_queue
        .sync_roundtrip(&mut globals, |_, _, _| unreachable!())
        .unwrap();

    for seat in globals.seats {
        let configuration = configuration.clone();
        seat.quick_assign(move |seat, event, mut idle_manager| match event {
            wl_seat::Event::Name{ name } => for config in configuration.clone() {
                if {
                match config.name.as_ref() {
                    Some(seat_name) => name.eq(seat_name),
                    None => true
                }
                } {
                    let idle_manager = idle_manager.get::<Main<OrgKdeKwinIdle>>().unwrap();
                    config.filter(&seat, &idle_manager);
                } else {
                    seat.quick_assign(move |_, _, _| {});
                }
            }
            _ => {}
        })
    }

    if let Some(mut idle_manager) = globals.idle_manager {
        loop {
            event_queue
                .dispatch(&mut idle_manager, |event, object, _| {
                    panic!(
                        "[callop] Encountered an orphan event: {}@{}: {}",
                        event.interface,
                        object.as_ref().id(),
                        event.name
                    );
                })
                .unwrap();
        }
    }
}

