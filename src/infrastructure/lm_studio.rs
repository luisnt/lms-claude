use anyhow::{Context, Result};
use serde::Deserialize;
use std::process::{Command, Stdio};

#[derive(Debug, Deserialize)]
pub struct ModelInfo {
    #[serde(rename = "modelKey")]
    pub id: String,
}

pub fn list_models() -> Result<Vec<ModelInfo>> {
    let output = Command::new("lms")
        .args(["ls", "--llm", "--json"])
        .output()
        .context("Falha ao executar lms ls. O LM Studio está no PATH?")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("lms ls falhou"));
    }

    let json_str = String::from_utf8(output.stdout)?;
    let models: Vec<ModelInfo> =
        serde_json::from_str(&json_str).context("Falha ao parsear o JSON do LM Studio")?;

    Ok(models)
}

pub fn start_server() -> Result<()> {
    // Stdio::null() descarta a saída "Success!" para não sujar a TUI
    Command::new("lms")
        .args(["server", "start", "--port", "1234"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .context("Falha ao iniciar o servidor LM Studio")?;

    // Pequena pausa de segurança para garantir que o processo iniciou sem imprimir nada no terminal
    std::thread::sleep(std::time::Duration::from_millis(200));
    Ok(())
}
