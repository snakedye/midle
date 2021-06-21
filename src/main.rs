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
    pub resume_command: Option<String>,
    pub timeout: u32,
}

impl Config {
    pub fn new() -> Config {
        { Config {
            name: None,
            command: None,
            resume_command: None,
            timeout: 1 << 31
        } }
    }
    pub fn filter(self, seat: &Main<WlSeat>, name: String, idle_manager: Option<&Main<OrgKdeKwinIdle>>) {
        let idle_timeout = idle_manager
            .expect("Compositor doesn\'t implement the idle protocol")
            .get_idle_timeout(seat, self.timeout);
        if {
            match &self.name {
                Some(seat_name) => name.eq(seat_name),
                None => true
            }
        } {
            idle_timeout.quick_assign(move |_, event, _| match event {
                Event::Idle => { 
                    if let Some(value) = &self.command {
                        run_command(value.clone())
                    }
                }
                Event::Resumed => { 
                    if let Some(value) = &self.resume_command {
                        run_command(value.clone())
                    }
                }
            });
        }
    }
}

fn run_command(value: String) {
    let mut string = value.split_whitespace();
    let mut command = Command::new(string.next().unwrap());
    command.args(string.collect::<Vec<&str>>());
    command.spawn().expect("Error");
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

    let mut atom_count = 0;
    let mut atom = Config::new();
    let mut args = std::env::args();
    loop {
        match args.next() {
            Some(flag) => match flag.as_str() {
                "-c" | "--command" => { 
                    if atom_count > 0 {
                        configuration.push(atom);
                        atom = Config::new();
                    }
                    if let Some(value) = args.next() {
                        atom.timeout = value.parse::<u32>().unwrap() * 1000;
                    }
                    atom_count += 1;
                    atom.command = args.next();
                }
                "-r" | "--resume" => { 
                    atom.resume_command = args.next();
                }
                "-s" | "--seat-name" => {
                    atom.name = args.next();
                }
                "-h" => { 
                    print!("Usage: midle [option]\n\n");
                    print!("  -c, --command <timeout> <command>     the timeout is in second\n");
                    print!("  -r, --resume <command>                resume command, follows a regular command\n");
                    println!("  -s, --seat_name <seat_name>           the name of the seat associated to a command");
                    std::process::exit(0);
                }
                _ => {} 
            },
            None => {
                configuration.push(atom);
                break;
            },
        }
    }

    GlobalManager::new_with_cb(
        &attached_display,
        wayland_client::global_filter!(
            [
                WlSeat,
                7,
                |seat: Main<WlSeat>, mut globals: DispatchData| {
                    seat.quick_assign(move |_, event, mut placeholder| match event {
                        wl_seat::Event::Name{ name } => (*placeholder.get::<String>().unwrap()) = name,
                        _ => {}
                    });
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
        let mut seat_name = String::new();
        event_queue
            .sync_roundtrip(&mut seat_name, |_, _, _| unreachable!())
            .unwrap();
        for config in &configuration {
            config.clone().filter(&seat, seat_name.clone(), globals.idle_manager.as_ref());
        }
    }

    if globals.idle_manager.is_some() {
        loop {
            event_queue
                .dispatch(&mut (), |event, object, _| {
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

