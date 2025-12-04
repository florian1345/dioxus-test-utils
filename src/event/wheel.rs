use std::any::Any;
use dioxus_html::geometry::{ClientPoint, ElementPoint, PagePoint, ScreenPoint, WheelDelta};
use dioxus_html::{HasMouseData, HasWheelData, InteractionElementOffset, InteractionLocation, Modifiers, ModifiersInteraction, PointerInteraction};
use dioxus_html::input_data::{MouseButton, MouseButtonSet};
use crate::event::{EventType, TestMouseData};

#[derive(Clone)]
pub struct TestWheelData {
    pub mouse_data: TestMouseData,
    pub wheel_delta: WheelDelta,
}

impl Default for TestWheelData {
    fn default() -> Self {
        TestWheelData {
            mouse_data: TestMouseData::default(),
            wheel_delta: WheelDelta::pixels(0.0, 0.0, 0.0),
        }
    }
}

impl PointerInteraction for TestWheelData {
    fn trigger_button(&self) -> Option<MouseButton> {
        self.mouse_data.trigger_button()
    }

    fn held_buttons(&self) -> MouseButtonSet {
        self.mouse_data.held_buttons()
    }
}

impl InteractionElementOffset for TestWheelData {
    fn element_coordinates(&self) -> ElementPoint {
        self.mouse_data.element_coordinates()
    }
}

impl InteractionLocation for TestWheelData {
    fn client_coordinates(&self) -> ClientPoint {
        self.mouse_data.client_coordinates()
    }

    fn screen_coordinates(&self) -> ScreenPoint {
        self.mouse_data.screen_coordinates()
    }

    fn page_coordinates(&self) -> PagePoint {
        self.mouse_data.page_coordinates()
    }
}

impl ModifiersInteraction for TestWheelData {
    fn modifiers(&self) -> Modifiers {
        self.mouse_data.modifiers()
    }
}

impl HasMouseData for TestWheelData {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl HasWheelData for TestWheelData {
    fn delta(&self) -> WheelDelta {
        self.wheel_delta
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WheelEventType {
    Wheel,
}

impl EventType for WheelEventType {
    type Data = TestWheelData;

    fn name(self) -> &'static str {
        match self {
            WheelEventType::Wheel => "wheel",
        }
    }
}
