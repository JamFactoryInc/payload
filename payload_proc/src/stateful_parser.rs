use crate::parse::ParseResult;

pub(crate) trait StatefulParser<AccRepr> {
    type Output;
    fn new() -> Self;
    fn parse_byte(&mut self, byte: &u8) -> ParseResult<AccRepr>;
}

#[macro_export]
macro_rules! stateful_parser {
    (
        for $parser_name:ident
        states {
            $(
                $variants:ident $(($($state_args:tt)*))? : $(
                    $match_condition:tt $(if $guard:tt)? => $result:ident $(($($result_arg:tt)*))? $({$($logic:tt)*})?
                )+
            ),+
        }
        accum_reprs {
            $($reprs:ident => $repr:ident),+
        }
    ) => {
        paste!{
            use crate::stateful_parser::StatefulParser;
            use self::[<$parser_name State>]::*;

            #[derive(Default)]
            enum [<$parser_name State>] {
                $($variants),+
            }

            #[derive(Default)]
            enum [<$parser_name AccRepr>] {
                $($reprs),+
            }

            #[derive(Default)]
            enum [<$parser_name Element>] {
                $($reprs ($repr)),+
            }
            impl [<$parser_name Element>] {
                pub(crate) fn try_parse(empty_variant: [<$parser_name AccRepr>], value: String) -> Result<Self, String> {
                    match empty_variant {
                        $(
                            [<$parser_name AccRepr>]::$reprs => Ok([<$parser_name Element>]::$reprs($repr.try_from(value)?))
                        ),+
                    }
                }
            }

            #[derive(Default)]
            struct [<$parser_name Parser>] {
                state: [<$parser_name State>],
                accumulator: Accumulator
            }

            impl StatefulParser<[<$parser_name AccRepr>]> for [<$parser_name Parser>] {
                type Output = $parser_name;

                fn parse_byte(&mut self, char: &u8) -> ParseResult<[<$parser_name AccRepr>]> {
                    match (&self.state, char) {
                        $(
                            ($variants $(($($state_args)*))?, )
                        )+
                    }
                }
            }

            impl TryFrom<String> for $parser_name {
                type Error = String;

                fn try_from(value: String) -> Result<Self, Self::Error> {
                    let mut parser = [<$parser_name Parser>]::default();

                    for byte in value.as_bytes() {
                        match parser.parse_byte(byte) {
                            ParseError(msg) => return Err(msg),
                            Accumulate(c) => self.accumulator.push(c),
                            ParseAccumulated(T),
                            // the parser is happy :)
                            Continue,
                            // defer responsibility to a new child parser and set its root type
                            Defer,
                            // the parser's done and gives its value back to the caller
                            Parsed,
                        }
                    }
                }
            }
        }
    };
}