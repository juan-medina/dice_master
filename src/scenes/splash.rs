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

use std::time::Duration;

use super::clear_scene;
use crate::game::{Assets, State};
use bevy::prelude::*;
use bevy_tweening::{
    lens::{SpriteColorLens, TransformPositionLens},
    *,
};

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

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

const INVISIBLE: Color = Color::rgba(1., 1., 1., 0.0);
const VISIBLE: Color = Color::rgba(1., 1., 1., 1.0);
const FADE_IN_DURATION: u64 = 2;
const PAUSE_DURATION: u64 = 2;
const FADE_OUT_DURATION: u64 = 1;
const TO_NEXT_SCENE_DURATION: f32 =
    (FADE_IN_DURATION + PAUSE_DURATION + FADE_OUT_DURATION + 1) as f32;

fn setup(mut commands: Commands, audio: Res<Audio>, assets: Res<Assets>) {
    let fade_in = Tween::new(
        EaseFunction::QuadraticIn,
        Duration::from_secs(FADE_IN_DURATION),
        SpriteColorLens {
            start: INVISIBLE.into(),
            end: VISIBLE.into(),
        },
    );
    let pause = Delay::new(Duration::from_secs(PAUSE_DURATION));
    let fade_out = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs(FADE_OUT_DURATION),
        SpriteColorLens {
            start: VISIBLE.into(),
            end: INVISIBLE.into(),
        },
    );
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: INVISIBLE.into(),
                ..default()
            },
            texture: assets.newolds_logo.clone(),
            ..default()
        },
        OnSplashScene,
        Animator::new(fade_in.then(pause).then(fade_out)),
    ));

    audio.play(assets.newolds_sound.clone());

    commands.insert_resource(SplashTimer(Timer::from_seconds(
        TO_NEXT_SCENE_DURATION,
        TimerMode::Once,
    )));
}

use bevy::prelude::State as BevyState;

fn countdown(
    mut game_state: ResMut<BevyState<State>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state
            .set(State::Menu)
            .expect("Failed to set game state");
    }
}
