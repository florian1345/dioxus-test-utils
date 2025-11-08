use std::any::Any;
use dioxus_html::HasAnimationData;
use crate::event::EventType;

#[derive(Clone, Default)]
pub struct TestAnimationData {
    pub animation_name: String,
    pub pseudo_element: String,
    pub elapsed_time: f32,
}

impl HasAnimationData for TestAnimationData {
    fn animation_name(&self) -> String {
        self.animation_name.clone()
    }

    fn pseudo_element(&self) -> String {
        self.pseudo_element.clone()
    }

    fn elapsed_time(&self) -> f32 {
        self.elapsed_time
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AnimationEventType {
    AnimationStart,
    AnimationEnd,
    AnimationIteration,
}

impl EventType for AnimationEventType {
    type Data = TestAnimationData;

    fn name(self) -> &'static str {
        match self {
            AnimationEventType::AnimationStart => "animationstart",
            AnimationEventType::AnimationEnd => "animationend",
            AnimationEventType::AnimationIteration => "animationiteration",
        }
    }
}
