# lms-claude

🇺🇸 **English** · [🇧🇷 Português](README.pt-BR.md)

A native Rust terminal user interface (TUI) for launching Claude Code with a model served locally by LM Studio. It discovers available models, lets you select one, starts the LM Studio server, and launches the Claude Code CLI with an Anthropic-compatible local endpoint.

## How it works

1. The launcher reads available language models from the LM Studio CLI (`lms`).
2. You navigate the model list and select the model to use.
3. The selected model is saved in the operating system's configuration directory.
4. On launch, the application starts the LM Studio server on port `1234` and runs `claude --model <model>` with local endpoint environment variables.

The model passed with `--model` takes precedence over the saved selection. When no model is provided or saved, the current default is `openai/gpt-oss-20b`.

## Requirements

- Rust stable (the repository pins the `stable` toolchain and includes `rustfmt` and `clippy`).
- LM Studio initialized at least once, with the `lms` CLI available on `PATH`.
- Claude Code CLI available as `claude` on `PATH`.
- An LM Studio local server accessible at `http://localhost:1234`.

## Build and run

Build and launch in release mode with Cargo:

```sh
cargo run --release
```

The root `package.json` also defines an npm script for this same command. This can be useful when a VS Code extension or task runner invokes the project's npm scripts:

```sh
npm run run
```

`package.json` is only a script entry point; the application itself is implemented in Rust and requires the Rust toolchain. The repository does not include a VS Code extension manifest or extension-specific commands.

To build an executable without launching it:

```sh
cargo build --release
```

## Command-line options

```sh
cargo run --release -- --help
cargo run --release -- --model openai/gpt-oss-20b
cargo run --release -- --list
cargo run --release -- --check
```

| Option | Behavior |
| --- | --- |
| `--model <MODEL>` | Start the interface with the specified model. |
| `--list` | Print model IDs returned by `lms ls --llm --json`. |
| `--check` | Print an environment-check message and exit. |

In the TUI, use the arrow keys to move, **Space** to save the active model, **Enter** to launch Claude Code, and **Esc** to exit.

## Configuration and local endpoint

The selected model is stored as JSON at `lms-claude/config.json` inside the platform-specific configuration directory returned by the Rust `dirs` library. The launcher sets the following variables for the spawned Claude Code process:

- `ANTHROPIC_BASE_URL=http://localhost:1234`
- `ANTHROPIC_AUTH_TOKEN=lmstudio`
- `ANTHROPIC_API_KEY` to an empty value
- `CLAUDE_CODE_ATTRIBUTION_HEADER=0`
- `CLAUDE_CODE_MAX_CONTEXT_TOKENS=131072`

The endpoint uses the local loopback interface. The launcher starts the LM Studio server with `lms server start --port 1234`; it does not stop the server when the TUI exits.

## Project structure

```text
src/
├── infrastructure/
│   ├── claude_code.rs   # Claude Code process and local endpoint environment
│   ├── config.rs        # Saved model selection
│   └── lm_studio.rs     # LM Studio CLI and model discovery
├── ui/
│   ├── components/      # Reusable terminal UI components
│   ├── screens/         # Model-selection dashboard
│   └── theme.rs         # Terminal color theme
└── main.rs              # CLI arguments and application flow
```

## Development and CI

The repository's GitHub Actions workflow checks formatting, compilation, tests, Clippy, and release builds on Windows x64, Linux x64, macOS Intel, and Apple Silicon runners. Run the same checks locally with:

```sh
cargo fmt --all -- --check
cargo check
cargo test
cargo clippy --all-targets -- -D warnings
cargo build --release
```

Cross-compilation requires the appropriate target toolchain and platform linker/SDK; installing a Rust target alone does not provide those system components.

## Author

Luis Caldas  
Senior Software Development Engineer

Software solutions development, systems architecture, and automation.

Portfolio: [github.com/luisnt](https://github.com/luisnt) · Explore [more projects](https://github.com/luisnt?tab=repositories) from my portfolio.
