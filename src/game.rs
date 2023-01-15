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

use crate::scenes::{self, State};

use bevy::{
    log::LogPlugin,
    prelude::*,
    text::TextSettings,
    window::{WindowResizeConstraints, WindowResized},
    winit::WinitSettings,
};

const TITLE: &str = "Dice Master!";
const LOG_FILTER: &str = "wgpu=error,dice_master=debug";
const DESIGN_RESOLUTION: Vec2 = Vec2::new(1920., 1080.);

pub fn run() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: TITLE.into(),
                        width: DESIGN_RESOLUTION.x,
                        height: DESIGN_RESOLUTION.y,
                        resize_constraints: WindowResizeConstraints {
                            min_width: DESIGN_RESOLUTION.x / 2.,
                            min_height: DESIGN_RESOLUTION.y / 2.,
                            ..default()
                        },
                        ..Default::default()
                    },
                    ..default()
                })
                .set(LogPlugin {
                    filter: LOG_FILTER.into(),
                    level: bevy::log::Level::INFO,
                }),
        )
        .insert_resource(TextSettings {
            allow_dynamic_font_size: true,
            ..default()
        })
        .insert_resource(WinitSettings::desktop_app())
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(setup)
        .add_system(scale_ui)
        .add_system(toggle_full_screen_on_alt_enter)
        .add_state(State::Hello)
        .add_plugin(scenes::Hello)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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

fn toggle_full_screen_on_alt_enter(input: Res<Input<KeyCode>>, windows: ResMut<Windows>) {
    if (input.pressed(KeyCode::LAlt) || input.pressed(KeyCode::RAlt))
        && input.just_pressed(KeyCode::Return)
    {
        toggle_full_screen(windows);
    }
}

fn toggle_full_screen(mut windows: ResMut<Windows>) {
    let window = windows
        .get_primary_mut()
        .expect("we should have a primary window");
    if window.mode() == WindowMode::Windowed {
        window.set_mode(WindowMode::BorderlessFullscreen);
    } else {
        window.set_mode(WindowMode::Windowed);
    }
}
