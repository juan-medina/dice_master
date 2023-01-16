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

use super::actions::Action;

#[derive(Component)]
pub struct SelectedButton;

pub const NORMAL_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const HOVERED_SELECTED_COLOR: Color = Color::rgb(0.25, 0.65, 0.25);
pub const CLICKED_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const BUTTON_FONT_NAME: &str = "fonts/FiraSans-Bold.ttf";
const BUTTON_FONT_SIZE: f32 = 40.0;
const BUTTON_FONT_SIZE_SMALL: f32 = 30.0;

pub fn colors(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedButton>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Clicked, _) => CLICKED_COLOR.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_SELECTED_COLOR.into(),
            (Interaction::Hovered, None) => HOVERED_COLOR.into(),
            (Interaction::None, Some(_)) => CLICKED_COLOR.into(),
            (Interaction::None, None) => NORMAL_COLOR.into(),
        }
    }
}

pub fn add(parent: &mut ChildBuilder, text: &str, action: Action, asset_server: &AssetServer) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: NORMAL_COLOR.into(),
                ..default()
            },
            action,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: asset_server.load(BUTTON_FONT_NAME),
                    font_size: BUTTON_FONT_SIZE,
                    color: TEXT_COLOR,
                },
            ));
        });
}

pub fn setting(
    parent: &mut ChildBuilder,
    text: &str,
    selected: bool,
    action: Action,
    asset_server: &AssetServer,
) {
    let color = if selected {
        CLICKED_COLOR
    } else {
        NORMAL_COLOR
    };

    let mut button = parent.spawn((
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(50.0)),
                margin: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: color.into(),
            ..default()
        },
        action,
    ));

    if selected {
        button.insert(SelectedButton);
    }

    button.with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font: asset_server.load(BUTTON_FONT_NAME),
                font_size: BUTTON_FONT_SIZE_SMALL,
                color: TEXT_COLOR,
            },
        ));
    });
}

fn select(entity: Entity, background_color: &mut Mut<BackgroundColor>, commands: &mut Commands) {
    background_color.0 = CLICKED_COLOR;
    commands.entity(entity).insert(SelectedButton);
}

fn unselect(entity: Entity, background_color: &mut Mut<BackgroundColor>, commands: &mut Commands) {
    background_color.0 = NORMAL_COLOR;
    commands.entity(entity).remove::<SelectedButton>();
}

pub fn change_selection(
    selected: bool,
    entity: Entity,
    background_color: &mut Mut<BackgroundColor>,
    commands: &mut Commands,
) {
    if selected {
        select(entity, background_color, commands);
    } else {
        unselect(entity, background_color, commands);
    }
}
