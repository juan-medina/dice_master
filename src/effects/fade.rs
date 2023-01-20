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

use bevy::prelude::*;
use bevy_tweening::{lens::SpriteColorLens, *};

use super::{delay, INVISIBLE, VISIBLE};

pub fn sprite_in(seconds: u64) -> Tween<Sprite> {
    Tween::new(
        EaseFunction::QuadraticIn,
        Duration::from_secs(seconds),
        SpriteColorLens {
            start: INVISIBLE.into(),
            end: VISIBLE.into(),
        },
    )
}

pub fn sprite_out(seconds: u64) -> Tween<Sprite> {
    Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs(seconds),
        SpriteColorLens {
            start: VISIBLE.into(),
            end: INVISIBLE.into(),
        },
    )
}

pub fn out_sprite() -> Sprite {
    Sprite {
        color: INVISIBLE,
        ..Default::default()
    }
}

pub fn in_out_sprite(time_in: u64, pause: u64, time_out: u64) -> Animator<Sprite> {
    let fade_in = sprite_in(time_in);
    let pause = delay(pause);
    let fade_out = sprite_out(time_out);

    Animator::new(fade_in.then(pause).then(fade_out))
}
