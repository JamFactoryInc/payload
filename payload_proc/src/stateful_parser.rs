use crate::parse::ParseResult;

pub(crate) trait StatefulParser<AccRepr> {
    type Output;
    fn parse_byte(&mut self, byte: &u8) -> ParseResult<AccRepr>;
    fn get_invalid_end_sate_error(&self) -> String {
        "input ended in an invalid final state".to_string()
    }
}

macro_rules! stateful_parser {
    (
        for struct $parser_name:ident
        states
            $($variants:ident $(($variant_arg_type:ident))?),+
        transitions
            $(
                [$($state_from:tt)*] match $($match_from:literal $(..$match_to:literal)?)|+ $(and $guard:tt)?
                    => $(
                [$state_to:ident $(($($state_to_args:tt)*))?]
                         $( impl {$($logic:tt)*})?
                            -> $return_variant:ident $(($($return_variant_args:tt)*))?
                )? $($err_msg:literal)?;
            )+
        elements
            $($element:ident),*
        $(
             parsed sub elements
                $($reprs:ident as $repr:ident),*
        )?


    ) => {
        paste::paste!{
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
                pub(crate) fn try_parse(empty_variant: [<$parser_name AccRepr>], _value: String) -> Result<Self, String> {
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
                            ($($state_from)*, $([<b$match_from>]$(..=[<b$match_to>])?)|+)
                                $(if $guard)? => {
                                $(
                                    $($($logic)*)?

                                    self.state = $state_to $(($($state_to_args)*))?;
                                    crate::parse::ParseResult::$return_variant $(($($return_variant_args)*))?
                                )?
                                $(
                                    crate::parse::ParseResult::ParseError($err_msg.to_string())
                                )?
                            }
                        )+
                        _ => crate::parse::ParseResult::ParseError("Invalid syntax".to_string())
                    }
                }
            }

            impl TryFrom<String> for [<$parser_name ElementVec>] {
                type Error = String;

                fn try_from(value: String) -> Result<Self, String> {
                    let mut parser = [<$parser_name Parser>]::default();
                    let bytes = value.as_bytes();
                    if bytes.is_empty() {
                        return Err("attempted to parse empty string".to_string())
                    }
                    let mut valid_state = false;

                    for byte in bytes {
                        match <[<$parser_name Parser>] as crate::stateful_parser::StatefulParser<[<$parser_name AccRepr>]>>::parse_byte(&mut parser, byte) {
                            crate::parse::ParseResult::ParseError(msg) => return Err(msg),
                            crate::parse::ParseResult::Accumulate(c) => parser.accumulator.push(c.clone()),
                            crate::parse::ParseResult::ParseAccumulated(repr) => {
                                parser.elements.push(
                                    [<$parser_name Element>]::try_parse(
                                        repr,
                                        parser.accumulator.move_str()
                                    )?
                                )
                                },
                            crate::parse::ParseResult::Continue => continue,
                            crate::parse::ParseResult::GreedyContinue => {
                                valid_state = true;
                                continue
                            },
                            crate::parse::ParseResult::Parsed => return Ok([<$parser_name ElementVec>](parser.elements)),
                            crate::parse::ParseResult::Defer => panic!("Defer no longer valid")
                        }
                    }

                    if valid_state {
                        Ok([<$parser_name ElementVec>](parser.elements))
                    } else {
                        Err(<[<$parser_name Parser>] as crate::stateful_parser::StatefulParser<[<$parser_name AccRepr>]>>::get_invalid_end_sate_error(&parser))
                    }
                }
            }
        }
    }
}