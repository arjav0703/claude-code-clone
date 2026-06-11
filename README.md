# Claude Code Clone

This is a TUI application that allows you to use your favourite LLM (via openrouter) to assist you with coding. I made this project to learn how AI interacts with natibe applications and to have a fun project to work on. I hope you enjoy it as much as I do!


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

### Environment Variables
- `OPENROUTER_API_KEY`
- `OPENROUTER_MODEL` (name of the model you want to use, e.g. `openrouter/owl-alpha`)


### AI usage disclosure
I had tab completions on throughout the development process. It helped every once in a while. I also used opencode for debugging when I wrote ugly code and confused myself 😭. Used ChatGPT for asking questions (mostly when i couldn't find something in the documentation)
