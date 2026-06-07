use std::marker::PhantomData;

use sge_text::{MONO, SANS};

use crate::prelude::*;

use super::PRIMARY_TEXT_COLOR;

pub struct TextInput;

impl TextInput {
    pub fn new(bg: Color, id: usize) -> UiRef {
        Fit::new(BoxFill::new(
            bg,
            base::TextInput::new(id, None, Some(SANS), 16, PRIMARY_TEXT_COLOR, 10.0),
        ))
    }

    pub fn with_prompt(bg: Color, prompt: impl ToString, id: usize) -> UiRef {
        Fit::new(BoxFill::new(
            bg,
            base::TextInput::new(
                id,
                Some(prompt.to_string()),
                Some(SANS),
                16,
                PRIMARY_TEXT_COLOR,
                10.0,
            ),
        ))
    }
}

pub struct DataInput<T: DataInputValue>(PhantomData<T>);

impl<T: DataInputValue> DataInput<T> {
    pub fn new(bg: Color, id: usize) -> UiRef {
        Fit::new(BoxFill::new(
            bg,
            base::DataInput::<T>::new(id, None, Some(SANS), 16, PRIMARY_TEXT_COLOR, 10.0),
        ))
    }

    pub fn with_prompt(bg: Color, prompt: impl ToString, id: usize) -> UiRef {
        Fit::new(BoxFill::new(
            bg,
            base::DataInput::<T>::new(
                id,
                Some(prompt.to_string()),
                Some(SANS),
                16,
                PRIMARY_TEXT_COLOR,
                10.0,
            ),
        ))
    }

    pub fn mono(bg: Color, id: usize) -> UiRef {
        Fit::new(BoxFill::new(
            bg,
            base::DataInput::<T>::new(id, None, Some(MONO), 16, PRIMARY_TEXT_COLOR, 10.0),
        ))
    }

    pub fn mono_with_prompt(bg: Color, prompt: impl ToString, id: usize) -> UiRef {
        Fit::new(BoxFill::new(
            bg,
            base::DataInput::<T>::new(
                id,
                Some(prompt.to_string()),
                Some(MONO),
                16,
                PRIMARY_TEXT_COLOR,
                10.0,
            ),
        ))
    }
}
