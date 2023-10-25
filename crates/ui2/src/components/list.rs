use std::marker::PhantomData;

use gpui2::{div, relative, Div};

use crate::settings::user_settings;
use crate::{
    h_stack, v_stack, Avatar, ClickHandler, Icon, IconColor, IconElement, IconSize, Label,
    LabelColor,
};
use crate::{prelude::*, Button};

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub enum ListItemVariant {
    /// The list item extends to the far left and right of the list.
    FullWidth,
    #[default]
    Inset,
}

#[derive(Element)]
pub struct ListHeader<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    label: SharedString,
    left_icon: Option<Icon>,
    variant: ListItemVariant,
    state: InteractionState,
    toggleable: Toggleable,
}

impl<S: 'static + Send + Sync> ListHeader<S> {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            state_type: PhantomData,
            label: label.into(),
            left_icon: None,
            variant: ListItemVariant::default(),
            state: InteractionState::default(),
            toggleable: Toggleable::Toggleable(ToggleState::Toggled),
        }
    }

    pub fn toggle(mut self, toggle: ToggleState) -> Self {
        self.toggleable = toggle.into();
        self
    }

    pub fn toggleable(mut self, toggleable: Toggleable) -> Self {
        self.toggleable = toggleable;
        self
    }

    pub fn left_icon(mut self, left_icon: Option<Icon>) -> Self {
        self.left_icon = left_icon;
        self
    }

    pub fn state(mut self, state: InteractionState) -> Self {
        self.state = state;
        self
    }

    fn disclosure_control(&self) -> Div<S> {
        let is_toggleable = self.toggleable != Toggleable::NotToggleable;
        let is_toggled = Toggleable::is_toggled(&self.toggleable);

        match (is_toggleable, is_toggled) {
            (false, _) => div(),
            (_, true) => div().child(
                IconElement::new(Icon::ChevronDown)
                    .color(IconColor::Muted)
                    .size(IconSize::Small),
            ),
            (_, false) => div().child(
                IconElement::new(Icon::ChevronRight)
                    .color(IconColor::Muted)
                    .size(IconSize::Small),
            ),
        }
    }

    fn label_color(&self) -> LabelColor {
        match self.state {
            InteractionState::Disabled => LabelColor::Disabled,
            _ => Default::default(),
        }
    }

    fn icon_color(&self) -> IconColor {
        match self.state {
            InteractionState::Disabled => IconColor::Disabled,
            _ => Default::default(),
        }
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let color = ThemeColor::new();

        let is_toggleable = self.toggleable != Toggleable::NotToggleable;
        let is_toggled = self.toggleable.is_toggled();

        let disclosure_control = self.disclosure_control();

        h_stack()
            .flex_1()
            .w_full()
            .bg(color.surface)
            .when(self.state == InteractionState::Focused, |this| {
                this.border().border_color(color.border_focused)
            })
            .relative()
            .child(
                div()
                    .h_5()
                    .when(self.variant == ListItemVariant::Inset, |this| this.px_2())
                    .flex()
                    .flex_1()
                    .w_full()
                    .gap_1()
                    .items_center()
                    .child(
                        div()
                            .flex()
                            .gap_1()
                            .items_center()
                            .children(self.left_icon.map(|i| {
                                IconElement::new(i)
                                    .color(IconColor::Muted)
                                    .size(IconSize::Small)
                            }))
                            .child(Label::new(self.label.clone()).color(LabelColor::Muted)),
                    )
                    .child(disclosure_control),
            )
    }
}

#[derive(Element)]
pub struct ListSubHeader<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    label: SharedString,
    left_icon: Option<Icon>,
    variant: ListItemVariant,
}

impl<S: 'static + Send + Sync> ListSubHeader<S> {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            state_type: PhantomData,
            label: label.into(),
            left_icon: None,
            variant: ListItemVariant::default(),
        }
    }

    pub fn left_icon(mut self, left_icon: Option<Icon>) -> Self {
        self.left_icon = left_icon;
        self
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let color = ThemeColor::new();

        h_stack().flex_1().w_full().relative().py_1().child(
            div()
                .h_6()
                .when(self.variant == ListItemVariant::Inset, |this| this.px_2())
                .flex()
                .flex_1()
                .w_full()
                .gap_1()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .gap_1()
                        .items_center()
                        .children(self.left_icon.map(|i| {
                            IconElement::new(i)
                                .color(IconColor::Muted)
                                .size(IconSize::Small)
                        }))
                        .child(Label::new(self.label.clone()).color(LabelColor::Muted)),
                ),
        )
    }
}

#[derive(Clone)]
pub enum LeftContent {
    Icon(Icon),
    Avatar(SharedString),
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum ListEntrySize {
    #[default]
    Small,
    Medium,
}

#[derive(Element)]
pub enum ListItem<S: 'static + Send + Sync> {
    Entry(ListEntry<S>),
    Details(ListDetailsEntry<S>),
    Separator(ListSeparator<S>),
    Header(ListSubHeader<S>),
}

impl<S: 'static + Send + Sync> From<ListEntry<S>> for ListItem<S> {
    fn from(entry: ListEntry<S>) -> Self {
        Self::Entry(entry)
    }
}

