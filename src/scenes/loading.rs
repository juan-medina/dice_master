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

use bevy::{
    prelude::*,
    render::texture::{CompressedImageFormats, ImageType},
};
use iyes_progress::ProgressCounter;

use super::clear_scene;
use crate::{effects, game::State};

pub struct Loading;

impl Plugin for Loading {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(State::Loading).with_system(setup))
            .add_system_set(SystemSet::on_update(State::Loading).with_system(print_progress))
            .add_system_set(
                SystemSet::on_exit(State::Loading).with_system(clear_scene::<OnLoadingScene>),
            )
            .add_system_set(
                SystemSet::on_update(State::Loading)
                    .with_system(fake_long_task.before(print_progress)),
            );
    }
}

#[derive(Component)]
struct OnLoadingScene;

use bevy::asset::Assets as BevyAssets;
fn setup(mut commands: Commands, mut images: ResMut<BevyAssets<Image>>) {
    let buff = include_bytes!("../../embedded/loading/load.png");
    let image_type = ImageType::Extension("png");
    let image = Image::from_buffer(buff, image_type, CompressedImageFormats::NONE, false)
        .expect("Failed to load image from buffer");
    let handle = images.add(image);
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                },
                ..Default::default()
            },
            OnLoadingScene,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            bottom: Val::Px(0.),
                            right: Val::Px(0.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    image: UiImage::from(handle),
                    ..Default::default()
                },
                effects::Rotate::angle_per_second(200.),
            ));
        });
}

fn print_progress(progress: Option<Res<ProgressCounter>>, mut last_done: Local<u32>) {
    if let Some(progress) = progress.map(|counter| counter.progress()) {
        if progress.done > *last_done {
            *last_done = progress.done;
            debug!(" Changed progress: {:?}", progress);
        }
    }
}

const DURATION_LONG_TASK_IN_SECS: f64 = 5.0;
fn fake_long_task(time: Res<Time>, progress: Res<ProgressCounter>) {
    if time.elapsed_seconds_f64() > DURATION_LONG_TASK_IN_SECS {
        progress.manually_track(true.into());
    } else {
        progress.manually_track(false.into());
    }
}
