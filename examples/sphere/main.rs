//! Displays a shaded sphere to the user.

use amethyst::{
    assets::{PrefabLoader, PrefabLoaderSystem, RonFormat},
    core::transform::TransformBundle,
    prelude::*,
    renderer::{DrawShaded, PosNormTex},
    utils::{application_root_dir, scene::BasicScenePrefab},
};

type MyPrefabData = BasicScenePrefab<Vec<PosNormTex>, f32>;

struct Example;

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // Initialise the scene with an object, a light and a camera.
        let handle = data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("prefab/sphere.ron", RonFormat, (), ())
        });
        data.world.create_entity().with(handle).build();
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("examples/sphere/resources/display_config.ron");

    let resources = app_root.join("examples/assets/");

    let game_data = GameDataBuilder::default()
        .with(PrefabLoaderSystem::<MyPrefabData>::default(), "", &[])
        .with_bundle(TransformBundle::new())?
        .with_basic_renderer(display_config_path, DrawShaded::<PosNormTex>::new(), false)?;
    let mut game = Application::new(resources, Example, game_data)?;
    game.run();
    Ok(())
}
