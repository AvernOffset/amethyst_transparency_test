extern crate amethyst;

use amethyst::{
    prelude::*,
    ecs::prelude::{WriteStorage, System, Read, Join},
    core::transform::{Transform, TransformBundle},
    renderer::{SpriteSheet, SpriteRender, SpriteSheetFormat, Texture, SpriteSheetHandle, PngFormat, TextureMetadata, DisplayConfig, DrawFlat2D, Pipeline,  RenderBundle, Stage, Camera, Projection, ColorMask, ALPHA, DepthMode},
    utils::application_root_dir,
    assets::{Loader, AssetStorage},
    input::{InputHandler, InputBundle},
};

struct Example;

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet("texture/sprites.png", "texture/sprites_def.ron", world);

        initialize_sprites(world, sprite_sheet_handle);
        initialize_camera(world);
    }
}

fn load_sprite_sheet(filename: &str, sprite_def: &str, world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            filename,
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        sprite_def,
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}

const ARENA_WIDTH: f32 = 1280.0;
const ARENA_HEIGHT: f32 = 720.0;

const BOTTOM: f32 = -15.0;
const MIDDLE: f32 = -10.0;
const TOP: f32 = -5.0;

const Y_TOP: f32 = ARENA_HEIGHT - 32.;
const Y_MID: f32 = ARENA_HEIGHT/2.0;
const Y_BOT: f32 = 32.;

fn initialize_sprites(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let checkerboard = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };
    let mut smiley = checkerboard.clone();
    smiley.sprite_number = 1;
    let mut hex = checkerboard.clone();
    hex.sprite_number = 2;

    let mut check_center = Transform::default();
    check_center.set_xyz(ARENA_WIDTH/2.0, ARENA_HEIGHT/2.0+1., BOTTOM);
    check_center.set_scale(3.0, 3.0, 1.0);

    let mut smiley_center = Transform::default();
    smiley_center.set_xyz(ARENA_WIDTH/2.0-20., ARENA_HEIGHT/2.0+1., MIDDLE);
    smiley_center.set_scale(3.0, 3.0, 1.0);

    let mut hex_center = Transform::default();
    hex_center.set_xyz(ARENA_WIDTH/2.0+20., ARENA_HEIGHT/2.0+1., TOP);
    hex_center.set_scale(3.0, 3.0, 1.0);

    let mut check_side = Transform::default();
    check_side.set_xyz(32., Y_BOT, 0.0);
    check_side.set_scale(3.0, 3.0, 1.0);

    let mut smiley_side = Transform::default();
    smiley_side.set_xyz(32., Y_MID, 0.0);
    smiley_side.set_scale(3.0, 3.0, 1.0);

    let mut hex_side = Transform::default();
    hex_side.set_xyz(32., Y_TOP, 0.0);
    hex_side.set_scale(3.0, 3.0, 1.0);

    world.create_entity()
        .with(checkerboard.clone())
        .with(check_center)
        .build();

    world.create_entity()
        .with(smiley.clone())
        .with(smiley_center)
        .build();

    world.create_entity()
        .with(hex.clone())
        .with(hex_center)
        .build();

    world.create_entity()
        .with(checkerboard.clone())
        .with(check_side)
        .build();

    world.create_entity()
        .with(smiley.clone())
        .with(smiley_side)
        .build();

    world.create_entity()
        .with(hex.clone())
        .with(hex_side)
        .build();

}



fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world.create_entity()
    .with(Camera::from(Projection::orthographic(
        0.0,
        ARENA_WIDTH,
        0.0,
        ARENA_HEIGHT,
    )))
    .with(transform)
    .build();
}

struct SwapLayerSystem {
    inactive: u32,
}

impl SwapLayerSystem {
    fn new() -> Self {
        SwapLayerSystem {
            inactive: 0,
        }
    }
}

impl<'s> System<'s> for SwapLayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, input): Self::SystemData) {
        if self.inactive > 0 {
            self.inactive -= 1;
            return;
        }
        if let Some(true) = input.action_is_down("swap_top_mid") {
            println!("Swapping top and mid");
            for transform in (&mut transforms).join() {
                let translation = transform.translation();
                if translation.y == Y_TOP || translation.z == TOP {
                    if translation.x == 32. {
                        transform.set_y(Y_MID);
                    }
                    else {
                        println!("Moving top layer to middle layer");
                        transform.set_z(MIDDLE);
                    }
                }
                else if translation.y == Y_MID || translation.z == MIDDLE {
                    if translation.x == 32. {
                        transform.set_y(Y_TOP);
                    }
                    else {
                        println!("moving middle layer to top layer");
                        transform.set_z(TOP);
                    }
                }
            }
            self.inactive = 40;
        }
        if let Some(true) = input.action_is_down("swap_mid_bot") {
            println!("Swapping mid and bottom");
            for transform in (&mut transforms).join() {
                let translation = transform.translation();
                if translation.y == Y_BOT || translation.z == BOTTOM {
                    if translation.x == 32. {
                        transform.set_y(Y_MID);
                    }
                    else {
                        println!("Moving bottom layer to middle layer");
                        transform.set_z(MIDDLE);
                    }
                }
                else if translation.y == Y_MID || translation.z == MIDDLE {
                    if translation.x == 32. {
                        transform.set_y(Y_BOT);
                    }
                    else {
                        println!("Moving middle layer to bottom layer.");
                        transform.set_z(BOTTOM);
                    }
                }
            }
            self.inactive = 40;
        }
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let path = app_root.join("resources/display_config.ron");
    let config = DisplayConfig::load(&path);

    let bindings_path = app_root.join("resources/bindings_config.ron");
    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(bindings_path)?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.1, 0.1, 0.1, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()
                .with_transparency(ColorMask::all(), ALPHA, Some(DepthMode::LessEqualWrite))),
    );

    let game_data =
        GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config))
            .with_sprite_sheet_processor()
            .with_sprite_visibility_sorting(&["transform_system"])
        )?
        .with_bundle(input_bundle)?
        .with(SwapLayerSystem::new(), "swap_layer_system", &["input_system"]);
    let mut game = Application::new("./", Example, game_data)?;

    game.run();

    Ok(())
}
