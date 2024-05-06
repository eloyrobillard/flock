use flock::*;

fn main() {
    App::new().add_systems(Startup, basic_window_system).run();
}

fn basic_window_system() {}
