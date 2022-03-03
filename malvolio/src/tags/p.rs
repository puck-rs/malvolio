/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/
use std::{borrow::Cow, collections::HashMap, fmt::Display};

use ammonia::clean;

use super::body::body_node::BodyNode;

use crate::{
    attributes::IntoAttribute,
    into_attribute_for_grouping_enum, into_grouping_union,
    prelude::{Class, Id},
    text::Text,
    utility_enum,
    utils::write_attributes,
};

/// The `<p>` tag.
///
/// See the [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/p) for more
/// info.
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "pub_fields", derive(FieldsAccessibleVariant))]
#[cfg_attr(feature = "fuzz", derive(serde::Serialize, serde::Deserialize))]
#[must_use]
pub struct P {
    pub(crate) attrs: HashMap<Cow<'static, str>, Cow<'static, str>>,
    pub(crate) text: Cow<'static, str>,
    pub(crate) children: Vec<BodyNode>,
}

#[cfg(feature = "fuzz")]
#[cfg_attr(feature = "fuzz", no_coverage)]
mod p {
    use fuzzcheck::{
        mutators::{
            map::MapMutator,
            tuples::{Tuple2Mutator, TupleMutatorWrapper},
        },
        DefaultMutator, Mutator,
    };

    use super::P;

    impl P {
        fn mutator() -> impl Mutator<P> {
            let mutator = TupleMutatorWrapper::new(Tuple2Mutator::new(
                crate::mutators::attr_mutator(),
                crate::mutators::cow_mutator(),
            ));
            MapMutator::new(
                mutator,
                |p: &P| Some((p.attrs.clone(), p.text.clone())),
                |(attrs, text)| P {
                    attrs: attrs.clone(),
                    text: text.clone(),
                    children: vec![],
                },
                |_, c| c,
            )
        }
    }

    impl DefaultMutator for P {
        type Mutator = impl Mutator<P>;

        fn default_mutator() -> Self::Mutator {
            P::mutator()
        }
    }
}

/// Creates a new `P` tag – functionally equivalent to `P::new()` (but easier to type.)
pub fn p() -> P {
    P::default()
}

into_grouping_union!(P, BodyNode);

impl Display for P {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<p ")?;
        write_attributes(&self.attrs, f)?;
        f.write_str(">")?;
        self.text.fmt(f)?;
        for child in &self.children {
            child.fmt(f)?;
        }
        f.write_str("</p>")
    }
}

impl P {
    /// Create a new paragraph with the provided text, sanitising it first.
    pub fn new(text: impl AsRef<str>) -> Self {
        Self::with_text(text)
    }

    /// A method to construct a paragraph containing the supplied text. This will sanitise the text
    /// provided beforehand.
    pub fn with_text<S>(text: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            text: clean(text.as_ref()).into(),
            ..Default::default()
        }
    }

    /// Create a new `<p>` tag, without sanitising the text first.
    pub fn new_unchecked(text: impl Into<Cow<'static, str>>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    /// Attach a new child to this tag.
    pub fn child(mut self, child: impl Into<BodyNode>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Add new children to this tag from an iterator.
    pub fn children(mut self, children: impl IntoIterator<Item = BodyNode>) -> Self {
        self.children.extend(children);
        self
    }

    /// Adds the supplied text to this node, overwriting the previously existing text (if text has
    /// already been added to the node).
    ///
    /// This method sanitises the input (i.e. it escapes HTML);
    /// this might not be what you want – if you are *absolutely certain* that the text you are
    /// providing does not come from a potentially malicious source (e.g. user-supplied text can
    /// contain script tags which will execute unwanted code) you can use `text_unsanitized` which
    /// is identical to this method, except for that it does not sanitise the inputted text (and is
    /// thus slightly faster).
    pub fn text<S>(self, text: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        self.child(BodyNode::Text(Text::new(text.into())))
    }

    /// Adds the supplied text to this node, overwriting the previously existing text (if text has
    /// already been added to the node).
    ///
    /// WARNING: Do not (under any circumstances) use this method with unescaped user-supplied text.
    /// It will be rendered and poses a major security threat to your application. If in doubt, use
    /// the `text` method instead of this one (the risk is much lower that way).
    pub fn text_unsanitized<S>(self, text: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        self.child(BodyNode::Text(Text::new_unchecked(text.into())))
    }

    /// Set the specified attribute on this `P` tag.
    pub fn attribute(mut self, attr: impl Into<PAttr>) -> Self {
        let (key, value) = attr.into().into_attribute();
        self.attrs.insert(key, value);
        self
    }

    /// Read an attribute from this tag, if it exists.
    pub fn read_attribute(&self, key: impl Into<Cow<'static, str>>) -> Option<&Cow<'static, str>> {
        self.attrs.get(&key.into())
    }
}

utility_enum! {
    #[allow(missing_docs)]
    pub enum PAttr {
        Id(Id),
        Class(Class)
    }
}

into_attribute_for_grouping_enum!(PAttr, Id, Class);

into_grouping_union!(Id, PAttr);
into_grouping_union!(Class, PAttr);

#[cfg(test)]
mod test {
    use crate::prelude::*;
    #[test]
    fn test_p() {
        let document = P::with_text("Some text").to_string();
        let document = scraper::Html::parse_document(&document);
        let p = scraper::Selector::parse("p").unwrap();
        let p = document.select(&p).next().unwrap();
        assert_eq!(
            p.children()
                .next()
                .unwrap()
                .value()
                .as_text()
                .unwrap()
                .to_string()
                .as_str(),
            "Some text"
        );
    }

    #[test]
    fn test_p_attrs() {
        let document = P::new("Some text")
            .attribute(Id::new("an-id"))
            .attribute(Class::from("a-class"))
            .to_string();
        let document = scraper::Html::parse_document(&document);
        let p = scraper::Selector::parse("p").unwrap();
        let p = document.select(&p).next().unwrap();
        assert_eq!(
            p.children()
                .next()
                .unwrap()
                .value()
                .as_text()
                .unwrap()
                .to_string()
                .as_str(),
            "Some text"
        );
        let el = p.value();
        assert_eq!(el.id(), Some("an-id"));
        assert_eq!(el.attr("class"), Some("a-class"));
    }
}