impl<S: 'static + Send + Sync> From<ListDetailsEntry<S>> for ListItem<S> {
    fn from(entry: ListDetailsEntry<S>) -> Self {
        Self::Details(entry)
    }
}

impl<S: 'static + Send + Sync> From<ListSeparator<S>> for ListItem<S> {
    fn from(entry: ListSeparator<S>) -> Self {
        Self::Separator(entry)
    }
}

impl<S: 'static + Send + Sync> From<ListSubHeader<S>> for ListItem<S> {
    fn from(entry: ListSubHeader<S>) -> Self {
        Self::Header(entry)
    }
}

impl<S: 'static + Send + Sync> ListItem<S> {
    fn render(&mut self, view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        match self {
            ListItem::Entry(entry) => div().child(entry.render(view, cx)),
            ListItem::Separator(separator) => div().child(separator.render(view, cx)),
            ListItem::Header(header) => div().child(header.render(view, cx)),
            ListItem::Details(details) => div().child(details.render(view, cx)),
        }
    }

    pub fn new(label: Label<S>) -> Self {
        Self::Entry(ListEntry::new(label))
    }

    pub fn as_entry(&mut self) -> Option<&mut ListEntry<S>> {
        if let Self::Entry(entry) = self {
            Some(entry)
        } else {
            None
        }
    }
}

#[derive(Element)]
pub struct ListEntry<S: 'static + Send + Sync> {
    disclosure_control_style: DisclosureControlVisibility,
    indent_level: u32,
    label: Option<Label<S>>,
    left_content: Option<LeftContent>,
    variant: ListItemVariant,
    size: ListEntrySize,
    state: InteractionState,
    toggle: Option<ToggleState>,
    overflow: OverflowStyle,
}

impl<S: 'static + Send + Sync> ListEntry<S> {
    pub fn new(label: Label<S>) -> Self {
        Self {
            disclosure_control_style: DisclosureControlVisibility::default(),
            indent_level: 0,
            label: Some(label),
            variant: ListItemVariant::default(),
            left_content: None,
            size: ListEntrySize::default(),
            state: InteractionState::default(),
            // TODO: Should use Toggleable::NotToggleable
            // or remove Toggleable::NotToggleable from the system
            toggle: None,
            overflow: OverflowStyle::Hidden,
        }
    }

    pub fn variant(mut self, variant: ListItemVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn indent_level(mut self, indent_level: u32) -> Self {
        self.indent_level = indent_level;
        self
    }

    pub fn toggle(mut self, toggle: ToggleState) -> Self {
        self.toggle = Some(toggle);
        self
    }

    pub fn left_content(mut self, left_content: LeftContent) -> Self {
        self.left_content = Some(left_content);
        self
    }

    pub fn left_icon(mut self, left_icon: Icon) -> Self {
        self.left_content = Some(LeftContent::Icon(left_icon));
        self
    }

    pub fn left_avatar(mut self, left_avatar: impl Into<SharedString>) -> Self {
        self.left_content = Some(LeftContent::Avatar(left_avatar.into()));
        self
    }

    pub fn state(mut self, state: InteractionState) -> Self {
        self.state = state;
        self
    }

    pub fn size(mut self, size: ListEntrySize) -> Self {
        self.size = size;
        self
    }

    pub fn disclosure_control_style(
        mut self,
        disclosure_control_style: DisclosureControlVisibility,
    ) -> Self {
        self.disclosure_control_style = disclosure_control_style;
        self
    }

    fn label_color(&self) -> LabelColor {
        match self.state {
            InteractionState::Disabled => LabelColor::Disabled,
            _ => Default::default(),
        }
    }

    fn icon_color(&self) -> IconColor {
        match self.state {
            InteractionState::Disabled => IconColor::Disabled,
            _ => Default::default(),
        }
    }

    fn disclosure_control(
        &mut self,
        cx: &mut ViewContext<S>,
    ) -> Option<impl Element<ViewState = S>> {
        let color = ThemeColor::new();

        let disclosure_control_icon = if let Some(ToggleState::Toggled) = self.toggle {
            IconElement::new(Icon::ChevronDown)
        } else {
            IconElement::new(Icon::ChevronRight)
        }
        .color(IconColor::Muted)
        .size(IconSize::Small);

        match (self.toggle, self.disclosure_control_style) {
            (Some(_), DisclosureControlVisibility::OnHover) => {
                Some(div().absolute().neg_left_5().child(disclosure_control_icon))
            }
            (Some(_), DisclosureControlVisibility::Always) => {
                Some(div().child(disclosure_control_icon))
            }
            (None, _) => None,
        }
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let color = ThemeColor::new();
        let color = ThemeColor::new();
        let settings = user_settings(cx);

        let left_content = match self.left_content.clone() {
            Some(LeftContent::Icon(i)) => Some(
                h_stack().child(
                    IconElement::new(i)
                        .size(IconSize::Small)
                        .color(IconColor::Muted),
                ),
            ),
            Some(LeftContent::Avatar(src)) => Some(h_stack().child(Avatar::new(src))),
            None => None,
        };

        let sized_item = match self.size {
            ListEntrySize::Small => div().h_6(),
            ListEntrySize::Medium => div().h_7(),
        };

        div()
            .relative()
            .group("")
            .bg(color.surface)
            .when(self.state == InteractionState::Focused, |this| {
                this.border().border_color(color.border_focused)
            })
            .child(
                sized_item
                    .when(self.variant == ListItemVariant::Inset, |this| this.px_2())
                    // .ml(rems(0.75 * self.indent_level as f32))
                    .children((0..self.indent_level).map(|_| {
                        div()
                            .w(*settings.list_indent_depth)
                            .h_full()
                            .flex()
                            .justify_center()
                            .group_hover("", |style| style.bg(color.border_focused))
                            .child(
                                h_stack()
                                    .child(div().w_px().h_full())
                                    .child(div().w_px().h_full().bg(color.border)),
                            )
                    }))
                    .flex()
                    .gap_1()
                    .items_center()
                    .relative()
                    .children(self.disclosure_control(cx))
                    .children(left_content)
                    .children(self.label.take()),
            )
    }
}

