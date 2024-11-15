use lsp_types::CompletionItemKind;

#[derive(Default, Clone, Copy, Debug)]
#[allow(unused)]
pub enum PropertyType {
    Class,
    DatatypeProperty,
    ObjectProperty,
    #[default]
    Property,
}

impl Into<CompletionItemKind> for PropertyType {
    fn into(self) -> CompletionItemKind {
        match self {
            PropertyType::Class => CompletionItemKind::CLASS,
            PropertyType::DatatypeProperty => CompletionItemKind::PROPERTY,
            PropertyType::ObjectProperty => CompletionItemKind::PROPERTY,
            PropertyType::Property => CompletionItemKind::PROPERTY,
        }
    }
}
