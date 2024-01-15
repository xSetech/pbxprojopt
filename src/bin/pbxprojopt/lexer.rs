//! .pbxproj lexer derivation

use pest_derive::Parser as PestParser;

#[derive(PestParser)]
#[grammar = "pbxproj.pest"]
pub struct PBXProjLexer;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser as PestParser;

    macro_rules! test_case_lexable {
        ($content:expr) => {{
            let r = PBXProjLexer::parse(
                Rule::File,
                $content,
            );
            r.unwrap();
        }};
    }

    macro_rules! test_case_unlexable {
        ($content:expr) => {{
            let r = PBXProjLexer::parse(
                Rule::File,
                $content,
            );
            assert!(r.is_err());
        }};
    }

    /// Case:  An empty file is not parsable
    #[test]
    pub fn test_empty_file() {
        test_case_unlexable!("");
    }

    #[test]
    pub fn test_comment_line() {
        test_case_lexable!("//");
        test_case_lexable!("//abc123");
        test_case_lexable!("// abc123");
        test_case_lexable!("// abc123 ");
        test_case_lexable!("// abc123 \n");
        test_case_lexable!("// abc123 \n// abc123 \n");
        test_case_lexable!(" // abc123 \n // abc123 \n");
        test_case_lexable!("\n // abc123 \n // abc123 \n");
    }

    #[test]
    pub fn test_comment_block() {
        test_case_lexable!("/**/");
        test_case_lexable!("/*abc123*/");
        test_case_lexable!("/* abc123 */");
        test_case_lexable!("\n/* abc123 */\n");
        test_case_lexable!("/*\nabc123\n*/");
        test_case_lexable!("/*\n*/");
    }

    #[test]
    pub fn test_string_quoted() {
        test_case_lexable!("\"\"");
        test_case_lexable!("\"abc123\"");
        test_case_lexable!("\"!@#$%^&*()_+1234567890-={}[];':<>,./?`~\"");

        // Note, character escape is implemented in the parser
        test_case_lexable!("\"\t\n\r\"");
        test_case_lexable!("\"\\t\\n\\r\"");
        test_case_lexable!("\"\\\\\"");

        /*
            - No single quotes
            - No unescaped "\"
        */
        test_case_unlexable!("''");
        test_case_unlexable!("'a'");
        test_case_unlexable!("'abc123'");
        test_case_unlexable!("\"\\\"");
    }

    #[test]
    pub fn test_string_unquoted() {
        test_case_lexable!("ABC123");
        test_case_lexable!("example.test");
        test_case_lexable!("/foo/bar/baz");
        test_case_lexable!("FOO_BAR_BAZ");
    }

    #[test]
    pub fn test_array() {
        test_case_lexable!("()");
        test_case_lexable!("( )");
        test_case_lexable!(" ( ) ");
        test_case_lexable!("(1)");
        test_case_lexable!("( 1 )");
        test_case_lexable!("(1,2,3)");
        test_case_lexable!("(1, 2, 3)");
        test_case_lexable!("( 1, 2, 3 )");
        test_case_lexable!("( \"1\", \"2\", \"3\" )");
        test_case_lexable!("( // \"1\", \"2\", \"3\" )\n1, 2, 3)");
        test_case_lexable!("( 1 /* a */, 2 /* a */, /* c */ 3 /* d */ )");
    }

    #[test]
    pub fn test_dictionary() {
        test_case_lexable!("{}");
        test_case_lexable!("{x=y;}");
        test_case_lexable!("{x=y;a=b;}");
        test_case_lexable!("{ x = y ; }");
        test_case_lexable!("{ \"x\" = \"y\" ; }");
        test_case_lexable!("{ \"x\" /* a */ = \"y\" /* b */ ; }");

        test_case_unlexable!("{;}");
        test_case_unlexable!("{x=}");
        test_case_unlexable!("{x=y}");
    }

    #[test]
    pub fn test_composite_objects() {
        test_case_lexable!("(())");
        test_case_lexable!("(1, 2, (3, 4))");
        test_case_lexable!("(1, 2, (3, {x = 1; y = 2;}))");
        test_case_lexable!("{x = (1, 2); y = 3;}");
        test_case_lexable!("{x = (); y = {z = 1;};}");
    }

}

// eof