use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple,
    sequence::delimited,
    character::complete::char,
    bytes::complete::is_not,
    error::ParseError,
    character::complete::multispace0,
    combinator::recognize,
    sequence::pair,
    branch::alt,
    character::complete::{alpha1},
    character::complete::alphanumeric1,
    combinator::{cut, map, opt},
    error::{context, VerboseError},
    multi::{many0, many1},
    sequence::{preceded, terminated},
    character::complete::{digit1, multispace1, one_of},
    multi::separated_list1,
    Parser,
};

pub type Error<T> = nom::Err<nom::error::Error<T>>;

pub(crate) mod string {
    //! This example shows an example of how to parse an escaped string. The
    //! rules for the string are similar to JSON and rust. A string is:
    //!
    //! - Enclosed by double quotes
    //! - Can contain any raw unescaped code point besides \ and "
    //! - Matches the following escape sequences: \b, \f, \n, \r, \t, \", \\, \/
    //! - Matches code points like Rust: \u{XXXX}, where XXXX can be up to 6
    //!   hex characters
    //! - an escape followed by whitespace consumes all whitespace between the
    //!   escape and the next non-whitespace character
    extern crate nom;

    use super::Error;

    use nom::branch::alt;
    use nom::bytes::streaming::{is_not, take_while_m_n};
    use nom::character::streaming::{char, multispace1};
    use nom::combinator::{map, map_opt, map_res, value, verify};
    use nom::error::{FromExternalError, ParseError};
    use nom::multi::fold_many0;
    use nom::sequence::{delimited, preceded};
    use nom::IResult;

    // parser combinators are constructed from the bottom up:
    // first we write parsers for the smallest elements (escaped characters),
    // then combine them into larger parsers.

    /// Parse a unicode sequence, of the form u{XXXX}, where XXXX is 1 to 6
    /// hexadecimal numerals. We will combine this later with parse_escaped_char
    /// to parse sequences like \u{00AC}.
    fn parse_unicode<'a, E>(input: &'a str) -> IResult<&'a str, char, E>
    where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        // `take_while_m_n` parses between `m` and `n` bytes (inclusive) that match
        // a predicate. `parse_hex` here parses between 1 and 6 hexadecimal numerals.
        let parse_hex = take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit());

        // `preceeded` takes a prefix parser, and if it succeeds, returns the result
        // of the body parser. In this case, it parses u{XXXX}.
        let parse_delimited_hex = preceded(
            char('u'),
            // `delimited` is like `preceded`, but it parses both a prefix and a suffix.
            // It returns the result of the middle parser. In this case, it parses
            // {XXXX}, where XXXX is 1 to 6 hex numerals, and returns XXXX
            delimited(char('{'), parse_hex, char('}')),
        );

        // `map_res` takes the result of a parser and applies a function that returns
        // a Result. In this case we take the hex bytes from parse_hex and attempt to
        // convert them to a u32.
        let parse_u32 = map_res(parse_delimited_hex, move |hex| u32::from_str_radix(hex, 16));

        // map_opt is like map_res, but it takes an Option instead of a Result. If
        // the function returns None, map_opt returns an error. In this case, because
        // not all u32 values are valid unicode code points, we have to fallibly
        // convert to char with from_u32.
        map_opt(parse_u32, |value| std::char::from_u32(value))(input)
    }

    /// Parse an escaped character: \n, \t, \r, \u{00AC}, etc.
    fn parse_escaped_char<'a, E>(input: &'a str) -> IResult<&'a str, char, E>
    where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        preceded(
            char('\\'),
            // `alt` tries each parser in sequence, returning the result of
            // the first successful match
            alt((
            parse_unicode,
            // The `value` parser returns a fixed value (the first argument) if its
            // parser (the second argument) succeeds. In these cases, it looks for
            // the marker characters (n, r, t, etc) and returns the matching
            // character (\n, \r, \t, etc).
            value('\n', char('n')),
            value('\r', char('r')),
            value('\t', char('t')),
            value('\u{08}', char('b')),
            value('\u{0C}', char('f')),
            value('\\', char('\\')),
            value('/', char('/')),
            value('"', char('"')),
            )),
        )(input)
    }

    /// Parse a backslash, followed by any amount of whitespace. This is used later
    /// to discard any escaped whitespace.
    fn parse_escaped_whitespace<'a, E: ParseError<&'a str>>(
    input: &'a str,
    ) -> IResult<&'a str, &'a str, E> {
        preceded(char('\\'), multispace1)(input)
    }
    /// Parse a non-empty block of text that doesn't include \ or "
    fn parse_literal<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    // `is_not` parses a string of 0 or more characters that aren't one of the
    // given characters.
    let not_quote_slash = is_not("\"\\");

    // `verify` runs a parser, then runs a verification function on the output of
    // the parser. The verification function accepts out output only if it
    // returns true. In this case, we want to ensure that the output of is_not
    // is non-empty.
    verify(not_quote_slash, |s: &str| !s.is_empty())(input)
    }

    /// A string fragment contains a fragment of a string being parsed: either
    /// a non-empty Literal (a series of non-escaped characters), a single
    /// parsed escaped character, or a block of escaped whitespace.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum StringFragment<'a> {
        Literal(&'a str),
        EscapedChar(char),
        EscapedWS,
    }

    /// Combine parse_literal, parse_escaped_whitespace, and parse_escaped_char
    /// into a StringFragment.
    fn parse_fragment<'a, E>(input: &'a str) -> IResult<&'a str, StringFragment<'a>, E>
    where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        alt((
            // The `map` combinator runs a parser, then applies a function to the output
            // of that parser.
            map(parse_literal, StringFragment::Literal),
            map(parse_escaped_char, StringFragment::EscapedChar),
            value(StringFragment::EscapedWS, parse_escaped_whitespace),
        ))(input)
    }

    pub fn some_string<'a, E>(source: &'a str) -> IResult<&'a str, String, E>
    where E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        // fold_many0 is the equivalent of iterator::fold. It runs a parser in a loop,
        // and for each output value, calls a folding function on each output value.
        fold_many0(
            // Our parser function??? parses a single string fragment
            parse_fragment,
            // Our init value, an empty string
            String::new(),
            // Our folding function. For each fragment, append the fragment to the
            // string.
            |mut string, fragment| {
            match fragment {
                StringFragment::Literal(s) => string.push_str(s),
                StringFragment::EscapedChar(c) => string.push(c),
                StringFragment::EscapedWS => {}
            }
            string
            },
        )(source)
    }


    /// Parse a string. Use a loop of parse_fragment and push all of the fragments
    /// into an output string.
    pub(crate) fn parse_string_impl<'a, E>(input: &'a str) -> IResult<&'a str, String, E>
    where E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        // Finally, parse the string. Note that, if `build_string` could accept a raw
        // " character, the closing delimiter " would never match. When using
        // `delimited` with a looping parser (like fold_many0), be sure that the
        // loop won't accidentally match your closing delimiter!
        delimited(char('"'), some_string, char('"'))(input)
    }

    pub(crate) fn parse_string(
        source: &str
    ) -> Result<(&str, String), Error<&str>>
    {
        match parse_string_impl(source) {
            Err(err) => Err(err),
            Ok(val) => Ok(val),
        }
    }
}

