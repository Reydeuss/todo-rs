pub struct Task {
    title: String,
    description: String,
    status: bool,
}
impl Task {
    pub fn new(title: &str, description: &str, status: bool) -> Self {
        Task {
            title: String::from(title),
            description: String::from(description),
            status,
        }
    }

    fn change_status(&mut self) {
        self.status = !self.status;
    }

    fn status_text(&self) -> &str {
        if self.status { "Done" } else { "Not Done Yet" }
    }

    pub fn stringify(&self) -> String {
        let title_text: &str = self.title.as_str();
        let description_text: &str = self.description.as_str();
        let status_text: &str = if self.status { "true" } else { "false" };

        return format!("{},{},{}", title_text, description_text, status_text);
    }
}

pub type TaskList = Vec<Task>;
