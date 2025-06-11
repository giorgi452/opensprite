use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct ProjectController {
    pub name: String,
    pub path: String,
    pub path_overridden: bool,
}

impl ProjectController {
    pub fn new() -> Self {
        ProjectController {
            name: String::from("New Project"),
            path: String::from(format!("/home/{}/Pictures/New Project.png", "giorgi",)),
            path_overridden: false,
        }
    }
}
