# AI Commit

AI Commit is a command-line tool that uses AI to automatically generate commit messages for your git repositories. It's designed with a "configure once, use anywhere" philosophy.

## Installation

1.  **Install Rust:** If you don't have Rust installed, get it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
2.  **Build the project:**
    ```bash
    cargo build --release
    ```
    The executable will be at `target/release/ai_commit`. It's recommended to move this executable to a directory in your system's `PATH` (e.g., `/usr/local/bin`) for easy access.

## Configuration

`ai_commit` uses a global configuration file, so you only need to set it up once.

### 1. Set Your API Key

First, you must provide an API key.

```bash
ai_commit config set-api-key <YOUR_API_KEY>
```

### 2. Configure Your Model Endpoint (Optional)

By default, `ai_commit` uses the OpenAI API endpoint. If you want to use a different provider (like DeepSeek, or a local Ollama instance) that is compatible with the OpenAI API format, you can set a custom URL.

```bash
# Example for DeepSeek
ai_commit config set-url https://api.deepseek.com/chat/completions

# Example for a local Ollama model
ai_commit config set-url http://localhost:11434/v1/chat/completions
```

### 3. Specify the Model Name (Optional, but recommended for custom endpoints)

You can also specify the exact model name to use. This is often required for custom API endpoints.

```bash
# For DeepSeek
ai_commit config set-model deepseek-chat

# For OpenAI (default)
ai_commit config set-model gpt-3.5-turbo

# For a specific Ollama model, e.g., "llama3"
ai_commit config set-model llama3
```

### 4. Other Configurations (Optional)

You can also set a default language for your commit messages or a custom prompt to further guide the AI.

```bash
# Set the language to Spanish
ai_commit config set-language es

# Set a custom prompt to generate conventional commits
ai_commit config set-prompt "Generate a conventional commit message. The format should be: <type>[optional scope]: <description>"
```

To see your current settings at any time, run:
```bash
ai_commit config show
```

## Usage

Once configured, simply run `ai_commit` in your git repository when you have staged changes:

```bash
ai_commit
```

This will:
1. Get the staged diff.
2. Generate a commit message using your configured settings.
3. Open the message in Vim (or your default editor) for you to review and edit.
4. Once you save and close the editor, the commit will be made.

You can temporarily override any saved setting with command-line flags:

```bash
# Generate a one-off commit message in French
ai_commit --language fr

# Use a specific model for a single commit
ai_commit --model gpt-4-turbo
```

## Git Hook Integration

To automatically generate a commit message every time you run `git commit`, you can use a `prepare-commit-msg` hook.

1.  **Create the hook file:**
    ```bash
    touch .git/hooks/prepare-commit-msg
    chmod +x .git/hooks/prepare-commit-msg
    ```
2.  **Add the following content to the file.** Make sure `ai_commit` is in your `PATH`.
    ```bash
    #!/bin/sh
    
    COMMIT_MSG_FILE=$1
    COMMIT_SOURCE=$2
    
    # Only run if the commit is not a merge, squash, or fixup
    if [ -z "$COMMIT_SOURCE" ]; then
      ai_commit -m >"$COMMIT_MSG_FILE"
    fi
    ```

Now, `git commit` will automatically open your editor with a generated message, ready for you to approve.
