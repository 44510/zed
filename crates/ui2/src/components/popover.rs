use gpui::{
    AnyElement, Component, Div, Element, ElementId, ParentElement, Rems, RenderOnce, Styled,
    WindowContext,
};
use smallvec::SmallVec;

use crate::{v_stack, StyledExt};

/// A popover is used to display a menu or show some options.
///
/// Clicking the element that launches the popover should not change the current view,
/// and the popover should be statically positioned relative to that element (not the
/// user's mouse.)
///
/// Example: A "new" menu with options like "new file", "new folder", etc,
/// Linear's "Display" menu, a profile menu that appers when you click your avatar.
///
/// Related elements:
///
/// `ContextMenu`:
///
/// Used to display a popover menu that only contains a list of items. Context menus are always
/// launched by secondary clicking on an element. The menu is positioned relative to the user's cursor.
///
/// Example: Right clicking a file in the file tree to get a list of actions, right clicking
/// a tab to in the tab bar to get a list of actions.
///
/// `Dropdown`:
///
/// Used to display a list of options when the user clicks an element. The menu is
/// positioned relative the element that was clicked, and clicking an item in the
/// dropdown should change the value of the element that was clicked.
///
/// Example: A theme select control. Displays "One Dark", clicking it opens a list of themes.
/// When one is selected, the theme select control displays the selected theme.
#[derive(RenderOnce)]
pub struct Popover {
    children: SmallVec<[AnyElement; 2]>,
    aside: Option<AnyElement>,
    max_height: Option<Rems>,
}

impl Component for Popover {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        v_stack()
            .relative()
            .elevation_2(cx)
            .p_1()
            .when_some(self.max_height, |this, max_height| {
                this.max_h(max_height).overflow_hidden_y()
            })
            .children(self.children)
            .when_some(self.aside, |this, aside| {
                // TODO: This will statically position the aside to the top right of the popover.
                // We should update this to use gpui2::overlay avoid collisions with the window edges.
                this.child(
                    v_stack()
                        .top_0()
                        .left_full()
                        .ml_1()
                        .absolute()
                        .elevation_2(cx)
                        .p_1()
                        .child(aside),
                )
            })
    }
}

impl Popover {
    pub fn new() -> Self {
        Self {
            children: SmallVec::new(),
            aside: None,
            max_height: None,
        }
    }

    pub fn max_height(mut self, rems: Rems) -> Self {
        self.max_height = Some(rems);
        self
    }

    pub fn aside(mut self, aside: impl RenderOnce) -> Self
    where
        Self: Sized,
    {
        self.aside = Some(aside.render_once().into_any());
        self
    }
}

impl ParentElement for Popover {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.children
    }
}
