use color_eyre::Result;
use ratatui::DefaultTerminal;
use serde_json::{Value, json};

use super::App;

use crate::log;
use crate::tools::{handle_bash, handle_read_file, handle_write_file};
use crate::util::Role;
impl App {
    pub async fn run(&mut self, mut terminal: DefaultTerminal) -> Result<i32> {
        // while self.exit_code.is_none() {
        //     terminal.draw(|f| self.render(f))?;
        // }

        let app_state = &self.state;

        let mut messages = app_state.messages.lock().unwrap().clone();

        // Initial request, ask worker thread to fetch OpenAI API response
        log!("[main] sending initial request");
        app_state.message_sender.send(messages.clone()).unwrap();

        let response_rx = &app_state.message_receiver;

        loop {
            log!("[main] waiting for response from worker thread...");
            let response = match response_rx.recv() {
                Ok(r) => r,
                Err(e) => {
                    log!("[main] FAILED to receive response from worker: {e}");
                    break;
                }
            };
            log!(
                "[main] received response: {}",
                serde_json::to_string_pretty(&response).unwrap()
            );

            if let Some(tool_calls) = response["choices"][0]["message"]["tool_calls"].as_array() {
                for tool_call in tool_calls {
                    log!(
                        "processing tool call: {}",
                        serde_json::to_string_pretty(tool_call)?
                    );
                    let name = tool_call["function"]["name"].as_str().unwrap();
                    let arguments: Value =
                        serde_json::from_str(tool_call["function"]["arguments"].as_str().unwrap())?;
                    let tool_call_id = tool_call["id"].as_str().unwrap();

                    let tool_response = match name {
                        "ReadFile" => handle_read_file(&arguments, tool_call_id, name),
                        "WriteFile" => handle_write_file(&arguments, tool_call_id, name),
                        "Bash" => handle_bash(&arguments, tool_call_id, name),
                        other => {
                            log!("Unknown toolcall name: {}", other);
                            Ok(json!({
                                "role": "tool",
                                "tool_call_id": tool_call_id,
                                "name": name,
                                "content": format!("Unknown toolcall: {}", other)
                            }))
                        }
                    }
                    .unwrap_or_else(|e| {
                        log!("Error handling tool call {}: {}", name, e);
                        json!({
                            "role": "tool",
                            "tool_call_id": tool_call_id,
                            "name": name,
                            "content": format!("Error handling tool call {}: {}", name, e)
                        })
                    });
                    messages.push(tool_response);
                }
                app_state.message_sender.send(messages.clone()).unwrap();
            } else if let Some(content) = response["choices"][0]["message"]["content"].as_str() {
                log!("no toolcall");
                log!("assistant response content: {}", content);
                messages.push(json!({
                    "role": Role::assistant,
                    "content": content
                }));
                log!("{}", content);
                break;
            } else {
                log!("Unexpected response format: {}", response);
                break;
            }
        }
        //
        // // Ok(())
        //
        // loop {
        //     terminal.draw(render)?;
        //     if crossterm::event::read()?.is_key_press() {
        //         break Ok(());
        //     }
        // }
        self.render_dummy_widget(&mut terminal.get_frame());
        Ok(0)
    }
}
