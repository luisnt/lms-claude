use anyhow::{Context, Result};
use std::process::Command;

pub fn launch_claude(model: &str) -> Result<()> {
    let mut cmd = Command::new("claude");
    cmd.arg("--model").arg(model);

    // Configuração da API local
    cmd.env("ANTHROPIC_BASE_URL", "http://localhost:1234");
    cmd.env("ANTHROPIC_AUTH_TOKEN", "lmstudio");
    cmd.env("ANTHROPIC_API_KEY", "");
    cmd.env("CLAUDE_CODE_ATTRIBUTION_HEADER", "0");
    cmd.env("CLAUDE_CODE_MAX_CONTEXT_TOKENS", "131072");

    println!("\nIniciando Claude Code com o modelo: {}", model);
    println!("Endpoint: http://localhost:1234\n");

    let status = cmd.status().context("Falha ao executar o Claude Code")?;

    if !status.success() {
        anyhow::bail!("Claude Code encerrou com erro: {:?}", status.code());
    }

    Ok(())
}
