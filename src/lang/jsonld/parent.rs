use enum_methods::{EnumIntoGetters, EnumIsA, EnumToGetters};

use lsp_core::model::Spanned;

use lsp_core::parent::ParentingSystem;

use super::{
    parser::{Json, ObjectMember},
    tokenizer::JsonToken,
};

#[derive(Clone, Debug, PartialEq, EnumIntoGetters, EnumIsA, EnumToGetters)]
pub enum JsonNode {
    Leaf(JsonToken),
    Array(Vec<usize>),
    Object(Vec<usize>),
    Kv(Spanned<String>, usize),
}

#[allow(unused)]
impl JsonNode {
    pub fn as_leaf(&self) -> Option<&JsonToken> {
        match self {
            JsonNode::Leaf(x) => Some(x),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<usize>> {
        match self {
            JsonNode::Array(x) => Some(x),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&Vec<usize>> {
        match self {
            JsonNode::Object(x) => Some(x),
            _ => None,
        }
    }

    pub fn as_kv(&self) -> Option<(&Spanned<String>, &usize)> {
        match self {
            JsonNode::Kv(ref a, ref b) => Some((a, b)),
            _ => None,
        }
    }
}

pub use helpers::to_json_vec;
mod helpers {
    use super::JsonNode;
    use lsp_core::model::Spanned;
    use lsp_core::parent::ParentingSystem;
    use std::io::{self, Cursor, Write};

    type This = ParentingSystem<Spanned<JsonNode>>;

    fn write(this: &This, node: &Spanned<JsonNode>, to_write: &mut impl Write) -> io::Result<()> {
        match node.value() {
            JsonNode::Leaf(node) => {
                write!(to_write, "{}", node)?;
            }
            JsonNode::Array(arr) => {
                write!(to_write, "[")?;
                if let Some(x) = arr.first() {
                    write(this, &this[*x], to_write)?;
                    for i in &arr[1..] {
                        write!(to_write, ",")?;
                        write(this, &this[*i], to_write)?;
                    }
                }
                write!(to_write, "]")?;
            }
            JsonNode::Object(arr) => {
                write!(to_write, "{{")?;
                if let Some(x) = arr.first() {
                    write(this, &this[*x], to_write)?;
                    for i in &arr[1..] {
                        write!(to_write, ",")?;
                        write(this, &this[*i], to_write)?;
                    }
                }
                write!(to_write, "}}")?;
            }
            JsonNode::Kv(s, k) => {
                write!(to_write, "\"{}\":", s.value())?;
                write(this, &this[*k], to_write)?;
            }
        }

        Ok(())
    }

    pub fn to_json_vec(this: &This) -> io::Result<Vec<u8>> {
        let mut cursor = Cursor::new(Vec::new());

        if let Some(x) = this.start_element() {
            write(this, &x, &mut cursor)?;
        }

        Ok(cursor.into_inner())
    }
}

pub fn system(element: Spanned<Json>) -> ParentingSystem<Spanned<JsonNode>> {
    let mut system = ParentingSystem::new();
    system.start = add(element, 0, &mut system);
    system
}

fn add(
    Spanned(el, span): Spanned<Json>,
    parent: usize,
    system: &mut ParentingSystem<Spanned<JsonNode>>,
) -> Option<usize> {
    match el {
        Json::Invalid => return None,
        Json::Array(jsons) => {
            let children: Vec<_> = jsons
                .into_iter()
                .map(|x| add(x, 0, system))
                .flatten()
                .collect();
            let this = system.add(Spanned(JsonNode::Array(children.clone()), span), parent);
            children
                .into_iter()
                .for_each(|i| system.set_parent(i, this));
            return Some(this);
        }
        Json::Object(ks) => {
            let mut children = Vec::new();
            for Spanned(x, kv_span) in ks {
                match x {
                    ObjectMember::Full(k, v) => {
                        if let Some(child) = add(v, 0, system) {
                            let kv = JsonNode::Kv(k.map(|x| x.into_string()), child);
                            let kv_child = system.add(Spanned(kv, kv_span), 0);
                            system.set_parent(child, kv_child);
                            children.push(kv_child);
                        }
                    }
                    _ => {}
                }
            }

            let this = system.add(Spanned(JsonNode::Object(children.clone()), span), parent);
            children
                .into_iter()
                .for_each(|i| system.set_parent(i, this));
            return Some(this);
        }
        Json::Token(i) => {
            let this = system.add(Spanned(JsonNode::Leaf(i), span), parent);
            return Some(this);
        }
    }
}
