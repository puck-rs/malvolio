/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/
use std::{borrow::Cow, collections::HashMap};

use crate::{
    heading_display, impl_of_heading_mutator, impl_of_heading_new_fn, into_grouping_union,
};

use super::head::head_node::HeadNode;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pub_fields", derive(FieldsAccessibleVariant))]
#[cfg_attr(feature = "fuzz", derive(serde::Serialize, serde::Deserialize))]
/// The `<title>` tag.
///
/// See the [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/title) for more
/// info.
#[must_use]
pub struct Title {
    text: Cow<'static, str>,
    attrs: HashMap<Cow<'static, str>, Cow<'static, str>>,
}

impl_of_heading_new_fn!(Title, title);

impl_of_heading_mutator!(Title);

heading_display!(Title);

into_grouping_union!(Title, HeadNode);
