# lms-claude

[🇺🇸 English](README.md) · 🇧🇷 **Português**

Uma interface de terminal (TUI) nativa em Rust para iniciar o Claude Code com um modelo servido localmente pelo LM Studio. O aplicativo identifica os modelos disponíveis, permite selecionar um deles, inicia o servidor do LM Studio e executa a CLI do Claude Code com um endpoint local compatível com a API da Anthropic.

## Como funciona

1. O launcher consulta os modelos de linguagem disponíveis pela CLI do LM Studio (`lms`).
2. Você navega pela lista e seleciona o modelo que deseja usar.
3. O modelo selecionado é salvo no diretório de configuração do sistema operacional.
4. Ao iniciar, o aplicativo executa o servidor do LM Studio na porta `1234` e inicia `claude --model <modelo>` com variáveis de ambiente apontadas para o endpoint local.

O modelo informado com `--model` tem precedência sobre a seleção salva. Quando nenhum modelo é informado ou salvo, o padrão atual é `openai/gpt-oss-20b`.

## Requisitos

- Rust stable (o repositório fixa a toolchain `stable` e inclui `rustfmt` e `clippy`).
- LM Studio inicializado pelo menos uma vez, com a CLI `lms` disponível no `PATH`.
- CLI do Claude Code disponível como `claude` no `PATH`.
- Servidor local do LM Studio acessível em `http://localhost:1234`.

## Compilação e execução

Compile e inicie em modo release com Cargo:

```sh
cargo run --release
```

O `package.json` na raiz também define um script npm para executar o mesmo comando. Isso pode ser útil quando uma extensão do VS Code ou um executor de tarefas chama os scripts npm do projeto:

```sh
npm run run
```

O `package.json` é apenas um ponto de entrada para scripts; o aplicativo é implementado em Rust e requer a toolchain Rust. O repositório não inclui manifesto de extensão do VS Code nem comandos específicos de uma extensão.

Para gerar o executável sem iniciá-lo:

```sh
cargo build --release
```

## Opções de linha de comando

```sh
cargo run --release -- --help
cargo run --release -- --model openai/gpt-oss-20b
cargo run --release -- --list
cargo run --release -- --check
```

| Opção | Comportamento |
| --- | --- |
| `--model <MODELO>` | Inicia a interface com o modelo informado. |
| `--list` | Exibe os IDs retornados por `lms ls --llm --json`. |
| `--check` | Exibe uma mensagem de verificação do ambiente e encerra. |

Na TUI, use as setas para navegar, **Espaço** para salvar o modelo ativo, **Enter** para iniciar o Claude Code e **Esc** para sair.

## Configuração e endpoint local

O modelo selecionado é armazenado em JSON no caminho `lms-claude/config.json`, dentro do diretório de configuração da plataforma identificado pela biblioteca Rust `dirs`. O launcher define as variáveis a seguir para o processo do Claude Code iniciado por ele:

- `ANTHROPIC_BASE_URL=http://localhost:1234`
- `ANTHROPIC_AUTH_TOKEN=lmstudio`
- `ANTHROPIC_API_KEY` com valor vazio
- `CLAUDE_CODE_ATTRIBUTION_HEADER=0`
- `CLAUDE_CODE_MAX_CONTEXT_TOKENS=131072`

O endpoint usa a interface local de loopback. O launcher inicia o servidor do LM Studio com `lms server start --port 1234`; ele não encerra o servidor quando a TUI sai.

## Estrutura do projeto

```text
src/
├── infrastructure/
│   ├── claude_code.rs   # Processo do Claude Code e ambiente do endpoint local
│   ├── config.rs        # Modelo selecionado salvo
│   └── lm_studio.rs     # CLI do LM Studio e descoberta de modelos
├── ui/
│   ├── components/      # Componentes reutilizáveis da interface de terminal
│   ├── screens/         # Painel de seleção de modelo
│   └── theme.rs         # Tema de cores do terminal
└── main.rs              # Argumentos da CLI e fluxo do aplicativo
```

## Desenvolvimento e CI

O workflow do GitHub Actions verifica formatação, compilação, testes, Clippy e builds de release em runners Windows x64, Linux x64, macOS Intel e Apple Silicon. Execute as mesmas verificações localmente com:

```sh
cargo fmt --all -- --check
cargo check
cargo test
cargo clippy --all-targets -- -D warnings
cargo build --release
```

A compilação cruzada requer a toolchain de destino e o linker/SDK da plataforma apropriados; instalar somente o target Rust não fornece esses componentes de sistema.

## Autor

Luis Caldas  
Senior Software Development Engineer

Desenvolvimento de soluções de software, arquitetura de sistemas e automação.

Portfólio: [github.com/luisnt](https://github.com/luisnt) · Conheça [outros projetos](https://github.com/luisnt?tab=repositories) do meu portfólio.
