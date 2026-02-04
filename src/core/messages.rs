use crate::objects::components::Object;
use crate::tools::components::Tool;
use bevy::prelude::*;
use moonshine_kind::Instance;
use std::marker::PhantomData;

/// Message fired when a tool hits a target.
/// Generic over tool type for type-safe hit detection.
#[derive(Message)]
pub struct Hit<T: Tool> {
    /// The object entity that was hit.
    pub target: Instance<Object>,
    /// The tool entity that hit it.
    pub tool: Instance<T>,
    _marker: PhantomData<T>,
}

impl<T: Tool> Hit<T> {
    pub fn new(target: Instance<Object>, tool: Instance<T>) -> Self {
        Self {
            target,
            tool,
            _marker: PhantomData,
        }
    }
}
