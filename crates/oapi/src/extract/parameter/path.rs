use std::fmt::{self, Formatter};
use std::ops::{Deref, DerefMut};

use salvo_core::extract::{Extractible, Metadata};
use salvo_core::http::ParseError;
use salvo_core::{async_trait, Request};
use serde::Deserialize;
use serde::Deserializer;

use crate::endpoint::EndpointModifier;
use crate::{AsParameter, Components, Operation, Parameter, ParameterIn};

/// Represents the parameters passed by the URI path.
pub struct PathParam<T> {
    name: String,
    value: T,
}
impl<T> PathParam<T> {
    /// Construct a new [`PathParam`] with given `name` and `value`.
    pub fn new(name: &str, value: T) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
    /// Returns the name of the parameter.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns the value of the parameter.
    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<T> Deref for PathParam<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for PathParam<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> AsParameter for PathParam<T> {
    fn parameter() -> Parameter {
        panic!("path parameter must have a argument");
    }
    fn parameter_with_arg(arg: &str) -> Parameter {
        Parameter::new(arg)
            .parameter_in(ParameterIn::Path)
            .description(format!("Get parameter `{arg}` from request url path"))
    }
}

impl<'de, T> Deserialize<'de> for PathParam<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(|value| PathParam {
            name: "unknown".into(),
            value,
        })
    }
}

impl<T> fmt::Debug for PathParam<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("PathParam")
            .field("name", &self.name)
            .field("value", &self.value)
            .finish()
    }
}

#[async_trait]
impl<'de, T> Extractible<'de> for PathParam<T>
where
    T: Deserialize<'de>,
{
    fn metadata() -> &'de Metadata {
        static METADATA: Metadata = Metadata::new("");
        &METADATA
    }
    async fn extract(_req: &'de mut Request) -> Result<Self, ParseError> {
        unimplemented!("path parameter can not be extracted from request")
    }
    async fn extract_with_arg(req: &'de mut Request, arg: &str) -> Result<Self, ParseError> {
        let value = req
            .param(arg)
            .ok_or_else(|| ParseError::other(format!("path parameter {} not found or convert to type failed", arg)))?;
        Ok(Self {
            name: arg.to_string(),
            value,
        })
    }
}

#[async_trait]
impl<T> EndpointModifier for PathParam<T> {
    fn modify(_components: &mut Components, _operation: &mut Operation) {
        panic!("path parameter can not modiify operation without argument");
    }
    fn modify_with_arg(_components: &mut Components, operation: &mut Operation, arg: &str) {
        operation.parameters.insert(Self::parameter_with_arg(arg));
    }
}