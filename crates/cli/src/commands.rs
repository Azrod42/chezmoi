use anyhow::{anyhow, bail, Result};

use crate::api::{self, Function};
use crate::cli::{
    Commands, ConfigArgs, ConfigCommand, ConfigSetArgs, CorrectArgs, LoginArgs, ModelTarget,
    RegisterArgs, TranslateArgs,
};
use crate::config::{self, AuthConfig, DEFAULT_CORRECT_MODEL, DEFAULT_TRANSLATE_MODEL};
use crate::output::print_answer;
use crate::ui::prompt;
use crate::util::now_epoch_seconds;

pub async fn run(command: Commands) -> Result<()> {
    match command {
        Commands::Login(args) => login(args).await,
        Commands::Register(args) => register(args).await,
        Commands::Ask(args) => ask(args).await,
        Commands::Translate(args) => translate(args).await,
        Commands::Correct(args) => correct(args).await,
        Commands::Config(args) => config(args),
    }
}

async fn login(args: LoginArgs) -> Result<()> {
    let email = match args.email {
        Some(email) => email,
        None => prompt("Email: ")?,
    };
    let password = match args.password {
        Some(password) => password,
        None => rpassword::prompt_password("Password: ")?,
    };

    let payload = api::login_request(&email, &password).await?;
    if payload.token_type.to_lowercase() != "bearer" {
        bail!("unexpected token type: {}", payload.token_type);
    }

    let expires_at = now_epoch_seconds()
        .checked_add(payload.expires_in)
        .ok_or_else(|| anyhow!("token expiry overflow"))?;

    let mut config = config::load_config()?;
    config.auth = Some(AuthConfig {
        token: payload.token,
        expires_at,
    });
    config::save_config(&config)?;

    println!("Login saved. Expires in {} seconds.", payload.expires_in);
    Ok(())
}

async fn register(args: RegisterArgs) -> Result<()> {
    let email = match args.email {
        Some(email) => email,
        None => prompt("Email: ")?,
    };
    let password = match args.password {
        Some(password) => password,
        None => rpassword::prompt_password("Password: ")?,
    };

    let payload = api::register_request(&email, &password).await?;
    println!("Registered {} ({})", payload.email, payload.id);
    Ok(())
}

async fn ask(args: TranslateArgs) -> Result<()> {
    let prompt = args.text.join(" ");
    let model = config::model_for(ModelTarget::Ask)?;
    let token = config::auth_token()?;
    let response =
        api::ai_request(Function::TRANSLATE, &args.language, &prompt, &model, &token).await?;
    print_answer(&response.answer, args.format);
    Ok(())
}

async fn translate(args: TranslateArgs) -> Result<()> {
    let prompt = args.text.join(" ");
    let model = config::model_for(ModelTarget::Translate)?;
    let token = config::auth_token()?;
    let response =
        api::ai_request(Function::TRANSLATE, &args.language, &prompt, &model, &token).await?;
    print_answer(&response.answer, args.format);
    Ok(())
}

async fn correct(args: CorrectArgs) -> Result<()> {
    let prompt = args.text.join(" ");
    let model = config::model_for(ModelTarget::Correct)?;
    let token = config::auth_token()?;
    let response =
        api::ai_request(Function::CORRECT, &args.language, &prompt, &model, &token).await?;
    print_answer(&response.answer, args.format);
    Ok(())
}

fn config(args: ConfigArgs) -> Result<()> {
    match args.command {
        ConfigCommand::Set(args) => config_set(args),
        ConfigCommand::Show => config_show(),
    }
}

fn config_set(args: ConfigSetArgs) -> Result<()> {
    let mut config = config::load_config()?;
    match args.target {
        ModelTarget::Translate => config.models.translate = Some(args.model),
        ModelTarget::Correct => config.models.correct = Some(args.model),
        ModelTarget::Ask => config.models.ask = Some(args.model),
    }
    config::save_config(&config)?;
    println!("Model updated.");
    Ok(())
}

fn config_show() -> Result<()> {
    let config = config::load_config()?;
    let translate = config
        .models
        .translate
        .as_deref()
        .unwrap_or(DEFAULT_TRANSLATE_MODEL);
    let correct = config
        .models
        .correct
        .as_deref()
        .unwrap_or(DEFAULT_CORRECT_MODEL);
    println!("translate: {translate}");
    println!("correct: {correct}");
    Ok(())
}
