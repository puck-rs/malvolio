/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/
use std::{borrow::Cow, collections::HashMap, fmt::Display};

use crate::{
    attributes::IntoAttribute, into_attribute_for_grouping_enum, into_grouping_union, utility_enum,
};

use super::head::head_node::HeadNode;

#[derive(Derivative, Debug, Clone)]
#[derivative(Default(new = "true"))]
#[cfg_attr(feature = "pub_fields", derive(FieldsAccessibleVariant))]
#[cfg_attr(feature = "fuzz", derive(serde::Serialize, serde::Deserialize))]
/// A metadata element. Useful for adding metadata which can not be represented through other HTML
/// tags.
///
/// See [MDN's page on this](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta) for
/// further information.
#[must_use]
pub struct Meta {
    attrs: HashMap<Cow<'static, str>, Cow<'static, str>>,
}

#[cfg(feature = "fuzz")]
#[cfg_attr(feature = "fuzz", no_coverage)]
mod meta_mutator {
    use fuzzcheck::{mutators::map::MapMutator, DefaultMutator, Mutator};

    use super::Meta;

    impl DefaultMutator for Meta {
        type Mutator = impl Mutator<Meta>;

        fn default_mutator() -> Self::Mutator {
            MapMutator::new(
                crate::mutators::attr_mutator(),
                |meta: &Meta| Some(meta.attrs.clone()),
                |attrs| Meta {
                    attrs: attrs.clone(),
                },
                |_, c| c,
            )
        }
    }
}

/// Creates a new `Meta` tag – functionally equivalent to `Meta::new()` (but easier to type.)
pub fn meta() -> Meta {
    Meta::new()
}

impl Meta {
    #[inline(always)]
    /// Add an attribute to this meta tag.
    pub fn attribute<A>(mut self, attr: A) -> Self
    where
        A: Into<MetaAttr>,
    {
        let (a, b) = attr.into().into_attribute();
        self.attrs.insert(a, b);
        self
    }

    /// Read an attribute that has been set
    pub fn read_attribute(&self, attribute: &'static str) -> Option<&Cow<'static, str>> {
        self.attrs.get(attribute)
    }

    crate::define_raw_attribute_fn!();
}

impl Display for Meta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<meta")?;
        for attr in &self.attrs {
            f.write_str(" ")?;
            attr.0.fmt(f)?;
            f.write_str("=\"")?;
            attr.1.fmt(f)?;
            f.write_str("\"")?;
        }
        f.write_str("/>")
    }
}

into_grouping_union!(Meta, HeadNode);

utility_enum!(
    #[allow(missing_docs)]
    pub enum MetaAttr {
        Content(Content),
        MetaName(MetaName),
    }
);

into_attribute_for_grouping_enum!(MetaAttr, Content, MetaName);

/// The "name" attribute for meta tags. This is called `MetaName` to disambiguate it from other
/// tags.
#[derive(Debug, Clone)]

pub enum MetaName {
    /// Specifies the charset of the HTML document. Note that for this to work, you also need to
    /// specify the `content` attribute on the relevant meta tag.
    /// ```
    /// # use malvolio::prelude::*;
    /// Meta::new()
    ///     .attribute(MetaName::Charset)
    ///     .attribute(Content::new("utf-8"));
    /// ```
    Charset,
    /// Sets the name of this `meta` tag to "viewport".
    ///
    /// See the
    /// [relevant MDN page](https://developer.mozilla.org/en-US/docs/Web/HTML/Viewport_meta_tag)
    /// for more information.
    Viewport,
}

impl IntoAttribute for MetaName {
    fn into_attribute(self) -> (Cow<'static, str>, Cow<'static, str>) {
        (
            "name".into(),
            match self {
                MetaName::Charset => "charset",
                MetaName::Viewport => "viewport",
            }
            .into(),
        )
    }
}

into_grouping_union!(MetaName, MetaAttr);

#[derive(Debug, Clone)]

/// The "content" attribute for a `<meta>` tag.
pub struct Content(Cow<'static, str>);

impl Content {
    /// Create a new "content" attribute, which can then be applied to a meta tag.
    pub fn new<C>(c: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        Self(c.into())
    }
}

impl IntoAttribute for Content {
    fn into_attribute(self) -> (Cow<'static, str>, Cow<'static, str>) {
        ("content".into(), self.0)
    }
}

into_grouping_union!(Content, MetaAttr);

#[cfg(test)]
mod test {
    use crate::prelude::*;
    #[test]
    fn test_a_with_attributes() {
        let document = Meta::default()
            .attribute(MetaName::Charset)
            .attribute(Content::new("utf-8"))
            .to_string();
        let document = scraper::Html::parse_document(&document);
        let a = scraper::Selector::parse("meta").unwrap();
        let a = document.select(&a).next().unwrap().value();
        assert_eq!(a.attr("name").unwrap(), "charset");
    }
}
