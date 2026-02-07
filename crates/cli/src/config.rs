use anyhow::{anyhow, bail, Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::ModelTarget;
use crate::util::now_epoch_seconds;

pub const DEFAULT_TRANSLATE_MODEL: &str = "morph/morph-v3-large:nitro";
pub const DEFAULT_CORRECT_MODEL: &str = "morph/morph-v3-large:nitro";
pub const DEFAULT_ASK_MODEL: &str = "morph/morph-v3-large:nitro";

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub auth: Option<AuthConfig>,
    pub models: ModelsConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfig {
    pub token: String,
    pub expires_at: u64,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ModelsConfig {
    pub ask: Option<String>,
    pub translate: Option<String>,
    pub correct: Option<String>,
}

pub fn load_config() -> Result<Config> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(Config::default());
    }
    let raw = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    let config = serde_json::from_str(&raw).context("invalid config json")?;
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    let path = config_path()?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).with_context(|| format!("create {}", dir.display()))?;
    }
    let raw = serde_json::to_string_pretty(config).context("encode config")?;
    fs::write(&path, raw).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

pub fn auth_token() -> Result<String> {
    let config = load_config()?;
    let auth = config
        .auth
        .ok_or_else(|| anyhow!("not logged in; run `cealum login`"))?;
    let now = now_epoch_seconds();
    if now >= auth.expires_at {
        bail!("session expired; run `cealum login` again");
    }
    Ok(auth.token)
}

pub fn model_for(target: ModelTarget) -> Result<String> {
    let config = load_config()?;
    let model = match target {
        ModelTarget::Ask => config
            .models
            .correct
            .unwrap_or_else(|| DEFAULT_ASK_MODEL.to_string()),
        ModelTarget::Translate => config
            .models
            .translate
            .unwrap_or_else(|| DEFAULT_TRANSLATE_MODEL.to_string()),
        ModelTarget::Correct => config
            .models
            .correct
            .unwrap_or_else(|| DEFAULT_CORRECT_MODEL.to_string()),
    };
    Ok(model)
}

fn config_path() -> Result<PathBuf> {
    let proj = ProjectDirs::from("dev", "cealum", "cealum")
        .ok_or_else(|| anyhow!("unable to resolve config directory"))?;
    Ok(Path::new(proj.config_dir()).join("config.json"))
}
