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
use crate::{
    effects::{fade, Go},
    game::{Assets, State},
};
use bevy::prelude::*;

pub struct Splash;

impl Plugin for Splash {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(State::Splash).with_system(setup))
            .add_system_set(
                SystemSet::on_exit(State::Splash).with_system(clear_scene::<OnSplashScene>),
            );
    }
}

#[derive(Component)]
struct OnSplashScene;

const IN: u64 = 2;
const PAUSE: u64 = 2;
const OUT: u64 = 1;
const DELAY: f32 = (IN + PAUSE + OUT + 1) as f32;

fn setup(mut commands: Commands, audio: Res<Audio>, assets: Res<Assets>) {
    commands.spawn((
        SpriteBundle {
            sprite: fade::out_sprite(),
            texture: assets.newolds_logo.clone(),
            ..default()
        },
        OnSplashScene,
        fade::in_out_sprite(IN, PAUSE, OUT),
    ));

    audio.play(assets.newolds_sound.clone());

    commands.insert_resource(Go::to(State::Menu).after(DELAY));
}
