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

use crate::game::{events, State};
use crate::scenes::menu::Submenu;
use bevy::{app::AppExit, prelude::*};

const AUDIO: &str = "menu/click.ogg";

#[derive(Component, PartialEq)]
pub enum Action {
    Play,
    Options,
    Windowed,
    FullScreen,
    Quit,
    Back,
}

use bevy::prelude::State as BevyState;

pub fn system(
    interaction_query: Query<(&Interaction, &Action), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<BevyState<State>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut menu_state: ResMut<BevyState<Submenu>>,
    mut ev_change_display_mode: EventWriter<events::ChangeDisplayMode>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Clicked {
            match button_action {
                Action::Quit => app_exit_events.send(AppExit),
                Action::Play => {
                    game_state
                        .set(State::Hello)
                        .expect("Failed to set game state");
                    menu_state
                        .set(Submenu::None)
                        .expect("Failed to set menu state");
                }
                Action::Options => menu_state
                    .set(Submenu::Options)
                    .expect("Failed to set menu state"),
                Action::Back => menu_state
                    .set(Submenu::Main)
                    .expect("Failed to set menu state"),
                Action::Windowed => {
                    ev_change_display_mode.send(events::ChangeDisplayMode::windowed());
                }

                Action::FullScreen => {
                    ev_change_display_mode.send(events::ChangeDisplayMode::full_screen());
                }
            }
            audio.play(asset_server.load(AUDIO));
        }
    }
}
