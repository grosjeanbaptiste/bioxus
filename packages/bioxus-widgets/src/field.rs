//! Form fields: a label, a native control and an optional error. Native controls only
//! (`number`, `date`, `select`, ...), so every platform shows its own picker.

use dioxus::prelude::*;

/// Native input type of an [`InputField`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputKind {
    #[default]
    Text,
    Number,
    Date,
}

impl InputKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Number => "number",
            Self::Date => "date",
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct InputFieldProps {
    pub label: String,
    pub value: String,
    pub on_input: EventHandler<String>,
    #[props(default)]
    pub kind: InputKind,
    #[props(default)]
    pub placeholder: Option<String>,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub autofocus: bool,
    #[props(default)]
    pub max_length: Option<i64>,
    /// Extra class on the input.
    #[props(default)]
    pub class: Option<String>,
    /// Keep the label for assistive tech only.
    #[props(default)]
    pub label_hidden: bool,
    /// Shown under the input and announced as invalid when set.
    #[props(default)]
    pub error: Option<String>,
}

#[component]
pub fn InputField(props: InputFieldProps) -> Element {
    let on_input = props.on_input;
    rsx! {
        label { class: "bx-field",
            span { class: if props.label_hidden { "bx-visually-hidden" } else { "bx-label" }, "{props.label}" }
            input {
                r#type: props.kind.as_str(),
                class: props.class.clone(),
                placeholder: props.placeholder.clone(),
                disabled: props.disabled,
                autofocus: props.autofocus,
                maxlength: props.max_length,
                value: "{props.value}",
                "aria-invalid": props.error.is_some().then_some("true"),
                oninput: move |e| on_input.call(e.value()),
            }
            if let Some(error) = &props.error {
                span { class: "bx-error", role: "alert", "{error}" }
            }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct SelectFieldProps {
    pub label: String,
    /// Value of the current option.
    pub value: String,
    /// `(value, label)` of each option, in display order.
    pub options: Vec<(String, String)>,
    pub on_change: EventHandler<String>,
    /// Keep the label for assistive tech only.
    #[props(default)]
    pub label_hidden: bool,
    /// Shown under the select and announced as invalid when set.
    #[props(default)]
    pub error: Option<String>,
}

#[component]
pub fn SelectField(props: SelectFieldProps) -> Element {
    let on_change = props.on_change;
    rsx! {
        label { class: "bx-field",
            span { class: if props.label_hidden { "bx-visually-hidden" } else { "bx-label" }, "{props.label}" }
            select {
                "aria-invalid": props.error.is_some().then_some("true"),
                onchange: move |e| on_change.call(e.value()),
                for (value, text) in props.options.iter() {
                    option { key: "{value}", value: "{value}", selected: *value == props.value, "{text}" }
                }
            }
            if let Some(error) = &props.error {
                span { class: "bx-error", role: "alert", "{error}" }
            }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct TextAreaFieldProps {
    pub label: String,
    pub value: String,
    pub on_input: EventHandler<String>,
    /// Keep the label for assistive tech only.
    #[props(default)]
    pub label_hidden: bool,
    /// Shown under the text area and announced as invalid when set.
    #[props(default)]
    pub error: Option<String>,
}

#[component]
pub fn TextAreaField(props: TextAreaFieldProps) -> Element {
    let on_input = props.on_input;
    rsx! {
        label { class: "bx-field",
            span { class: if props.label_hidden { "bx-visually-hidden" } else { "bx-label" }, "{props.label}" }
            textarea {
                value: "{props.value}",
                "aria-invalid": props.error.is_some().then_some("true"),
                oninput: move |e| on_input.call(e.value()),
            }
            if let Some(error) = &props.error {
                span { class: "bx-error", role: "alert", "{error}" }
            }
        }
    }
}

/// A native checkbox with its label.
#[component]
pub fn CheckField(
    label: String,
    checked: bool,
    on_change: EventHandler<bool>,
    #[props(default)] disabled: bool,
) -> Element {
    rsx! {
        label { class: "bx-check",
            input {
                r#type: "checkbox",
                checked,
                disabled,
                onchange: move |e| on_change.call(e.checked()),
            }
            span { "{label}" }
        }
    }
}
