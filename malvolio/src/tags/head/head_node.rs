/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/
use crate::{
    enum_display,
    tags::{meta::Meta, style::StyleTag, title::Title},
    utility_enum,
};

utility_enum!(
    #[cfg_attr(feature = "fuzz", derive(serde::Serialize, serde::Deserialize))]
    #[allow(missing_docs)]
    /// A node which can be attached to the `<head>` tag.
    pub enum HeadNode {
        Title(Title),
        Meta(Meta),
        StyleTag(StyleTag),
    }
);

#[cfg(feature = "fuzz")]
#[cfg_attr(feature = "fuzz", no_coverage)]
mod head_node_mutator {
    use fuzzcheck::make_mutator;

    use crate::tags::{meta::Meta, style::StyleTag, title::Title};

    use super::HeadNode;

    make_mutator! {
        name: HeadNodeMutator,
        recursive: false,
        default: true,
        type: pub enum HeadNode {
            Title(Title),
            Meta(Meta),
            StyleTag(StyleTag),
        }
    }
}

enum_display!(HeadNode, Title, Meta, StyleTag);
