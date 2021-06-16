/// For the macro...
extern crate pest;
#[macro_use]
extern crate pest_derive;

/// import modules
// use std::fs;
use pest::Parser;
use pest::iterators::Pair;

/// marker struct and add path to grammar file
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct IdentParser;

/// Print detail of a current Pair and optional divider
fn print_pair(pair: &Pair<Rule>, hard_divider: bool) {
    println!("Rule: {:?}", pair.as_rule());
    println!("Span: {:?}", pair.as_span());
    println!("Text: {:?}", pair.as_str());
    if hard_divider {
        println!("{:=>60}", "");
    } else {
        println!("{:->60}", "");
    }
}

fn main() {
    // parse sample string input
    let pair = IdentParser::parse(Rule::declaration, "var fool, bar_99, fooBar;")
        .expect("unsuccessful parse")
        .next().unwrap();

    print_pair(&pair, true);

    // Iterate over the "inner" pairs
    for inner_pair in pair.into_inner() {
        print_pair(&inner_pair, true);

        match inner_pair.as_rule() {
            // if we match ident rule
            Rule::idents => {
                // iterate over another inner_pair
                for inner_inner_pair in inner_pair.into_inner() {
                    match inner_inner_pair.as_rule() {
                        // term ident is the last level
                        Rule::ident => {
                            print_pair(&inner_inner_pair, false)
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

