use std::marker::PhantomData;

use crate::prelude::*;
use crate::{v_stack, Buffer, Icon, IconButton, Label};

#[derive(Element)]
pub struct MultiBuffer<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    buffers: Vec<Buffer<S>>,
}

impl<S: 'static + Send + Sync + Clone> MultiBuffer<S> {
    pub fn new(buffers: Vec<Buffer<S>>) -> Self {
        Self {
            state_type: PhantomData,
            buffers,
        }
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let color = ThemeColor::new();

        v_stack()
            .w_full()
            .h_full()
            .flex_1()
            .children(self.buffers.clone().into_iter().map(|buffer| {
                v_stack()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .p_4()
                            .bg(color.editor_subheader)
                            .child(Label::new("main.rs"))
                            .child(IconButton::new("arrow_up_right", Icon::ArrowUpRight)),
                    )
                    .child(buffer)
            }))
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::{hello_world_rust_buffer_example, Story};

    use super::*;

    #[derive(Element)]
    pub struct MultiBufferStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> MultiBufferStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(
            &mut self,
            _view: &mut S,
            cx: &mut ViewContext<S>,
        ) -> impl Element<ViewState = S> {
            let color = ThemeColor::new();

            Story::container(cx)
                .child(Story::title_for::<_, MultiBuffer<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(MultiBuffer::new(vec![
                    hello_world_rust_buffer_example(&color),
                    hello_world_rust_buffer_example(&color),
                    hello_world_rust_buffer_example(&color),
                    hello_world_rust_buffer_example(&color),
                    hello_world_rust_buffer_example(&color),
                ]))
        }
    }
}
