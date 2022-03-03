/*
This source code file is distributed subject to the terms of the Mozilla Public License v2.0.
A copy of this license can be found in the `licenses` directory at the root of this project.
*/
use crate::{
    enum_display,
    tags::{
        a::A,
        br::Br,
        div::Div,
        form::Form,
        headings::{H1, H2, H3, H4, H5, H6},
        img::Img,
        input::Input,
        label::Label,
        noscript::NoScript,
        p::P,
        select::Select,
    },
    text::Text,
    utility_enum,
};

utility_enum!(
    #[allow(missing_docs)]
    #[cfg_attr(feature = "fuzz", derive(serde::Serialize, serde::Deserialize))]
    /// A node which can be mounted to the `<body>` tag (or any of its children).
    pub enum BodyNode {
        H1(H1),
        H2(H2),
        H3(H3),
        H4(H4),
        H5(H5),
        H6(H6),
        P(P),
        Text(Text),
        Form(Form),
        Br(Br),
        Div(Div),
        A(A),
        Input(Input),
        Label(Label),
        Select(Select),
        NoScript(NoScript),
        Img(Img),
    }
);

#[allow(missing_docs)]
#[cfg(feature = "fuzz")]
#[cfg_attr(feature = "fuzz", no_coverage)]
/// Just look away, look away.
/// This code is nothing but dismay.
mod body_mutator {
    use std::borrow::Cow;

    use super::*;
    use fuzzcheck::{
        make_mutator,
        mutators::{map::MapMutator, recursive::RecurToMutator, vector::VecMutator},
        DefaultMutator, Mutator,
    };

    impl BodyNode {
        fn mutator() -> impl Mutator<BodyNode> {
            MapMutator::new(
                BranchNode::default_mutator(),
                |node: &BodyNode| Some(BranchNode::from_body_node(node.clone())),
                |leaf_or_branch: &BranchNode| leaf_or_branch.clone().into_body_node(),
                |_, c| c,
            )
        }
    }

    impl DefaultMutator for BodyNode {
        type Mutator = impl Mutator<BodyNode>;

        fn default_mutator() -> Self::Mutator {
            BodyNode::mutator()
        }
    }

    #[derive(Clone)]
    struct Attr {
        name: Attr1String,
        value: Attr0String,
    }

    #[derive(Clone)]
    struct Attr0String(String);

    impl Attr0String {
        fn mutator() -> impl Mutator<Self> {
            MapMutator::new(
                crate::mutators::valid_attr_string_mutator::<0>(),
                |attr: &Attr0String| Some(attr.0.clone()),
                |string| Self(string.clone()),
                |_, c| c,
            )
        }
    }

    impl DefaultMutator for Attr0String {
        type Mutator = impl Mutator<Self>;

        fn default_mutator() -> Self::Mutator {
            Self::mutator()
        }
    }

    #[derive(Clone)]
    struct Attr1String(String);

    impl Attr1String {
        fn mutator() -> impl Mutator<Self> {
            MapMutator::new(
                crate::mutators::valid_attr_string_mutator::<1>(),
                |attr: &Attr1String| Some(attr.0.clone()),
                |string| Self(string.clone()),
                |_, c| c,
            )
        }
    }

    impl DefaultMutator for Attr1String {
        type Mutator = impl Mutator<Self>;

        fn default_mutator() -> Self::Mutator {
            Self::mutator()
        }
    }

    make_mutator! {
        name: AttrMutator,
        recursive: false,
        default: false,
        type:
            struct Attr {
                name: Attr1String,
                value: Attr0String
            }
    }

    #[derive(Clone)]
    enum BranchNode {
        P {
            attrs: Vec<(Attr1String, Attr0String)>,
            // todo: check whatwg spec
            text: Attr0String,
            children: Vec<BranchNode>,
        },
        Form {
            children: Vec<BranchNode>,
            attrs: Vec<(Attr1String, Attr0String)>,
        },
        Div {
            attrs: Vec<(Attr1String, Attr0String)>,
            children: Vec<BranchNode>,
        },
        LeafNode(LeafNode),
    }

