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

use crate::game::State;
use bevy::prelude::*;
use bevy_tweening::*;
use std::time::Duration;

pub fn delay<T>(seconds: u64) -> Delay<T> {
    Delay::new(Duration::from_secs(seconds))
}

pub struct Handler;

impl Plugin for Handler {
    fn build(&self, app: &mut App) {
        app.add_system(go_to_state_system);
    }
}

#[derive(Debug, Clone, Resource)]
pub struct GoToState {
    state: State,
    timer: Timer,
}

impl GoToState {
    pub fn new(state: State, timer: Timer) -> Self {
        Self { state, timer }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Go {
    pub state: State,
}

impl Go {
    pub fn to(state: State) -> Self {
        Self { state }
    }
    pub fn after(&self, seconds: f32) -> GoToState {
        GoToState::new(self.state, Timer::from_seconds(seconds, TimerMode::Once))
    }
}

use bevy::prelude::State as BevyState;
fn go_to_state_system(
    mut game_state: ResMut<BevyState<State>>,
    time: Res<Time>,
    mut op_goto: Option<ResMut<GoToState>>,
    mut commands: Commands,
) {
    if let Some(goto) = op_goto.as_mut() {
        goto.timer.tick(time.delta());
        if goto.timer.finished() {
            game_state
                .set(goto.state)
                .expect("Failed to set game state");

            commands.remove_resource::<GoToState>();
        }
    }
}
