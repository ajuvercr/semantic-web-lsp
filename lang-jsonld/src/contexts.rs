use json_ld::Nullable;

use json_ld_syntax::context::term_definition::TermDefinition;
use json_ld_syntax::context::{definition, FragmentRef};

fn get_definition_ref<'a>(re: &definition::FragmentRef<'a>) -> Option<Definition> {
    match re {
        definition::FragmentRef::Entry(definition::EntryRef::Definition(
            key,
            Nullable::Some(value),
        )) => {
            let binding = match value {
                TermDefinition::Simple(s) => s.as_str().to_string(),
                TermDefinition::Expanded(e) => {
                    e.id.as_ref()
                        .unwrap()
                        .as_ref()
                        .unwrap()
                        .as_str()
                        .to_string()
                }
            };
            Some(Definition {
                key: key.as_str().to_string(),
                value: binding,
            })
        }
        _ => None,
    }
}

pub fn filter_definition<'a>(frag: FragmentRef<'a>) -> Option<Definition> {
    match frag {
        json_ld_syntax::context::FragmentRef::DefinitionFragment(x) => get_definition_ref(&x),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct Definition {
    pub key: String,
    pub value: String,
}
