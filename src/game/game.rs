/***
Copyright (c) 2022 Juan Medina

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
***/

use bevy::{
    app::AppExit,
    log::LogPlugin,
    prelude::*,
    render::camera::ScalingMode,
    text::TextSettings,
    window::{WindowResizeConstraints, WindowResized},
    winit::WinitSettings,
};
use bevy_pkv::PkvStore;

use super::{events, Config, DisplayMode, State};
use crate::scenes;

const TITLE: &str = "Dice Master!";
const LOG_FILTER: &str = "wgpu=error,dice_master=debug";
const DESIGN_RESOLUTION: Vec2 = Vec2::new(1920., 1080.);
const CLEAR_COLOR: Color = Color::rgb(0., 0., 0.);
const COMPANY: &str = "NewOlds";
const APP_NAME: &str = "dice_master";
const CONFIG_KEY: &str = "game_config";

pub struct Game {
    app: App,
}

impl Game {
    pub fn new() -> Self {
        Self { app: App::new() }
    }

    pub fn run(&mut self) {
        let (store, config) = self.get_config();
        self.default_plugins(config);
        self.insert_plugins();
        self.insert_resources(store, config);
        self.add_main_systems();
        self.set_scenes();

        self.app.run();
    }

    fn get_config(&self) -> (PkvStore, Config) {
        let store = PkvStore::new(COMPANY, APP_NAME);
        let config = if let Ok(config) = store.get::<Config>(CONFIG_KEY) {
            config
        } else {
            Config::default()
        };
        (store, config)
    }

    fn set_scenes(&mut self) {
        self.app
            .add_state(State::Splash)
            .add_plugin(scenes::Hello)
            .add_plugin(scenes::Menu)
            .add_plugin(scenes::Splash);
    }

    fn insert_resources(&mut self, store: PkvStore, config: Config) {
        self.app
            .insert_resource(ClearColor(CLEAR_COLOR))
            .insert_resource(WinitSettings::desktop_app())
            .insert_resource(TextSettings {
                allow_dynamic_font_size: true,
                ..default()
            })
            .insert_resource(config)
            .insert_resource(store);
    }

    fn add_main_systems(&mut self) {
        self.app
            .add_startup_system(setup)
            .add_system(scale_ui)
            .add_system(toggle_full_screen_on_alt_enter)
            .add_system(bevy::window::close_on_esc)
            .add_system_to_stage(CoreStage::PostUpdate, app_exit);
    }

    fn default_plugins(&mut self, config: Config) -> &mut App {
        self.app.add_plugins(
            DefaultPlugins
                .set(self.setup_window(config))
                .set(self.setup_log()),
        )
    }

    fn setup_window(&self, config: Config) -> WindowPlugin {
        let window = if config.mode == DisplayMode::FullScreen {
            WindowDescriptor {
                title: TITLE.into(),
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }
        } else {
            WindowDescriptor {
                title: TITLE.into(),
                width: DESIGN_RESOLUTION.x,
                height: DESIGN_RESOLUTION.y,
                resize_constraints: WindowResizeConstraints {
                    min_width: DESIGN_RESOLUTION.x / 2.,
                    min_height: DESIGN_RESOLUTION.y / 2.,
                    ..default()
                },
                ..default()
            }
        };

        WindowPlugin {
            window,
            ..default()
        }
    }

    fn setup_log(&self) -> LogPlugin {
        LogPlugin {
            filter: LOG_FILTER.into(),
            level: bevy::log::Level::INFO,
        }
    }

    fn insert_plugins(&mut self) {
        self.app.add_plugin(events::Handler);
    }
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedHorizontal(DESIGN_RESOLUTION.x);
    commands.spawn(camera_bundle);
}

fn scale_ui(resize_event: Res<Events<WindowResized>>, mut ui_scale: ResMut<UiScale>) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        let scale_x = e.width / DESIGN_RESOLUTION.x;
        let scale_y = e.height / DESIGN_RESOLUTION.y;
        let scale = (scale_x * scale_y).sqrt();
        ui_scale.scale = scale as f64;
    }
}

fn toggle_full_screen_on_alt_enter(
    input: Res<Input<KeyCode>>,
    config: Res<Config>,
    mut ev_change_display_mode: EventWriter<events::ChangeDisplayMode>,
) {
    if (input.pressed(KeyCode::LAlt) || input.pressed(KeyCode::RAlt))
        && input.just_pressed(KeyCode::Return)
    {
        ev_change_display_mode.send(events::ChangeDisplayMode::to(!config.mode.clone()));
    }
}

fn app_exit(
    mut events: EventReader<AppExit>,
    config: Res<Config>,
    mut config_store: ResMut<PkvStore>,
) {
    for _ in events.iter() {
        debug!("config {:?}", config);
        config_store
            .set(CONFIG_KEY, config.as_ref())
            .expect("config can't no be saved");
    }
}
