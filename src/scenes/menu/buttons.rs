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

pub const NORMAL_COLOR: Color = Color::rgb(0.30, 0.15, 0.15);
pub const HOVERED_COLOR: Color = Color::rgb(0.45, 0.25, 0.25);
pub const PRESSED_COLOR: Color = Color::rgb(1.0, 0.35, 0.35);
pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BUTTON_FONT_NAME: &str = "fonts/FiraSans-Bold.ttf";
const BUTTON_FONT_SIZE: f32 = 40.0;

pub fn colors(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        *color = match *interaction {
            Interaction::Clicked => PRESSED_COLOR.into(),
            Interaction::Hovered => HOVERED_COLOR.into(),
            Interaction::None => NORMAL_COLOR.into(),
        }
    }
}

pub fn add(parent: &mut ChildBuilder, text: &str, action: Action, asset_server: &AssetServer) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                    // center button
                    margin: UiRect::all(Val::Px(10.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
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