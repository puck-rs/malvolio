/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/
use std::{borrow::Cow, collections::HashMap, fmt::Display};

use self::body_node::BodyNode;
use super::headings::{H1, H2, H3, H4, H5, H6};
use crate::attributes::IntoAttribute;
use crate::{into_attribute_for_grouping_enum, into_grouping_union, prelude::Style, utility_enum};

/// Contains the `BodyNode` enum.
pub mod body_node;

#[derive(Derivative, Debug, Clone)]
#[derivative(Default(new = "true"))]
#[cfg_attr(feature = "pub_fields", derive(FieldsAccessibleVariant))]
#[cfg_attr(feature = "fuzz", derive(serde::Serialize, serde::Deserialize))]
/// The `<body>` tag.
#[must_use]
pub struct Body {
    children: Vec<BodyNode>,
    attrs: HashMap<Cow<'static, str>, Cow<'static, str>>,
}

#[cfg(feature = "fuzz")]
#[cfg_attr(feature = "fuzz", no_coverage)]
mod body_mutator {
    use fuzzcheck::{
        mutators::{
            map::MapMutator,
            tuples::{Tuple2Mutator, TupleMutatorWrapper},
            vector::VecMutator,
        },
        DefaultMutator, Mutator,
    };

    use super::{body_node::BodyNode, Body};

    impl DefaultMutator for Body {
        type Mutator = impl Mutator<Body>;

        fn default_mutator() -> Self::Mutator {
            let tuple = Tuple2Mutator::new(
                VecMutator::new(BodyNode::default_mutator(), 0..=usize::MAX),
                crate::mutators::attr_mutator(),
            );
            let tuple = TupleMutatorWrapper::new(tuple);

            MapMutator::new(
                tuple,
                |body: &Body| Some((body.children.clone(), body.attrs.clone())),
                |(children, attrs)| Body {
                    children: children.clone(),
                    attrs: attrs.clone(),
                },
                |_, c| c,
            )
        }
    }
}

/// Creates a new `Body` tag – functionally equivalent to `Body::new()` (but shorter to type.)
pub fn body() -> Body {
    Body::new()
}

utility_enum! {
    pub enum BodyAttr {
        /// Add a "style" attribute to this item.
        Style(Style),
    }
}

into_grouping_union!(Style, BodyAttr);
into_attribute_for_grouping_enum!(BodyAttr, Style);

