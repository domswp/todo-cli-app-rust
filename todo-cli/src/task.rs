#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub done: bool,
}

impl Task {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.trim().to_string(),
            done: false,
        }
    }

    pub fn display(&self) -> String {
        let status = if self.done { "[x]" } else { "[ ]" };
        format!("{} {}", status, self.name)
    }

    pub fn to_string(&self) -> String {
        format!("{}|{}", if self.done { "1" } else { "0" }, self.name)
    }

    pub fn from_string(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.splitn(2, "|").collect();
        if parts.len() != 2 {
            return None;
        }
        Some(Self {
            done: parts[0] == "1",
            name: parts[1].to_string(),
        })
    }
}

