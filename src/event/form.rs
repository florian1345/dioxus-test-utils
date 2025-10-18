use std::any::Any;

use dioxus_html::{FileData, FormValue, HasFileData, HasFormData};

use crate::event::EventType;

#[derive(Clone)]
pub struct TestFormData {
    /// See [HasFormData::value].
    pub value: String,

    /// See [HasFormData::valid].
    pub valid: bool,

    /// See [HasFormData::values].
    pub values: Vec<(String, FormValue)>,

    /// See [HasFileData::files].
    pub files: Vec<FileData>,
}

impl Default for TestFormData {
    fn default() -> TestFormData {
        TestFormData {
            value: String::new(),
            valid: true,
            values: Vec::new(),
            files: Vec::new(),
        }
    }
}

impl HasFileData for TestFormData {
    fn files(&self) -> Vec<FileData> {
        self.files.clone()
    }
}

impl HasFormData for TestFormData {
    fn value(&self) -> String {
        self.value.clone()
    }

    fn valid(&self) -> bool {
        self.valid
    }

    fn values(&self) -> Vec<(String, FormValue)> {
        self.values.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub enum FormEventType {
    Change,
    Input,
    Invalid,
    Reset,
    Submit,
}

impl EventType for FormEventType {
    type Data = TestFormData;

    fn name(self) -> &'static str {
        match self {
            FormEventType::Change => "change",
            FormEventType::Input => "input",
            FormEventType::Invalid => "invalid",
            FormEventType::Reset => "reset",
            FormEventType::Submit => "submit",
        }
    }
}