struct ListDetailsEntryHandlers<S: 'static + Send + Sync> {
    click: Option<ClickHandler<S>>,
}

impl<S: 'static + Send + Sync> Default for ListDetailsEntryHandlers<S> {
    fn default() -> Self {
        Self { click: None }
    }
}

#[derive(Element)]
pub struct ListDetailsEntry<S: 'static + Send + Sync> {
    label: SharedString,
    meta: Option<SharedString>,
    left_content: Option<LeftContent>,
    handlers: ListDetailsEntryHandlers<S>,
    actions: Option<Vec<Button<S>>>,
    // TODO: make this more generic instead of
    // specifically for notifications
    seen: bool,
}

impl<S: 'static + Send + Sync> ListDetailsEntry<S> {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            meta: None,
            left_content: None,
            handlers: ListDetailsEntryHandlers::default(),
            actions: None,
            seen: false,
        }
    }

    pub fn meta(mut self, meta: impl Into<SharedString>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    pub fn seen(mut self, seen: bool) -> Self {
        self.seen = seen;
        self
    }

    pub fn on_click(mut self, handler: ClickHandler<S>) -> Self {
        self.handlers.click = Some(handler);
        self
    }

    pub fn actions(mut self, actions: Vec<Button<S>>) -> Self {
        self.actions = Some(actions);
        self
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let color = ThemeColor::new();
        let settings = user_settings(cx);

        let (item_bg, item_bg_hover, item_bg_active) = match self.seen {
            true => (
                color.ghost_element,
                color.ghost_element_hover,
                color.ghost_element_active,
            ),
            false => (
                color.filled_element,
                color.filled_element_hover,
                color.filled_element_active,
            ),
        };

        let label_color = match self.seen {
            true => LabelColor::Muted,
            false => LabelColor::Default,
        };

        v_stack()
            .relative()
            .group("")
            .bg(item_bg)
            .px_1()
            .py_1p5()
            .w_full()
            .line_height(relative(1.2))
            .child(Label::new(self.label.clone()).color(label_color))
            .children(
                self.meta
                    .take()
                    .map(|meta| Label::new(meta).color(LabelColor::Muted)),
            )
            .child(
                h_stack()
                    .gap_1()
                    .justify_end()
                    .children(self.actions.take().unwrap_or_default().into_iter()),
            )
    }
}

#[derive(Clone, Element)]
pub struct ListSeparator<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync> ListSeparator<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let color = ThemeColor::new();

        div().h_px().w_full().bg(color.border)
    }
}

#[derive(Element)]
pub struct List<S: 'static + Send + Sync> {
    items: Vec<ListItem<S>>,
    empty_message: SharedString,
    header: Option<ListHeader<S>>,
    toggleable: Toggleable,
}

impl<S: 'static + Send + Sync> List<S> {
    pub fn new(items: Vec<ListItem<S>>) -> Self {
        Self {
            items,
            empty_message: "No items".into(),
            header: None,
            toggleable: Toggleable::default(),
        }
    }

    pub fn empty_message(mut self, empty_message: impl Into<SharedString>) -> Self {
        self.empty_message = empty_message.into();
        self
    }

    pub fn header(mut self, header: ListHeader<S>) -> Self {
        self.header = Some(header);
        self
    }

    pub fn toggle(mut self, toggle: ToggleState) -> Self {
        self.toggleable = toggle.into();
        self
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let color = ThemeColor::new();
        let is_toggleable = self.toggleable != Toggleable::NotToggleable;
        let is_toggled = Toggleable::is_toggled(&self.toggleable);

        let list_content = match (self.items.is_empty(), is_toggled) {
            (_, false) => div(),
            (false, _) => div().children(self.items.drain(..)),
            (true, _) => {
                div().child(Label::new(self.empty_message.clone()).color(LabelColor::Muted))
            }
        };

        v_stack()
            .py_1()
            .children(
                self.header
                    .take()
                    .map(|header| header.toggleable(self.toggleable)),
            )
            .child(list_content)
    }
}
