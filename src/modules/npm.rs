use ansi_term::Color;
use std::process::Command;
use url::Url;

use super::{Context, Module};

/// Creates a module with the current NPM registry URL
///
/// Will display the NPM registry URL if all of the following criteria are met:
///     - A .nodenv-vars file exists in the current directory or any parent
///     - The `nodenv` command is available and it has a definition for `npm_config_registry`
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    // TODO: this should also check parent directories
    let is_npm_project = context
        .try_begin_scan()?
        .set_files(&[".nodenv-vars"])
        .is_match();

    if !is_npm_project {
        return None;
    }

    match get_npm_url() {
        Some(npm_url) => {
            const NODE_CHAR: &str = "⬢ ";

            let mut module = context.new_module("npm");
            let module_style = module
                .config_value_style("style")
                .unwrap_or_else(|| Color::Green.bold());
            module.set_style(module_style);

            let url = npm_url.trim();
            module.new_segment("symbol", NODE_CHAR);
            module.new_segment("version", url);

            Some(module)
        }
        None => None,
    }
}

fn get_npm_url() -> Option<String> {
    match Command::new("nodenv").arg("vars").output() {
        Ok(output) => {
            let text = String::from_utf8(output.stdout).unwrap();
            for line in text.lines() {
                if line.starts_with("export npm_config_registry=") {
                    let npm_url_str = line.rsplitn(2, "=").next().unwrap();
                    let clean_url_str = npm_url_str.trim_matches(&['\''] as &[_]);
                    return Url::parse(clean_url_str).unwrap().host_str().map(String::from);
                }
            }
            return None;
        }
        Err(_) => None,
    }
}
