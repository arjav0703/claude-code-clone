use super::AppState;
use async_openai::Client;
use serde_json::{Value, json};
use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
};

use crate::{app::Model, log, util::ToolSpec};

impl AppState {
    pub fn listen_for_messages(
        &self,
        request_rx: Receiver<Vec<Value>>,
        response_tx: Sender<Value>,
    ) {
        let config = self.config.clone();
        let model = self.model.name.clone();

        thread::spawn(move || {
            log!("[worker thread] started");
            let rt = tokio::runtime::Runtime::new().unwrap();
            let client = Client::with_config(config);

            while let Ok(messages) = request_rx.recv() {
                log!(
                    "[worker thread] received request: {}",
                    serde_json::to_string_pretty(&messages).unwrap()
                );
                let value = client.clone();
                let model = model.clone();
                let response: Result<Value, async_openai::error::OpenAIError> =
                    rt.block_on(async move {
                        log!("[worker thread][tokio] creating and sending OpenAI chat request");
                        let res = value
                            .chat()
                            .create_byot(json!({
                                "messages": messages,
                                "model": model,
                                "tools": [
                                    json!({
                                        "type": "function",
                                        "function": ToolSpec::read_file()
                                    }),
                                    json!({
                                        "type": "function",
                                        "function": ToolSpec::write_file()
                                    }),
                                    json!({
                                        "type": "function",
                                        "function": ToolSpec::bash()
                                    })
                                ]
                            }))
                            .await;
                        log!("[worker thread][tokio] OpenAI chat returned");
                        res
                    });
                log!(
                    "[worker thread] completed OpenAI call, result variant: {}",
                    if response.is_ok() { "Ok" } else { "Err" }
                );
                // On error, return an error Value
                let resp_val = match response {
                    Ok(ref r) => {
                        log!(
                            "[worker thread] AI response: {}",
                            serde_json::to_string_pretty(r).unwrap()
                        );
                        r.clone()
                    }
                    Err(ref e) => {
                        log!("[worker thread] OpenAI error: {e}");
                        json!({"error": format!("{e}")})
                    }
                };
                if let Err(e) = response_tx.send(resp_val) {
                    log!("[worker thread] FAILED to send response to main thread: {e}");
                } else {
                    log!("[worker thread] sent result to main thread");
                }
            }
        });
    }
}
