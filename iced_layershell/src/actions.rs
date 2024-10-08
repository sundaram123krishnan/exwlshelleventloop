use crate::reexport::{Anchor, Layer};
use iced::window::Id as IcedId;
use iced_core::mouse::Interaction;
use iced_runtime::command::Action;
use layershellev::id::Id as LayerId;
use layershellev::NewLayerShellSettings;

#[allow(unused)]
#[derive(Debug, Clone)]
pub(crate) enum LayerShellActions<INFO: Clone> {
    Mouse(Interaction),
    CustomActions(Vec<LayershellCustomActionsWithInfo<INFO>>),
    CustomActionsWithId(Vec<LayershellCustomActionsWithIdInner<INFO>>),
    RedrawAll,
    RedrawWindow(LayerId), // maybe one day it is useful, but now useless
    NewMenu((IcedNewPopupSettings, INFO)),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IcedNewPopupSettings {
    pub size: (u32, u32),
    pub position: (i32, i32),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MenuDirection {
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IcedNewMenuSettings {
    pub size: (u32, u32),
    pub direction: MenuDirection,
}

#[derive(Debug, Clone, Copy)]
pub enum LayershellCustomActionsWithInfo<INFO: Clone> {
    AnchorChange(Anchor),
    LayerChange(Layer),
    MarginChange((i32, i32, i32, i32)),
    SizeChange((u32, u32)),
    VirtualKeyboardPressed {
        time: u32,
        key: u32,
    },
    NewLayerShell((NewLayerShellSettings, INFO)),
    NewPopUp((IcedNewPopupSettings, INFO)),
    NewMenu((IcedNewMenuSettings, INFO)),
    /// is same with WindowAction::Close(id)
    RemoveWindow(IcedId),
    ForgetLastOutput,
}

pub type LayershellCustomActions = LayershellCustomActionsWithInfo<()>;

#[derive(Debug, Clone, Copy)]
pub struct LayershellCustomActionsWithIdAndInfo<INFO: Clone>(
    pub IcedId,
    pub LayershellCustomActionsWithInfo<INFO>,
);

impl<INFO: Clone> LayershellCustomActionsWithIdAndInfo<INFO> {
    pub fn new(id: IcedId, actions: LayershellCustomActionsWithInfo<INFO>) -> Self {
        Self(id, actions)
    }
}

pub type LayershellCustomActionsWithId = LayershellCustomActionsWithIdAndInfo<()>;

// first one means
#[derive(Debug, Clone, Copy)]
pub(crate) struct LayershellCustomActionsWithIdInner<INFO: Clone>(
    pub Option<LayerId>,                       // come from
    pub Option<LayerId>,                       // target if has one
    pub LayershellCustomActionsWithInfo<INFO>, // actions
);

impl<T, INFO: Clone + 'static> From<LayershellCustomActionsWithIdAndInfo<INFO>> for Action<T> {
    fn from(value: LayershellCustomActionsWithIdAndInfo<INFO>) -> Self {
        Action::Custom(Box::new(value.clone()))
    }
}
impl<T, INFO: Clone + 'static> From<LayershellCustomActionsWithInfo<INFO>> for Action<T> {
    fn from(value: LayershellCustomActionsWithInfo<INFO>) -> Self {
        Action::Custom(Box::new(value.clone()))
    }
}

impl<INFO: Clone + 'static> LayershellCustomActionsWithInfo<INFO> {
    pub fn to_action<T>(&self) -> Action<T> {
        (*self).clone().into()
    }
}
