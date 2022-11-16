// Copyright 2022 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use glazier::kurbo::{Point, Size};
use piet_scene::{Affine, Brush, Color, SceneBuilder, SceneFragment};

use crate::text::ParleyBrush;

use super::{
    align::{FirstBaseline, LastBaseline, SingleAlignment, VertAlignment},
    contexts::LifeCycleCx,
    AlignCx, EventCx, LayoutCx, LifeCycle, PaintCx, RawEvent, Rendered, UpdateCx, Widget,
};

pub struct TextWidget {
    text: String,
    is_wrapped: bool,
}

impl TextWidget {
    pub fn new(text: String) -> TextWidget {
        TextWidget {
            text,
            is_wrapped: false,
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

impl Widget for TextWidget {
    fn event(&mut self, _cx: &mut EventCx, _event: &RawEvent) {}

    fn lifecycle(&mut self, _cx: &mut LifeCycleCx, _event: &LifeCycle) {}

    fn update(&mut self, cx: &mut UpdateCx) {
        // All changes potentially require layout. Note: we could be finer
        // grained, maybe color changes wouldn't.
        cx.request_layout();
    }

    fn measure(&mut self, cx: &mut LayoutCx) -> (Size, Size) {
        let min_size = Size::ZERO;
        let max_size = Size::new(50.0, 50.0);
        self.is_wrapped = false;
        (min_size, max_size)
    }

    fn layout(&mut self, cx: &mut LayoutCx, proposed_size: Size) -> Size {
        cx.widget_state.max_size
    }

    fn align(&self, cx: &mut AlignCx, alignment: SingleAlignment) {}

    fn paint(&mut self, cx: &mut PaintCx) -> Rendered {
        let mut fragment = SceneFragment::default();
        let mut builder = SceneBuilder::for_fragment(&mut fragment);
        let mut lcx = parley::LayoutContext::new();
        let mut layout_builder = lcx.ranged_builder(cx.font_cx(), "hello xilem", 1.0);
        layout_builder.push_default(&parley::style::StyleProperty::Brush(ParleyBrush(
            Brush::Solid(Color::rgb8(255, 255, 255)),
        )));
        let mut layout = layout_builder.build();
        // Question for Chad: is this needed?
        layout.break_all_lines(None, parley::layout::Alignment::Start);
        let transform = Affine::translate(40.0, 40.0);
        crate::text::render_text(&mut builder, transform, &layout);
        Rendered(fragment)
    }
}
