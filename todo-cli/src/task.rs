#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub done: bool,
    pub deadline: Option<String>,
}

impl Task {
    pub fn new(name: &str, deadline: Option<String>) -> Self {
        Self {
            name: name.trim().to_string(),
            done: false,
            deadline,
        }
    }

    pub fn display(&self) -> String {
        let status = if self.done { "[x]" } else { "[ ]" };
        let deadline = match &self.deadline {
            Some(d) => format!(" (deadline: {})", d),
            None => "".to_string(),
        };
        format!("{} {}{}", status, self.name, deadline)
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}|{}|{}",
            if self.done { "1" } else { "0" },
            self.name,
            self.deadline.clone().unwrap_or_default()
        )
    }

    pub fn from_string(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.splitn(3, "|").collect();
        if parts.len() < 2 {
            return None;
        }

        Some(Self {
            done: parts[0] == "1",
            name: parts[1].to_string(),
            deadline: if parts.len() == 3 && !parts[2].is_empty() {
                Some(parts[2].to_string())
            } else {
                None
            },
        })
    }
}

