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

use bevy::prelude::*;

use super::{Config, DisplayMode};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ChangeDisplayMode {
    pub mode: DisplayMode,
}

impl ChangeDisplayMode {
    pub fn to(mode: DisplayMode) -> Self {
        Self { mode }
    }

    pub fn windowed() -> Self {
        Self::to(DisplayMode::Windowed)
    }

    pub fn full_screen() -> Self {
        Self::to(DisplayMode::FullScreen)
    }
}

pub struct Handler;

impl Plugin for Handler {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangeDisplayMode>().add_system(game_events);
    }
}

fn game_events(
    mut ev_change_display_mode: EventReader<ChangeDisplayMode>,
    mut config: ResMut<Config>,
    mut windows: ResMut<Windows>,
) {
    for change_display_mode in ev_change_display_mode.iter() {
        config.mode = change_display_mode.mode.clone();
        let window = windows
            .get_primary_mut()
            .expect("we should have a primary window");
        window.set_mode(match change_display_mode.mode {
            DisplayMode::Windowed => WindowMode::Windowed,
            DisplayMode::FullScreen => WindowMode::Fullscreen,
        });
    }
}
