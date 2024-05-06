enum SLabel {
    Startup,
    Update,
}

trait ScheduleLabel {
    fn get_label(&self) -> SLabel;
}

pub struct Startup;
impl ScheduleLabel for Startup {
    fn get_label(&self) -> SLabel {
        SLabel::Startup
    }
}

pub struct Update;
impl ScheduleLabel for Update {
    fn get_label(&self) -> SLabel {
        SLabel::Update
    }
}

pub struct App {
    startups: Vec<fn() -> ()>,
    updates: Vec<fn() -> ()>,
}

impl App {
    pub fn new() -> App {
        App { 
            startups: vec![],
            updates: vec![]
        }
    }

    pub fn add_systems(&mut self, schedule: impl ScheduleLabel, system: fn() -> ()) -> &mut Self {
        match schedule.get_label() {
            SLabel::Startup => &self.startups.push(system),
            SLabel::Update => &self.updates.push(system),
        };
        self
    }

    pub fn run(&self) {
        for startup in &self.startups {
            startup();
        }

        for update in &self.updates {
            update();
        }
    }
}

#[cfg(test)]
mod tests {}
