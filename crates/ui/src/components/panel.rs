use std::marker::PhantomData;

use gpui2::geometry::AbsoluteLength;

use crate::prelude::*;
use crate::{theme, token, v_stack};

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PanelAllowedSides {
    LeftOnly,
    RightOnly,
    BottomOnly,
    #[default]
    LeftAndRight,
    All,
}

impl PanelAllowedSides {
    /// Return a `HashSet` that contains the allowable `PanelSide`s.
    pub fn allowed_sides(&self) -> HashSet<PanelSide> {
        match self {
            Self::LeftOnly => HashSet::from_iter([PanelSide::Left]),
            Self::RightOnly => HashSet::from_iter([PanelSide::Right]),
            Self::BottomOnly => HashSet::from_iter([PanelSide::Bottom]),
            Self::LeftAndRight => HashSet::from_iter([PanelSide::Left, PanelSide::Right]),
            Self::All => HashSet::from_iter([PanelSide::Left, PanelSide::Right, PanelSide::Bottom]),
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PanelSide {
    #[default]
    Left,
    Right,
    Bottom,
}

use std::collections::HashSet;

#[derive(Element)]
pub struct Panel<V: 'static> {
    view_type: PhantomData<V>,
    scroll_state: ScrollState,
    current_side: PanelSide,
    /// Defaults to PanelAllowedSides::LeftAndRight
    allowed_sides: PanelAllowedSides,
    initial_width: AbsoluteLength,
    width: Option<AbsoluteLength>,
}

impl<V: 'static> Panel<V> {
    pub fn new(scroll_state: ScrollState) -> Self {
        let token = token();

        Self {
            view_type: PhantomData,
            scroll_state,
            current_side: PanelSide::default(),
            allowed_sides: PanelAllowedSides::default(),
            initial_width: token.default_panel_size,
            width: None,
        }
    }

    pub fn initial_width(mut self, initial_width: AbsoluteLength) -> Self {
        self.initial_width = initial_width;
        self
    }

    pub fn width(mut self, width: AbsoluteLength) -> Self {
        self.width = Some(width);
        self
    }

    pub fn allowed_sides(mut self, allowed_sides: PanelAllowedSides) -> Self {
        self.allowed_sides = allowed_sides;
        self
    }

    pub fn side(mut self, side: PanelSide) -> Self {
        let allowed_sides = self.allowed_sides.allowed_sides();

        if allowed_sides.contains(&side) {
            self.current_side = side;
        } else {
            panic!(
                "The panel side {:?} was not added as allowed before it was set.",
                side
            );
        }
        self
    }

    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let token = token();
        let theme = theme(cx);

        let panel_base;
        let current_width = if let Some(width) = self.width {
            width
        } else {
            self.initial_width
        };

        match self.current_side {
            PanelSide::Left => {
                panel_base = v_stack()
                    .overflow_y_scroll(self.scroll_state.clone())
                    .h_full()
                    .w(current_width)
                    .fill(theme.middle.base.default.background)
                    .border_r()
                    .border_color(theme.middle.base.default.border);
            }
            PanelSide::Right => {
                panel_base = v_stack()
                    .overflow_y_scroll(self.scroll_state.clone())
                    .h_full()
                    .w(current_width)
                    .fill(theme.middle.base.default.background)
                    .border_r()
                    .border_color(theme.middle.base.default.border);
            }
            PanelSide::Bottom => {
                panel_base = v_stack()
                    .overflow_y_scroll(self.scroll_state.clone())
                    .w_full()
                    .h(current_width)
                    .fill(theme.middle.base.default.background)
                    .border_r()
                    .border_color(theme.middle.base.default.border);
            }
        }

        panel_base
    }
}