impl Body {
    /// Attach multiple children to this tag, from an iterator of items implementing
    /// `Into<BodyNode>`
    pub fn children<I, C>(mut self, children: I) -> Self
    where
        C: Into<BodyNode>,
        I: IntoIterator<Item = C>,
    {
        self.children
            .extend(children.into_iter().map(Into::into).collect::<Vec<_>>());
        self
    }
    /// Attach a single child to this tag.
    pub fn child<C>(mut self, child: C) -> Self
    where
        C: Into<BodyNode>,
    {
        self.children.push(child.into());
        self
    }
    /// Apply a function to this tag.
    pub fn map<F>(self, mapping: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        mapping(self)
    }
    /// Add a new attribute to this tag.
    pub fn attribute<A>(mut self, attribute: A) -> Self
    where
        A: Into<BodyAttr>,
    {
        let (a, b) = attribute.into().into_attribute();
        self.attrs.insert(a, b);
        self
    }
    /// Read an attribute that has been set
    pub fn read_attribute(&self, attribute: &'static str) -> Option<&Cow<'static, str>> {
        self.attrs.get(attribute)
    }
    /// Attach a new `H1` instance to this class. Note that this method only allows you to provide
    /// text, and no additional attributes. If you want to specify extra attributes, you should
    /// instead use the "child" method (see the documentation of that method for more details).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().h1("Hello World!");
    /// ```
    ///
    /// This is just a convenience wrapper, and is functionally equivalent to calling the `child`
    /// method (but somewhat more concise).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().child(H1::new("Hello World!"));
    /// ```
    pub fn h1<C>(self, c: C) -> Self
    where
        C: Into<H1>,
    {
        self.child(c.into())
    }
    /// Attach a new `H2` instance to this class. Note that this method only allows you to provide
    /// text, and no additional attributes. If you want to specify extra attributes, you should
    /// instead use the "child" method (see the documentation of that method for more details).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().h2("Hello World!");
    /// ```
    ///
    /// This is just a convenience wrapper, and is functionally equivalent to calling the `child`
    /// method (but somewhat more concise).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().child(H2::new("Hello World!"));
    /// ```
    pub fn h2<C>(self, c: C) -> Self
    where
        C: Into<H2>,
    {
        self.child(c.into())
    }
    /// Attach a new `H3` instance to this class. Note that this method only allows you to provide
    /// text, and no additional attributes. If you want to specify extra attributes, you should
    /// instead use the "child" method (see the documentation of that method for more details).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().h3("Hello World!");
    /// ```
    ///
    /// This is just a convenience wrapper, and is functionally equivalent to calling the `child`
    /// method (but somewhat more concise).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().child(H3::new("Hello World!"));
    /// ```
    pub fn h3<C>(self, c: C) -> Self
    where
        C: Into<H3>,
    {
        self.child(c.into())
    }
    /// Attach a new `H4` instance to this class. Note that this method only allows you to provide
    /// text, and no additional attributes. If you want to specify extra attributes, you should
    /// instead use the "child" method (see the documentation of that method for more details).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().h4("Hello World!");
    /// ```
    ///
    /// This is just a convenience wrapper, and is functionally equivalent to calling the `child`
    /// method (but somewhat more concise).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().child(H4::new("Hello World!"));
    /// ```
    pub fn h4<C>(self, c: C) -> Self
    where
        C: Into<H4>,
    {
        self.child(c.into())
    }
    /// Attach a new `H5` instance to this class. Note that this method only allows you to provide
    /// text, and no additional attributes. If you want to specify extra attributes, you should
    /// instead use the "child" method (see the documentation of that method for more details).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().h5("Hello World!");
    /// ```
    ///
    /// This is just a convenience wrapper, and is functionally equivalent to calling the `child`
    /// method (but somewhat more concise).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().child(H6::new("Hello World!"));
    /// ```
    pub fn h5<C>(self, c: C) -> Self
    where
        C: Into<H5>,
    {
        self.child(c.into())
    }
    /// Attach a new `H6` instance to this class. Note that this method only allows you to provide
    /// text, and no additional attributes. If you want to specify extra attributes, you should
    /// instead use the "child" method (see the documentation of that method for more details).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().h6("Hello World!");
    /// ```
    ///
    /// This is just a convenience wrapper, and is functionally equivalent to calling the `child`
    /// method (but somewhat more concise).
    ///
    /// ```rust
    /// # use malvolio::prelude::*;
    /// Body::new().child(H6::new("Hello World!"));
    /// ```
    pub fn h6<C>(self, c: C) -> Self
    where
        C: Into<H6>,
    {
        self.child(c.into())
    }
}

impl Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<body")?;
        for attr in &self.attrs {
            f.write_str(" ")?;
            attr.0.fmt(f)?;
            f.write_str("=\"")?;
            attr.1.fmt(f)?;
            f.write_str("\"")?;
        }
        f.write_str(">")?;
        for node in &self.children {
            node.fmt(f)?;
        }
        f.write_str("</body>")
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    #[test]
    fn test_children() {
        use crate::prelude::*;
        let document = Body::new()
            .children(
                vec!["1", "2", "3"]
                    .into_iter()
                    .map(|item| H1::new(item).attribute(Id::new(item))),
            )
            .to_string();
        let document = scraper::Html::parse_document(&document);
        let h1_selector = scraper::Selector::parse("h1").unwrap();
        let h1s = document.select(&h1_selector).collect::<Vec<_>>();
        assert_eq!(h1s.len(), 3);
        assert_eq!(h1s[0].value().attr("id"), Some("1"));
        assert_eq!(
            h1s[0]
                .first_child()
                .unwrap()
                .value()
                .as_text()
                .map(Deref::deref),
            Some("1")
        );
        assert_eq!(h1s[1].value().attr("id"), Some("2"));
        assert_eq!(
            h1s[1]
                .first_child()
                .unwrap()
                .value()
                .as_text()
                .map(Deref::deref),
            Some("2")
        );
        assert_eq!(h1s[2].value().attr("id"), Some("3"));
        assert_eq!(
            h1s[2]
                .first_child()
                .unwrap()
                .value()
                .as_text()
                .map(Deref::deref),
            Some("3")
        );
    }
}
