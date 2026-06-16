use crate::app::App;

impl App<'_> {
    pub fn save_current_conversation(&self) {
        let messages = self.state.messages.lock().unwrap();
        let conversation = serde_json::to_string_pretty(&*messages).unwrap();
        std::fs::write(format!("{}.json", self.state.chat_id), conversation).unwrap();
    }

    pub fn get_history_list(&self) -> Vec<String> {
        let mut history_list = vec![];
        for entry in std::fs::read_dir(".").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file()
                && path.extension().is_some_and(|ext| ext == "json")
                && path.to_str().is_some_and(|s| s.contains("chat"))
                && let Some(file_name) = path.file_stem().and_then(|s| s.to_str())
            {
                history_list.push(file_name.to_string());
            }
        }
        history_list.sort();
        history_list
    }
}
