use std::marker::PhantomData;

use gpui2::AnyElement;
use smallvec::SmallVec;

use crate::{h_stack, prelude::*, v_stack, Button, Icon, IconButton, Label};

#[derive(Element)]
pub struct Modal<S: 'static + Send + Sync> {
    id: ElementId,
    state_type: PhantomData<S>,
    title: Option<SharedString>,
    primary_action: Option<Button<S>>,
    secondary_action: Option<Button<S>>,
    children: SmallVec<[AnyElement<S>; 2]>,
}

impl<S: 'static + Send + Sync> Modal<S> {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            state_type: PhantomData,
            title: None,
            primary_action: None,
            secondary_action: None,
            children: SmallVec::new(),
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn primary_action(mut self, action: Button<S>) -> Self {
        self.primary_action = Some(action);
        self
    }

    pub fn secondary_action(mut self, action: Button<S>) -> Self {
        self.secondary_action = Some(action);
        self
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let color = ThemeColor::new();

        v_stack()
            .id(self.id.clone())
            .w_96()
            // .rounded_xl()
            .bg(color.background)
            .border()
            .border_color(color.border)
            .shadow_2xl()
            .child(
                h_stack()
                    .justify_between()
                    .p_1()
                    .border_b()
                    .border_color(color.border)
                    .child(div().children(self.title.clone().map(|t| Label::new(t))))
                    .child(IconButton::new("close", Icon::Close)),
            )
            .child(v_stack().p_1().children(self.children.drain(..)))
            .when(
                self.primary_action.is_some() || self.secondary_action.is_some(),
                |this| {
                    this.child(
                        h_stack()
                            .border_t()
                            .border_color(color.border)
                            .p_1()
                            .justify_end()
                            .children(self.secondary_action.take())
                            .children(self.primary_action.take()),
                    )
                },
            )
    }
}

impl<S: 'static + Send + Sync> ParentElement for Modal<S> {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<Self::ViewState>; 2]> {
        &mut self.children
    }
}
