use std::ops::Not;

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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DisplayMode {
    Windowed,
    FullScreen,
}

impl Default for DisplayMode {
    fn default() -> Self {
        DisplayMode::Windowed
    }
}

impl Not for DisplayMode {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            DisplayMode::Windowed => DisplayMode::FullScreen,
            DisplayMode::FullScreen => DisplayMode::Windowed,
        }
    }
}

#[derive(Resource, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Config {
    pub mode: DisplayMode,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: Default::default(),
        }
    }
}
