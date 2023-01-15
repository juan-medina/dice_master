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

use super::clear_scene;
use crate::game::State;
use bevy::prelude::*;

pub struct Splash;

impl Plugin for Splash {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(State::Splash).with_system(setup))
            .add_system_set(SystemSet::on_update(State::Splash).with_system(countdown))
            .add_system_set(
                SystemSet::on_exit(State::Splash).with_system(clear_scene::<OnSplashScene>),
            );
    }
}

#[derive(Component)]
struct OnSplashScene;

const FONT_NAME: &str = "fonts/FiraSans-Bold.ttf";
const FONT_SIZE: f32 = 80.0;
const FONT_COLOR: Color = Color::WHITE;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnSplashScene,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Splash Screen",
                    TextStyle {
                        font: asset_server.load(FONT_NAME),
                        font_size: FONT_SIZE,
                        color: FONT_COLOR,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );
        });
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

use bevy::prelude::State as BevyState;

fn countdown(
    mut game_state: ResMut<BevyState<State>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state
            .set(State::Hello)
            .expect("Failed to set game state");
    }
}
