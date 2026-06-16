use crate::app::App;

impl App<'_> {
    pub fn save_current_conversation(&self) {
        let messages = self.state.messages.lock().unwrap();
        let conversation = serde_json::to_string_pretty(&*messages).unwrap();
        std::fs::write(format!("{}.json", self.state.chat_id), conversation).unwrap();
    }
}
