use anyhow::Result;

use crate::cdp::{self, CdpConnection};

pub async fn cmd_eval(port: u16, script: &str, json: bool, tab_id: Option<&str>) -> Result<()> {
    let mut cdp = match tab_id {
        Some(id) => cdp::connect_by_tab_id(port, id).await?,
        None => cdp::connect_active(port).await?,
    };
    let result = cdp.eval(script).await?;
    if json {
        println!("{}", serde_json::to_string(&result)?);
    } else {
        println!("{}", serde_json::to_string_pretty(&result)?);
    }
    Ok(())
}

pub async fn cmd_get(
    port: u16,
    what: &crate::GetCommand,
    json: bool,
    tab_id: Option<&str>,
) -> Result<()> {
    let targets = cdp::get_targets(port).await?;
    let target = match tab_id {
        Some(id) => cdp::find_target_by_id(&targets, id)?,
        None => cdp::find_active_target(&targets)?,
    };

    match what {
        crate::GetCommand::Title => print_field(json, "title", &target.title),
        crate::GetCommand::Url => print_field(json, "url", &target.url),
        crate::GetCommand::Text { selector } => {
            let ws = target.webSocketDebuggerUrl.as_ref().unwrap();
            eval_and_print_str(ws, &build_text_script(selector)?).await?;
        }
        crate::GetCommand::Html { selector } => {
            let ws = target.webSocketDebuggerUrl.as_ref().unwrap();
            let script = format!(
                "document.querySelector({})?.innerHTML || ''",
                serde_json::to_string(selector)?
            );
            eval_and_print_str(ws, &script).await?;
        }
        crate::GetCommand::Value { selector } => {
            let ws = target.webSocketDebuggerUrl.as_ref().unwrap();
            let script = format!(
                "document.querySelector({})?.value || ''",
                serde_json::to_string(selector)?
            );
            eval_and_print_str(ws, &script).await?;
        }
        crate::GetCommand::Attr { selector, name } => {
            let ws = target.webSocketDebuggerUrl.as_ref().unwrap();
            let script = format!(
                "document.querySelector({})?.getAttribute({}) || ''",
                serde_json::to_string(selector)?,
                serde_json::to_string(name)?
            );
            eval_and_print_str(ws, &script).await?;
        }
        crate::GetCommand::Count { selector } => {
            let ws = target.webSocketDebuggerUrl.as_ref().unwrap();
            let script = format!(
                "document.querySelectorAll({}).length",
                serde_json::to_string(selector)?
            );
            let result = CdpConnection::connect(ws).await?.eval(&script).await?;
            println!("{}", result);
        }
    }
    Ok(())
}

fn build_text_script(selector: &Option<String>) -> Result<String> {
    Ok(match selector {
        Some(sel) => format!(
            "document.querySelector({})?.innerText || ''",
            serde_json::to_string(sel)?
        ),
        None => "document.body.innerText".to_string(),
    })
}

async fn eval_and_print_str(ws_url: &str, script: &str) -> Result<()> {
    let mut cdp = CdpConnection::connect(ws_url).await?;
    print_eval_str(&mut cdp, script).await
}

fn print_field(json: bool, key: &str, value: &str) {
    if json {
        println!("{}", serde_json::json!({ key: value }));
    } else {
        println!("{}", value);
    }
}

async fn print_eval_str(cdp: &mut CdpConnection, script: &str) -> Result<()> {
    let result = cdp.eval(script).await?;
    if let Some(text) = result.as_str() {
        println!("{}", text);
    }
    Ok(())
}

pub async fn cmd_tabs(port: u16, action: &crate::TabsCommand, json: bool) -> Result<()> {
    let targets = cdp::get_targets(port).await?;

    match action {
        crate::TabsCommand::List => {
            if json {
                let tabs: Vec<_> = targets
                    .iter()
                    .map(|t| serde_json::json!({ "title": t.title, "url": t.url, "id": t.id }))
                    .collect();
                println!("{}", serde_json::to_string_pretty(&tabs)?);
            } else {
                for (i, target) in targets.iter().enumerate() {
                    println!("{} [{}]: {} - {}", i, target.id, target.title, target.url);
                }
            }
        }
    }
    Ok(())
}