    impl BranchNode {
        fn from_body_node(node: BodyNode) -> Self {
            match node {
                node @ BodyNode::H1(_)
                | node @ BodyNode::H2(_)
                | node @ BodyNode::H3(_)
                | node @ BodyNode::H4(_)
                | node @ BodyNode::H5(_)
                | node @ BodyNode::H6(_)
                | node @ BodyNode::Text(_)
                | node @ BodyNode::A(_)
                | node @ BodyNode::Input(_)
                | node @ BodyNode::Label(_)
                | node @ BodyNode::Select(_)
                | node @ BodyNode::NoScript(_)
                | node @ BodyNode::Img(_)
                | node @ BodyNode::Br(_) => Self::LeafNode(LeafNode::from_body_node(node)),
                BodyNode::Div(div) => Self::Div {
                    attrs: div
                        .attrs
                        .into_iter()
                        .map(|(a, b)| (Attr1String(a.to_string()), Attr0String(b.to_string())))
                        .collect(),
                    children: div
                        .children
                        .into_iter()
                        .map(BranchNode::from_body_node)
                        .collect(),
                },
                BodyNode::Form(form) => Self::Form {
                    attrs: form
                        .attrs
                        .into_iter()
                        .map(|(a, b)| (Attr1String(a.to_string()), Attr0String(b.to_string())))
                        .collect(),
                    children: form
                        .children
                        .into_iter()
                        .map(BranchNode::from_body_node)
                        .collect(),
                },
                BodyNode::P(p) => Self::P {
                    attrs: p
                        .attrs
                        .into_iter()
                        .map(|(a, b)| (Attr1String(a.to_string()), Attr0String(b.to_string())))
                        .collect(),
                    text: Attr0String(p.text.to_string()),
                    children: p
                        .children
                        .into_iter()
                        .map(BranchNode::from_body_node)
                        .collect(),
                },
            }
        }

        fn into_body_node(self) -> BodyNode {
            match self {
                BranchNode::Div { attrs, children } => BodyNode::Div(Div {
                    children: children
                        .into_iter()
                        .map(BranchNode::into_body_node)
                        .collect(),
                    attrs: attrs
                        .into_iter()
                        .map(|(a, b)| (Cow::Owned(a.0), Cow::Owned(b.0)))
                        .collect(),
                }),
                BranchNode::Form { attrs, children } => BodyNode::Form(Form {
                    children: children
                        .into_iter()
                        .map(BranchNode::into_body_node)
                        .collect(),
                    attrs: attrs
                        .into_iter()
                        .map(|(a, b)| (Cow::Owned(a.0), Cow::Owned(b.0)))
                        .collect(),
                }),
                BranchNode::LeafNode(leaf) => leaf.into_body_node(),
                BranchNode::P {
                    attrs,
                    text,
                    children,
                } => BodyNode::P(P {
                    attrs: attrs
                        .into_iter()
                        .map(|(a, b)| (Cow::Owned(a.0), Cow::Owned(b.0)))
                        .collect(),
                    text: Cow::Owned(text.0),
                    children: children
                        .into_iter()
                        .map(BranchNode::into_body_node)
                        .collect(),
                }),
            }
        }
    }

