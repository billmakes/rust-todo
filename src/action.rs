pub struct Action {
    pub command: String,
    pub id: usize,
    pub body: String,
}

impl Action {
    pub fn new(action_str: String) -> Self {
        let mut action_vec: Vec<String> = action_str
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let command = action_vec.remove(0);

        let id = if !action_vec.is_empty() {
            action_vec[0].parse::<usize>().unwrap_or(0)
        } else {
            0
        };

        let body = action_vec.join(" ");
        Self { command, id, body }
    }
}
