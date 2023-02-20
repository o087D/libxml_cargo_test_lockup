use libxml::parser::Parser;
use libxml::schemas::{SchemaParserContext, SchemaValidationContext};

fn main() {
    println!("Hello, world!");
}

pub fn do_xml_things(schema: &str, xml: &str) {
    let mut schema_parser = SchemaParserContext::from_buffer(schema);
    let mut xsd = SchemaValidationContext::from_parser(&mut schema_parser).unwrap();

    let doc = Parser::default().parse_string(xml).unwrap();
    xsd.validate_document(&doc).unwrap();
}

#[cfg(test)]
mod test {

    use super::*;

    const SCHEMA: &str = r#"<?xml version="1.0"?>
    <xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
    
    <xs:element name="note">
      <xs:complexType>
        <xs:sequence>
          <xs:element name="to" type="xs:string"/>
          <xs:element name="from" type="xs:string"/>
          <xs:element name="heading" type="xs:string"/>
          <xs:element name="body" type="xs:string"/>
        </xs:sequence>
      </xs:complexType>
    </xs:element>
    
    </xs:schema>"#;

    #[test]
    fn test_a() {
        do_xml_things(
            SCHEMA,
            "<note><to>Anyone</to><from>Me</from><heading /><body /></note>",
        );
    }

    #[test]
    fn test_b() {
        do_xml_things(
            SCHEMA,
            "<note><to>Anyone</to><from>Me</from><heading /><body /></note>",
        );
    }

    #[test]
    fn test_c() {
        do_xml_things(
            SCHEMA,
            "<note><to>Anyone</to><from>Me</from><heading /><body /></note>",
        );
    }

    #[test]
    fn test_d() {
        do_xml_things(
            SCHEMA,
            "<note><to>Anyone</to><from>Me</from><heading /><body /></note>",
        );
    }

    #[test]
    fn test_e() {
        do_xml_things(
            SCHEMA,
            "<note><to>Anyone</to><from>Me</from><heading /><body /></note>",
        );
    }
}