pub(crate) fn parens<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F
) -> impl FnMut(&'a str) -> Result<(&'a str, O), nom::Err<E>>
    where F: Fn(&'a str) -> Result<(&'a str, O), nom::Err<E>>
{
    move |source: &str| {
        let (source, _) = char('(')(source)?;
        let (source, value) = inner(source)?;
        let (source, _) = char(')')(source)?;
        Ok((source, value))
    }
}

pub(crate) fn choice<'a, F: 'a, O, E: ParseError<&'a str>>(
    parsers: &'static [F]
) -> impl FnMut(&'a str) -> Result<(&'a str, O), Error<&str>>
    where
        F: Fn(&'a str) -> Result<(&'a str, O), nom::Err<E>>,
        O: std::fmt::Debug,
        O: std::fmt::Display
{
    move |source: &'a str| -> Result<(&'a str, O), Error<&str>> {
        // let mut result: Option<O> = None;
        // let mut errors = Vec::new();
        // let mut matches: Vec<(&str, O)> = Vec::new();
        for f in parsers.to_owned() {
            match f(source) {
                Ok(x) => {
                    // println!("{:?} -> {:?} , {}", source, x.0, x.1);
                    return Ok(x)
                    // matches.push(x);
                    // return Ok(x);
                }
                Err(_) => ()
            }
        }
        // let mut last: Option<(&str, O)> = None;
        // println!("LENGTH: {}", matches.len());
        // for res in matches.iter() {
        //     println!("REST: {}", res.0);
        // }
        // for res in matches {
        //     // println!("{}\t-->\t{:?} <-> {:?}", source, res.0, res.1);
        //     let last_len = match last {
        //         Some((x, _)) => x.len(),
        //         _ => 0
        //     };
        //     if res.0.len() <= last_len {
        //         last = Some(res);
        //     }
        //     // return Ok(res);
        // }
        // println!("*******************");
        // match last {
        //     Some(x) => return Ok(x),
        //     _ => ()
        // }
        let e: Error<&str> = nom::Err::Error(nom::error::Error::new(
            source,
            nom::error::ErrorKind::Alt
        ));
        // println!("PASSED: {}", source);
        Err(e)
    }
}

pub(crate) fn comma(source: &str) -> Result<(&str, &str), Error<&str>> {
    let (source, val) = ws(tag(","))(source)?;
    Ok((source, val))
}

pub(crate) fn identifier(source: &str) -> Result<(&str, String), Error<&str>> {
    let (source, ident) = recognize(
        pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"), tag("-"))))
        )
    )(source)?;
    Ok((source, ident.to_owned()))
}

/// Whitespace.
pub(crate) fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where F: Fn(&'a str) -> IResult<&'a str, O, E>
{
  delimited(
    multispace0,
    inner,
    multispace0,
  )
}

