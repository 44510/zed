use std::marker::PhantomData;

use crate::prelude::*;

#[derive(Element)]
pub struct ToolDivider<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync> ToolDivider<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let color = ThemeColor::new();

        div().w_px().h_3().bg(color.border)
    }
}
