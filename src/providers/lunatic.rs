use super::Provider;
use crate::nixpacks::{
    app::App,
    environment::Environment,
    plan::{
        phase::{Phase, StartPhase},
        BuildPlan,
    },
};
use crate::providers::rust::RustProvider;
use anyhow::{Result};

pub struct LunaticProvider {}

impl Provider for LunaticProvider {
    fn name(&self) -> &str {
        "lunatic"
    }

    fn detect(&self, app: &App, _env: &Environment) -> Result<bool> {
        if !app.includes_file("Cargo.toml") {
            return Ok(false)
        }

        Ok(false)
        
    }

    fn get_build_plan(&self, app: &App, env: &Environment) -> Result<Option<BuildPlan>> {
        let setup = LunaticProvider::get_setup(app, env)?;
        let build = LunaticProvider::get_build(app, env)?;
        let start = LunaticProvider::get_start(app, env)?;

        let plan = BuildPlan::new(&vec![setup, build], start);

        Ok(Some(plan))
    }
}

impl LunaticProvider {
    fn get_setup(app: &App, env: &Environment) -> Result<Phase> {
        RustProvider::get_setup(app, env)
    }

    fn get_build(app: &App, env: &Environment) -> Result<Phase> {
        RustProvider::get_build(app, env)
    }

    fn get_start(app: &App, env: &Environment) -> Result<Option<StartPhase>> {
        RustProvider::get_start(app, env)
        
    }
}

#[cfg(test)]
mod test {
    use super::*;

}
