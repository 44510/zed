use std::marker::PhantomData;

use crate::prelude::*;

#[derive(Clone, Copy)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

#[derive(Element)]
struct TrafficLight<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    color: TrafficLightColor,
    window_has_focus: bool,
}

impl<S: 'static + Send + Sync> TrafficLight<S> {
    fn new(color: TrafficLightColor, window_has_focus: bool) -> Self {
        Self {
            state_type: PhantomData,
            color,
            window_has_focus,
        }
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let color = ThemeColor::new();

        let fill = match (self.window_has_focus, self.color) {
            (true, TrafficLightColor::Red) => color.mac_os_traffic_light_red,
            (true, TrafficLightColor::Yellow) => color.mac_os_traffic_light_yellow,
            (true, TrafficLightColor::Green) => color.mac_os_traffic_light_green,
            (false, _) => color.filled_element,
        };

        div().w_3().h_3().rounded_full().bg(fill)
    }
}

#[derive(Element)]
pub struct TrafficLights<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    window_has_focus: bool,
}

impl<S: 'static + Send + Sync> TrafficLights<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
            window_has_focus: true,
        }
    }

    pub fn window_has_focus(mut self, window_has_focus: bool) -> Self {
        self.window_has_focus = window_has_focus;
        self
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let color = ThemeColor::new();

        div()
            .flex()
            .items_center()
            .gap_2()
            .child(TrafficLight::new(
                TrafficLightColor::Red,
                self.window_has_focus,
            ))
            .child(TrafficLight::new(
                TrafficLightColor::Yellow,
                self.window_has_focus,
            ))
            .child(TrafficLight::new(
                TrafficLightColor::Green,
                self.window_has_focus,
            ))
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::Story;

    use super::*;

    #[derive(Element)]
    pub struct TrafficLightsStory<S: 'static + Send + Sync> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync> TrafficLightsStory<S> {
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
            Story::container(cx)
                .child(Story::title_for::<_, TrafficLights<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(TrafficLights::new())
                .child(Story::label(cx, "Unfocused"))
                .child(TrafficLights::new().window_has_focus(false))
        }
    }
}
