use super::LocalPrefix;

pub const LOCAL_PREFIXES: &'static [LocalPrefix] = &[
    LocalPrefix {
        location: "http://xmlns.com/foaf/0.1/",
        content: include_str!("../prefixes/foaf.ttl"),
        name: "foaf",
        title: "Friend of a Friend vocabulary",
    },
    LocalPrefix {
        location: "https://w3id.org/tree#",
        content: include_str!("../prefixes/tree.ttl"),
        name: "tree",
        title: "TREE",
    },
    LocalPrefix {
        location: "http://w3id.org/rml/core#",
        content: include_str!("../prefixes/rml.ttl"),
        name: "rml",
        title: "RML: Generic Mapping Language for RDF",
    },
    LocalPrefix {
        location: "http://w3id.org/rml/cc/",
        content: include_str!("../prefixes/rml-cc.ttl"),
        name: "rml-cc",
        title: "RML-Containers",
    },
    LocalPrefix {
        location: "http://w3id.org/rml/fnml/",
        content: include_str!("../prefixes/rml-fnml.ttl"),
        name: "rml-fnml",
        title: "RML-FNML",
    },
    LocalPrefix {
        location: "http://w3id.org/rml/io/",
        content: include_str!("../prefixes/rml-io.ttl"),
        name: "rml-io",
        title: "RML-IO: Source and Target",
    },
    LocalPrefix {
        location: "http://w3id.org/rml/star/",
        content: include_str!("../prefixes/rml-star.ttl"),
        name: "rml-star",
        title: "RML-star",
    },
    LocalPrefix {
        location: "http://www.w3.org/2002/07/owl#",
        content: include_str!("../prefixes/owl.ttl"),
        name: "owl",
        title: "The OWL 2 Schema vocabulary",
    },
    LocalPrefix {
        location: "http://www.w3.org/2000/01/rdf-schema#",
        content: include_str!("../prefixes/rdfs.ttl"),
        name: "rdfs",
        title: "The RDF Schema vocabulary",
    },
    LocalPrefix {
        location: "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
        content: include_str!("../prefixes/rdf.ttl"),
        name: "rdf",
        title: "The RDF Concepts Vocabulary",
    },
];
