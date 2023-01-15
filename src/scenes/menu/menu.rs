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

use super::{
    super::clear_scene,
    actions::{self, Action},
    buttons,
};
use crate::game::State;
use bevy::prelude::*;

pub struct Menu;

impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(State::Menu).with_system(setup))
            .add_system_set(SystemSet::on_update(State::Menu).with_system(buttons::colors))
            .add_system_set(SystemSet::on_update(State::Menu).with_system(actions::system))
            .add_system_set(
                SystemSet::on_exit(State::Menu).with_system(clear_scene::<OnMenuScene>),
            );
    }
}

const FONT_NAME: &str = "fonts/FiraSans-Bold.ttf";
const FONT_SIZE: f32 = 80.0;
const FONT_COLOR: Color = Color::WHITE;

#[derive(Component)]
struct OnMenuScene;

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
            OnMenuScene,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        // This will display its children in a column, from top to bottom
                        flex_direction: FlexDirection::Column,
                        // `align_items` will align children on the cross axis. Here the main axis is
                        // vertical (column), so the cross axis is horizontal. This will center the
                        // children
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Menu!",
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
                    buttons::add(parent, "Play", Action::Play, asset_server.as_ref());
                    buttons::add(parent, "Quit", Action::Quit, asset_server.as_ref());
                });
        });
}
