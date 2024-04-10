use geng::prelude::*;

#[derive(geng::asset::Load)]
pub struct Assets {
    pub zombie: Rc<ugli::Texture>,
    pub aaah: Rc<geng::Sound>,
}

impl Assets {
    pub async fn load(manager: &geng::asset::Manager) -> anyhow::Result<Self> {
        geng::asset::Load::load(manager, &run_dir().join("assets"), &())
            .await
            .context("failed to load assets")
    }
}
