/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/

use std::{borrow::Cow, collections::HashMap};

use crate::{
    heading_display, impl_of_heading_mutator, impl_of_heading_new_fn,
    into_attribute_for_grouping_enum, into_grouping_union,
    prelude::{Class, Id, Style},
    utility_enum,
};

use super::body::body_node::BodyNode;

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "pub_fields", derive(FieldsAccessibleVariant))]
#[cfg_attr(feature = "fuzz", derive(serde::Serialize, serde::Deserialize))]
/// The `<h1>` tag.
///
/// See
/// [MDN's page on this](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Heading_Elements)
/// for further information.
#[must_use]
pub struct H1 {
    text: Cow<'static, str>,
    attrs: HashMap<Cow<'static, str>, Cow<'static, str>>,
}

impl_of_heading_new_fn!(H1, h1);

impl_of_heading_mutator!(H1);

into_grouping_union!(H1, BodyNode);

heading_display!(H1);

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "pub_fields", derive(FieldsAccessibleVariant))]
#[cfg_attr(feature = "fuzz", derive(serde::Serialize, serde::Deserialize))]
/// The `<h2>` tag.
///
/// See
/// [MDN's page on this](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Heading_Elements)
/// for further information.
#[must_use]
pub struct H2 {
    text: Cow<'static, str>,
    attrs: HashMap<Cow<'static, str>, Cow<'static, str>>,
}

impl_of_heading_new_fn!(H2, h2);

impl_of_heading_mutator!(H2);

into_grouping_union!(H2, BodyNode);

heading_display!(H2);

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "pub_fields", derive(FieldsAccessibleVariant))]
#[cfg_attr(feature = "fuzz", derive(serde::Serialize, serde::Deserialize))]
/// The `<h3>` tag.
///
/// See
/// [MDN's page on this](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Heading_Elements)
/// for further information.
#[must_use]
pub struct H3 {
    text: Cow<'static, str>,
    attrs: HashMap<Cow<'static, str>, Cow<'static, str>>,
}

impl_of_heading_new_fn!(H3, h3);

impl_of_heading_mutator!(H3);

into_grouping_union!(H3, BodyNode);

heading_display!(H3);

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "pub_fields", derive(FieldsAccessibleVariant))]
#[cfg_attr(feature = "fuzz", derive(serde::Serialize, serde::Deserialize))]
/// The `<h4>` tag.
///
/// See
/// [MDN's page on this](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Heading_Elements)
/// for further information.
#[must_use]
pub struct H4 {
    text: Cow<'static, str>,
    attrs: HashMap<Cow<'static, str>, Cow<'static, str>>,
}

impl_of_heading_new_fn!(H4, h4);

impl_of_heading_mutator!(H4);

into_grouping_union!(H4, BodyNode);

heading_display!(H4);

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "pub_fields", derive(FieldsAccessibleVariant))]
#[cfg_attr(feature = "fuzz", derive(serde::Serialize, serde::Deserialize))]
/// The `<h5>` tag.
///
/// See
/// [MDN's page on this](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Heading_Elements)
/// for further information.
#[must_use]
pub struct H5 {
    text: Cow<'static, str>,
    attrs: HashMap<Cow<'static, str>, Cow<'static, str>>,
}

impl_of_heading_new_fn!(H5, h5);

impl_of_heading_mutator!(H5);

into_grouping_union!(H5, BodyNode);

heading_display!(H5);

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "pub_fields", derive(FieldsAccessibleVariant))]
#[cfg_attr(feature = "fuzz", derive(serde::Serialize, serde::Deserialize))]
/// The `<h6>` tag.
///
/// See
/// [MDN's page on this](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Heading_Elements)
/// for further information.
#[must_use]
pub struct H6 {
    text: Cow<'static, str>,
    attrs: HashMap<Cow<'static, str>, Cow<'static, str>>,
}

impl_of_heading_new_fn!(H6, h6);

impl_of_heading_mutator!(H6);

into_grouping_union!(H6, BodyNode);

heading_display!(H6);

utility_enum!(
    /// An attribute for a heading tag.
    #[allow(missing_docs)]
    #[must_use]
    pub enum HeadingAttr {
        Class(Class),
        Id(Id),
        Style(Style),
    }
);
into_attribute_for_grouping_enum!(HeadingAttr, Class, Id, Style);

into_grouping_union!(Class, HeadingAttr);

into_grouping_union!(Id, HeadingAttr);

into_grouping_union!(Style, HeadingAttr);

#[test]
fn test_headings() {
    use crate::prelude::*;
    let document = Html::default()
        .head(Head::default().child(Title::new("Some title")))
        .body(
            Body::default()
                .child(H6::new("Some heading"))
                .child(H6::new("Some other heading"))
                .child(H5::new("Some other other heading"))
                .child(
                    H4::new("Some other other other heading")
                        .attribute(Class::from(Cow::Borrowed("heading-class")))
                        .raw_attribute("raw-attr-key", "raw-attr-value"),
                ),
        )
        .to_string();
    let document = scraper::Html::parse_document(&document);
    let h6_selector = scraper::Selector::parse("h6").unwrap();
    let h5_selector = scraper::Selector::parse("h5").unwrap();
    let h4_selector = scraper::Selector::parse("h4").unwrap();
    assert_eq!(document.select(&h6_selector).count(), 2);
    assert_eq!(document.select(&h5_selector).count(), 1);
    assert_eq!(
        document
            .select(&h6_selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap(),
        "Some heading"
    );
    assert_eq!(
        document
            .select(&h5_selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap(),
        "Some other other heading"
    );
    let h4 = document.select(&h4_selector).next().unwrap();
    assert_eq!(h4.text().next().unwrap(), "Some other other other heading");
    assert_eq!(h4.value().attr("class").unwrap(), "heading-class");
    assert_eq!(h4.value().attr("raw-attr-key").unwrap(), "raw-attr-value");
}