    make_mutator! {
        name: BranchNodeMutator,
        recursive: true,
        default: true,
        type: enum BranchNode {
                Div {
                    attrs: Vec<(Attr1String, Attr0String)>,
                    #[field_mutator(
                        VecMutator<BranchNode, RecurToMutator<BranchNodeMutator<M0_0, M1_1, M2_0, M2_1, M3_0>>> = {
                            VecMutator::new(self_.into(), 0..=usize::MAX)
                        }
                    )]
                    children: Vec<BranchNode>
                },
                Form {
                    #[field_mutator(
                        VecMutator<BranchNode, RecurToMutator<BranchNodeMutator<M0_0, M1_1, M2_0, M2_1, M3_0>>> = {
                            VecMutator::new(self_.into(), 0..=usize::MAX)
                        }
                    )]
                    children: Vec<BranchNode>,
                    attrs: Vec<(Attr1String, Attr0String)>,
                },
                P {
                    attrs: Vec<(Attr1String, Attr0String)>,
                    text: Attr0String,
                    #[field_mutator(
                        VecMutator<BranchNode, RecurToMutator<BranchNodeMutator<M0_0, M1_1, M2_0, M2_1, M3_0>>> = {
                            VecMutator::new(self_.into(), 0..=usize::MAX)
                        }
                    )]
                    children: Vec<BranchNode>,
                },
                LeafNode(LeafNode),
            }
    }

    #[derive(Clone)]
    enum LeafNode {
        H1(H1),
        H2(H2),
        H3(H3),
        H4(H4),
        H5(H5),
        H6(H6),
        Text(Text),
        Br(Br),
        A(A),
        Input(Input),
        Label(Label),
        NoScript(NoScript),
        Img(Img),
    }

    make_mutator! {
        name: LeafNodeMutator,
        recursive: false,
        default: true,
        type: enum LeafNode {
            H1(H1),
            H2(H2),
            H3(H3),
            H4(H4),
            H5(H5),
            H6(H6),
            Br(Br),
            A(A),
            Input(Input),
            Label(Label),
            NoScript(NoScript),
            Img(Img),
        }
    }

    impl LeafNode {
        fn from_body_node(node: BodyNode) -> Self {
            match node {
                BodyNode::H1(h1) => Self::H1(h1),
                BodyNode::H2(h2) => Self::H2(h2),
                BodyNode::H3(h3) => Self::H3(h3),
                BodyNode::H4(h4) => Self::H4(h4),
                BodyNode::H5(h5) => Self::H5(h5),
                BodyNode::H6(h6) => Self::H6(h6),
                BodyNode::Text(text) => Self::Text(text),
                BodyNode::Br(br) => Self::Br(br),
                BodyNode::A(a) => Self::A(a),
                BodyNode::Input(input) => Self::Input(input),
                BodyNode::Label(label) => Self::Label(label),
                BodyNode::NoScript(script) => Self::NoScript(script),
                BodyNode::Img(img) => Self::Img(img),
                _ => unreachable!(),
            }
        }

        fn into_body_node(self) -> BodyNode {
            match self {
                LeafNode::H1(h1) => BodyNode::H1(h1),
                LeafNode::H2(h2) => BodyNode::H2(h2),
                LeafNode::H3(h3) => BodyNode::H3(h3),
                LeafNode::H4(h4) => BodyNode::H4(h4),
                LeafNode::H5(h5) => BodyNode::H5(h5),
                LeafNode::H6(h6) => BodyNode::H6(h6),
                LeafNode::Text(t) => BodyNode::Text(t),
                LeafNode::Br(b) => BodyNode::Br(b),
                LeafNode::A(a) => BodyNode::A(a),
                LeafNode::Input(i) => BodyNode::Input(i),
                LeafNode::Label(l) => BodyNode::Label(l),
                LeafNode::NoScript(n) => BodyNode::NoScript(n),
                LeafNode::Img(i) => BodyNode::Img(i),
            }
        }
    }
}

enum_display!(
    BodyNode, H1, H2, H3, H4, H5, H6, P, Br, Text, Form, Div, A, Input, Select, NoScript, Img,
    Label
);

#[allow(missing_docs)]
impl BodyNode {
    pub fn as_h1(&self) -> Option<&H1> {
        if let Self::H1(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_h2(&self) -> Option<&H2> {
        if let Self::H2(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_h3(&self) -> Option<&H3> {
        if let Self::H3(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_h4(&self) -> Option<&H4> {
        if let Self::H4(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_h5(&self) -> Option<&H5> {
        if let Self::H5(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_h6(&self) -> Option<&H6> {
        if let Self::H6(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_p(&self) -> Option<&P> {
        if let Self::P(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_text(&self) -> Option<&Text> {
        if let Self::Text(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_form(&self) -> Option<&Form> {
        if let Self::Form(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_br(&self) -> Option<&Br> {
        if let Self::Br(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_div(&self) -> Option<&Div> {
        if let Self::Div(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_a(&self) -> Option<&A> {
        if let Self::A(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_input(&self) -> Option<&Input> {
        if let Self::Input(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_label(&self) -> Option<&Label> {
        if let Self::Label(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_select(&self) -> Option<&Select> {
        if let Self::Select(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_no_script(&self) -> Option<&NoScript> {
        if let Self::NoScript(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_img(&self) -> Option<&Img> {
        if let Self::Img(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
