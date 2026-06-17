
# Claude Code Clone

This is a TUI application that allows you to use your favourite LLM (via openrouter) to assist you with coding. I made this project to learn how AI interacts with natibe applications and to have a fun project to work on. I hope you enjoy it as much as I do!

<img width="1526" height="1050" alt="feca8fab707e3f1d6319663eae144cadd62404b65fc9230924b51cb440ddf023" src="https://github.com/user-attachments/assets/6932c45c-2598-460f-aefb-9722f9ba62d7" />

## Tools implemented
- Read tool: ALlows AI model to read files on the host system.
- Write tool: Allows AI model to write files on the host system.
- Bash tool: Allows AI model to execute commands on the host system.

## Installation
1. Clone the repository
```bash
git clone https://github.com/arjav0703/claude-code-clone.git
```

2. Make sure you have cargo installed (https://rustup.rs)
3. Run the project
```bash
cd claude-code-clone
cargo run
```

### Keybinds
- `ALT + Enter`: Send message
- `Ctrl + Q`: Quit
- `Ctrl + N`: New chat
- `Ctrl + H`: Acess previous Chat
- `Ctrl + L`: Show Logs
- `Ctrl + .`: Settings panel

### Configuration file (`config.toml`)
1. Create a file called `config.toml` in the root directory of the project.
2. Add the following content to the file, replacing the values with your own:
```toml
OPENROUTER_API_KEY="your_openrouter_api_key_here"
MODEL_NAME = "openrouter/owl-alpha"
```

### AI usage disclosure
I had tab completions on throughout the development process. It helped every once in a while. I also used opencode for debugging when I wrote ugly code and confused myself 😭. Used ChatGPT for asking questions (mostly when i couldn't find something in the documentation)
