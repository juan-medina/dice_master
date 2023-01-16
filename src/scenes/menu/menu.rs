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
use crate::game::{events, Assets, Config, DisplayMode, State};
use bevy::prelude::*;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Submenu {
    None,
    Main,
    Options,
}

pub struct Menu;

impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(State::Menu).with_system(setup))
            .add_system_set(SystemSet::on_update(State::Menu).with_system(buttons::colors))
            .add_system_set(SystemSet::on_update(State::Menu).with_system(actions::system))
            .add_system_set(SystemSet::on_exit(State::Menu).with_system(clear_scene::<OnMenuScene>))
            .add_state(Submenu::None)
            .add_system_set(SystemSet::on_enter(Submenu::Main).with_system(setup_main))
            .add_system_set(
                SystemSet::on_exit(Submenu::Main).with_system(clear_scene::<OnMenuScene>),
            )
            .add_system_set(SystemSet::on_enter(Submenu::Options).with_system(setup_options))
            .add_system_set(
                SystemSet::on_update(Submenu::Options).with_system(update_options_buttons),
            )
            .add_system_set(
                SystemSet::on_exit(Submenu::Options).with_system(clear_scene::<OnMenuScene>),
            );
    }
}

const FONT_SIZE: f32 = 80.0;
const FONT_SIZE_SMALL: f32 = 45.0;
const FONT_COLOR: Color = Color::WHITE;

#[derive(Component)]
struct OnMenuScene;

use bevy::prelude::State as BevyState;
fn setup(mut menu_state: ResMut<BevyState<Submenu>>) {
    let _ = menu_state.set(Submenu::Main);
}

fn setup_main(mut commands: Commands, assets: Res<Assets>) {
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
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Menu",
                            TextStyle {
                                font: assets.default_font.clone(),
                                font_size: FONT_SIZE,
                                color: FONT_COLOR,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );
                    buttons::add(parent, "Play", Action::Play, assets.as_ref());
                    buttons::add(parent, "Options", Action::Options, assets.as_ref());
                    buttons::add(parent, "Quit", Action::Quit, assets.as_ref());
                });
        });
}

fn setup_options(mut commands: Commands, assets: Res<Assets>, config: Res<Config>) {
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
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Options",
                            TextStyle {
                                font: assets.default_font.clone(),
                                font_size: FONT_SIZE,
                                color: FONT_COLOR,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::GRAY.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Display:",
                                    TextStyle {
                                        font: assets.default_font.clone(),
                                        font_size: FONT_SIZE_SMALL,
                                        color: FONT_COLOR,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(10.0)),
                                    ..default()
                                }),
                            );
                            buttons::setting(
                                parent,
                                "Windowed",
                                config.mode == DisplayMode::Windowed,
                                Action::Windowed,
                                assets.as_ref(),
                            );
                            buttons::setting(
                                parent,
                                "Full Screen",
                                config.mode == DisplayMode::FullScreen,
                                Action::FullScreen,
                                assets.as_ref(),
                            );
                        });
                    buttons::add(parent, "Back", Action::Back, assets.as_ref());
                });
        });
}

fn update_options_buttons(
    mut ev_change_display_mode: EventReader<events::ChangeDisplayMode>,
    mut buttons_query: Query<(Entity, &mut BackgroundColor, &Action)>,
    mut commands: Commands,
) {
    for change_display_mode in ev_change_display_mode.iter() {
        for (entity, mut background_color, action) in buttons_query.iter_mut() {
            let need_selection_change = match action {
                Action::Windowed => Some(DisplayMode::Windowed),
                Action::FullScreen => Some(DisplayMode::FullScreen),
                _ => None,
            };
            if let Some(mode) = need_selection_change {
                let selected = mode == change_display_mode.mode;
                buttons::change_selection(selected, entity, &mut background_color, &mut commands);
            }
        }
    }
}
