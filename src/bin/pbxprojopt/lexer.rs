//! .pbxproj lexer derivation

use ansi_term::Style;
use pest::iterators::Pair;
use pest_derive::Parser as PestParser;

#[derive(PestParser)]
#[grammar = "pbxproj.pest"]
pub struct PBXProjLexer;

/// Recursively stylize tokens and matching rules from the lexer. Returns
/// whether the inner most token was reached. If 'buffer' is None, text will be
/// printed to stdout. If 'stylize' is true, ANSI color codes will be used.
///
pub fn visualize_matched_grammar_rule(
    pair: &Pair<Rule>,
    offset: usize,
    buffer: &mut Option<&mut String>,
    stylize: bool
) -> bool {

    let span = pair.clone().as_span();
    let inner_pairs = pair.clone().into_inner();

    // Format the rule name that generated the token and append the cursor
    // position if it's the inner-most token.
    let rule_name: String = format!("{:?}", pair.as_rule());
    let arrow: &str = if offset != 0 { "->" } else { "" };
    let position: String = if inner_pairs.len() == 0 {
        format!("<{},{}>", span.start(), span.end())
    } else { String::new() };

    let rule_name_style: Style;
    let location_style: Style;
    if stylize {
        rule_name_style = Style::new().bold();
        location_style = Style::new().dimmed();
    } else {
        rule_name_style = Style::new();
        location_style = Style::new();
    }

    let formatted = format!(
        "{}{}{}", arrow, rule_name_style.paint(&rule_name), location_style.paint(&position)
    );

    match buffer {
        Some(buf) => buf.push_str(formatted.as_str()),
        None => print!("{}", formatted),
    }

    // If the token is a leaf, also append the matching text
    // e.g. "File->LineComment->LineCommentValue: foo"
    if inner_pairs.len() == 0 {
        let leaf_text = format!(": {}", span.as_str().escape_debug());
        match buffer {
            Some(buf) => buf.push_str(leaf_text.as_str()),
            None => print!("{}", leaf_text),
        }
        return true;
    }

    // Format any inner rules/tokens
    let printed: usize = arrow.len() + rule_name.len() + position.len();
    let mut first: bool = true;
    for inner_pair in inner_pairs {

        // Vertically align inner tokens, e.g.
        // "File->LineComment->LineCommentValue: foo"
        // "    ->LineComment->LineCommentValue: bar"
        if first {
            first = false;
        } else {
            let indent = format!("{}", " ".repeat(printed + offset));
            match buffer {
                Some(buf) => buf.push_str(indent.as_str()),
                None => print!("{}", indent),
            }
        }

        let need_newline: bool = visualize_matched_grammar_rule(&inner_pair, printed + offset, buffer, stylize);
        if need_newline {
            match buffer {
                Some(buf) => buf.push_str("\n"),
                None => println!(""),
            }
        }
    }

    return false;
}

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
        test_case_lexable!("(1, 2, 3,)");
    }

    #[test]
    pub fn test_dictionary() {
        test_case_lexable!("{}");
        test_case_lexable!("{x=y;}");
        test_case_lexable!("{x=y;a=b;}");
        test_case_lexable!("{ x = y ; }");
        test_case_lexable!(r#"{ "x" = "y" ; }"#);
        test_case_lexable!(r#"{ "x" /* a */ = "y" /* b */ ; }"#);
        test_case_lexable!("{\nx = 1;\n}");
        test_case_lexable!("{ x = 1; /* test */ }");

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