use crate::parse::ParseResult;

pub(crate) trait StatefulParser<AccRepr> {
    type Output;
    fn parse_byte(&mut self, byte: &u8) -> ParseResult<AccRepr>;
}

macro_rules! stateful_parser {
    (
        for struct $parser_name:ident
        use transitions
            $(
                if ($($match_pattern:tt)*) $(and $guard:tt)?
                    move from $state_from:ident $(($($state_from_args:tt)*))?
                    to $state_to:ident $(($($state_to_args:tt)*))?
                         $(and use impl {$($logic:tt)*})?
                            then return $return_variant:ident $(($($return_variant_args:tt)*))?.
            )+
        for states
            $($variants:ident $(($variant_arg_type:ident))?),+
         with elements
            $($element:ident),*
        $(
             and sub elements
                $($reprs:ident as $repr:ident),*
        )?


    ) => {
        paste!{
            use self::[<$parser_name State>]::*;

            #[derive(Default)]
            enum [<$parser_name State>] {
                #[default]
                $($variants $(($variant_arg_type))?),+
            }

            enum [<$parser_name Element>] {
                    $($reprs ($repr)),*
                }

            #[derive(Default)]
            enum [<$parser_name AccRepr>] {
                #[default]
                Nil,
                $($reprs),*
            }

            impl [<$parser_name Element>] {
                pub(crate) fn try_parse(empty_variant: [<$parser_name AccRepr>], value: String) -> Result<Self, String> {
                    match empty_variant {
                        $(
                            [<$parser_name AccRepr>]::$reprs => Ok([<$parser_name Element>]::$reprs($repr::try_from(value)?))
                        ),*
                        _ => Err("Unsupported parse input variant".to_string())
                    }
                }
            }


            struct [<$parser_name ElementVec>](Vec<[<$parser_name Element>]>);

            #[derive(Default)]
            struct [<$parser_name Parser>] {
                pub(crate) state: [<$parser_name State>],
                pub(crate) accumulator: crate::accumulator::Accumulator,
                pub(crate) elements: Vec<[<$parser_name Element>]>
            }

            impl crate::stateful_parser::StatefulParser<[<$parser_name AccRepr>]> for [<$parser_name Parser>] {
                type Output = $parser_name;

                fn parse_byte(&mut self, char: &u8) -> crate::parse::ParseResult<[<$parser_name AccRepr>]> {
                    match (&self.state, char) {
                        $(
                            ($state_from $(($($state_from_args)*))?, $($match_pattern)*)
                                $(if $guard)? => {

                                $($($logic)*)?

                                self.state = $state_to $(($($state_to_args)*))?;
                                crate::parse::ParseResult::$return_variant $(($($return_variant_args)*))?
                            }
                        )+
                    }
                }
            }

            impl TryFrom<String> for [<$parser_name ElementVec>] {
                type Error = String;

                fn try_from(value: String) -> Result<Self, String> {
                    let mut parser = [<$parser_name Parser>]::default();

                    for byte in value.as_bytes() {
                        match parser.parse_byte(byte) {
                            crate::parse::ParseResult::ParseError(msg) => return Err(msg),
                            crate::parse::ParseResult::Accumulate(c) => parser.accumulator.push(c.clone()),
                            crate::parse::ParseResult::ParseAccumulated(repr) => {
                                parser.elements.push(
                                    [<$parser_name Element>]::try_parse(
                                        repr,
                                        crate::accumulator::Accumulator::vec_to_str(parser.accumulator.move_vec())
                                    )?
                                )
                                },
                            crate::parse::ParseResult::Continue => continue,
                            crate::parse::ParseResult::Parsed => return Ok([<$parser_name ElementVec>](parser.elements)),
                        }
                    }
                }
            }
        }
    }
}