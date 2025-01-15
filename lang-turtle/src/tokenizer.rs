use chumsky::prelude::*;
use lsp_core::{
    prelude::{spanned, Spanned},
    token::Token,
};
use token_helpers::*;

pub fn parse_token() -> t!(Token) {
    choice((
        keywords(),
        comment(),
        iri_ref(),
        pname_ns(),
        blank_node_label(),
        lang_tag(),
        integer(),
        strings(),
        tokens(),
    ))
    .recover_with(skip_parser(invalid()))
}

pub fn parse_tokens() -> t!(Vec<Spanned<Token>>) {
    parse_token()
        .map_with_span(spanned)
        .padded()
        .repeated()
        .then_ignore(end().recover_with(skip_then_retry_until([])))
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;

    #[allow(unused)]
    fn log_parse_results(tokens: Vec<(Token, Range<usize>)>, err: &Vec<Simple<char>>) {
        tokens.iter().for_each(|tk| {
            println!("{:?}", tk);
        });

        err.iter().for_each(|er| eprintln!("{:?}", er));
    }

    #[test]
    fn parse_directives() {
        let input = "
            @prefix elm: <http://elm.com/types#> .
            @prefix : <http://elm.com/types#> .
            @base <http://example.com/#> . 
            # Test comment!
        ";

        assert!(parse_tokens().parse(input).is_ok());
    }

    #[test]
    fn parse_named_node() {
        let input = "
            <http://localhost/elmBeta> 
            elm:Beta

            :testing
            ";

        let (tok, err) = parse_tokens().parse_recovery(input);
        assert!(tok.is_some());
        assert!(err.is_empty());
    }

    #[test]
    fn simple_test() {
        let input = "
            @prefix elm: <http://elm.com/types#> .
            @base <http://example.com/#> . 
            
            elm:Beta foaf:string \"cookie\"@en ;
                     foaf:astring \"jar\"^^xsd:string .

            elm:Bnode a [ foaf:name \"Kachan\" ; 
                          foaf:lastName \"Bruh\" ; 
                          foaf:email \"kb@kbc.be\", \"notkb@notkbc.be\" ].
            ";

        let (tok, err) = parse_tokens().parse_recovery(input);
        assert!(tok.is_some());
        assert!(err.is_empty());
    }

    #[test]
    fn complex_test() {
        let input = "
            @prefix rr: <http://www.w3.org/ns/r2rml#> .
            @prefix foaf: <http://xmlns.com/foaf/0.1/> .
            @prefix ex: <http://example.com/> .
            @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
            @prefix rml: <http://semweb.mmlab.be/ns/rml#> .
            @prefix ql: <http://semweb.mmlab.be/ns/ql#> .

            @base <http://example.com/base/> .

            <TriplesMap1>
              a rr:TriplesMap;
                    
              rml:logicalSource [ 
                rml:source \"student.csv\";
                rml:referenceFormulation ql:CSV
              ] ;
                
              rr:subjectMap [ 
                rr:template \"http://example.com/{Name}\" 
              ]; 
                
              rr:predicateObjectMap [ 
                rr:predicate foaf:name ; 
                rr:objectMap [ 
                  rml:reference \"Name\" 
                ]
              ].
            ";

        let (tok, err) = parse_tokens().parse_recovery(input);
        assert!(tok.is_some());
        assert!(err.is_empty());
    }

    #[test]
    fn parse_invalid() {
        let input = "
            @prefix elm: http .
            ";

        let (tok, err) = parse_tokens().parse_recovery(input);
        assert!(tok.is_some());

        println!("tokens {:?}", tok);
        assert_eq!(tok.unwrap().len(), 4);
        assert_eq!(err.len(), 1);
    }
}
