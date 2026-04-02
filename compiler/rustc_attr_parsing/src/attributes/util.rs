use std::num::IntErrorKind;

use rustc_ast::LitKind;
use rustc_ast::attr::AttributeExt;
use rustc_errors::ErrorGuaranteed;
use rustc_feature::is_builtin_attr_name;
use rustc_hir::RustcVersion;
use rustc_hir::limit::Limit;
use rustc_span::{Span, Symbol};

use crate::context::{AcceptContext, Stage};
use crate::parser::{ArgParser, NameValueParser};
use crate::session_diagnostics::LimitInvalid;

/// Parse a rustc version number written inside string literal in an attribute,
/// like appears in `since = "1.0.0"`. Suffixes like "-dev" and "-nightly" are
/// not accepted in this position, unlike when parsing CFG_RELEASE.
pub fn parse_version(s: Symbol) -> Option<RustcVersion> {
    let mut components = s.as_str().split('-');
    let d = components.next()?;
    if components.next().is_some() {
        return None;
    }
    let mut digits = d.splitn(3, '.');
    let major = digits.next()?.parse().ok()?;
    let minor = digits.next()?.parse().ok()?;
    let patch = digits.next().unwrap_or("0").parse().ok()?;
    Some(RustcVersion { major, minor, patch })
}

pub fn is_builtin_attr(attr: &impl AttributeExt) -> bool {
    attr.is_doc_comment().is_some() || attr.name().is_some_and(|name| is_builtin_attr_name(name))
}

impl<S: Stage> AcceptContext<'_, '_, S> {
    /// Parse a single integer.
    ///
    /// Used by attributes that take a single integer as argument, such as
    /// `#[link_ordinal]` and `#[rustc_layout_scalar_valid_range_start]`.
    /// `args` is the parser for the attribute arguments.
    pub(crate) fn parse_single_integer(&self, args: &ArgParser) -> Option<u128> {
        let Some(list) = args.list() else {
            self.expected_list(self.attr_span, args);
            return None;
        };
        let Some(single) = list.single() else {
            self.expected_single_argument(list.span);
            return None;
        };
        let Some(lit) = single.lit() else {
            self.expected_integer_literal(single.span());
            return None;
        };
        let LitKind::Int(num, _ty) = lit.kind else {
            self.expected_integer_literal(single.span());
            return None;
        };
        Some(num.0)
    }

    pub(crate) fn parse_limit_int(&self, nv: &NameValueParser) -> Option<Limit> {
        let Some(limit) = nv.value_as_str() else {
            self.expected_string_literal(nv.value_span, Some(nv.value_as_lit()));
            return None;
        };

        let error_str = match limit.as_str().parse() {
            Ok(i) => return Some(Limit::new(i)),
            Err(e) => match e.kind() {
                IntErrorKind::PosOverflow => "`limit` is too large",
                IntErrorKind::Empty => "`limit` must be a non-negative integer",
                IntErrorKind::InvalidDigit => "not a valid integer",
                IntErrorKind::NegOverflow => {
                    panic!(
                        "`limit` should never negatively overflow since we're parsing into a usize and we'd get Empty instead"
                    )
                }
                IntErrorKind::Zero => {
                    panic!("zero is a valid `limit` so should have returned Ok() when parsing")
                }
                kind => panic!("unimplemented IntErrorKind variant: {:?}", kind),
            },
        };

        self.emit_err(LimitInvalid { span: self.attr_span, value_span: nv.value_span, error_str });

        None
    }

    pub(crate) fn expect_limit_int(&self, args: &ArgParser, name: Symbol) -> Option<Limit> {
        let ArgParser::NameValue(nv) = args else {
            self.expected_name_value(self.inner_span, Some(name));
            return None;
        };
        self.parse_limit_int(nv)
    }

    pub(crate) fn expect_any_ident(&self, args: &ArgParser) -> Result<Symbol, ErrorGuaranteed> {
        self.expect_single_ident_inner(args, None, false)
    }

    pub(crate) fn expect_single_ident(
        &self,
        args: &ArgParser,
        valid: &[Symbol],
    ) -> Result<Symbol, ErrorGuaranteed> {
        self.expect_single_ident_inner(args, Some(valid), false)
    }

    pub(crate) fn expect_single_ident_or_no_args(
        &self,
        args: &ArgParser,
        valid: &[Symbol],
    ) -> Result<Symbol, ErrorGuaranteed> {
        self.expect_single_ident_inner(args, Some(valid), true)
    }

    fn expect_single_ident_inner(
        &self,
        args: &ArgParser,
        valid: Option<&[Symbol]>,
        no_args: bool,
    ) -> Result<Symbol, ErrorGuaranteed> {
        let error = || match valid {
            Some(valid) if no_args => {
                self.expected_specific_argument_and_list_or_no_argument(self.attr_span, valid)
            }
            Some(valid) => self.expected_specific_argument_and_list(self.attr_span, valid),
            None => self.expected_single_argument(args.span().unwrap_or(self.inner_span)),
        };
        let Some(list) = args.list() else {
            return Err(error());
        };
        let Some(single) = list.single() else {
            return Err(error());
        };
        let Some(item) = single.meta_item() else {
            return Err(error());
        };
        let Some(word) = item.path().word() else {
            return Err(error());
        };
        if let Some(valid) = valid
            && !valid.contains(&word.name)
        {
            return Err(self.expected_specific_argument(single.span(), valid));
        }
        Ok(word.name)
    }

    pub(crate) fn expect_single_str(
        &self,
        args: &ArgParser,
        expected_name: Symbol,
    ) -> Option<(Symbol, Span)> {
        self.expect_single_str_inner(args, expected_name, true)
    }

    pub(crate) fn expect_single_non_empty_str(
        &self,
        args: &ArgParser,
        expected_name: Symbol,
    ) -> Option<(Symbol, Span)> {
        self.expect_single_str_inner(args, expected_name, false)
    }

    fn expect_single_str_inner(
        &self,
        args: &ArgParser,
        expected_name: Symbol,
        allow_empty: bool,
    ) -> Option<(Symbol, Span)> {
        let Some(nv) = args.name_value() else {
            self.expected_name_value(self.inner_span, Some(expected_name));
            return None;
        };
        let Some(value) = nv.value_as_str() else {
            self.expected_string_literal(nv.value_span, Some(nv.value_as_lit()));
            return None;
        };
        if !allow_empty && value.as_str().trim().is_empty() {
            self.expected_non_empty_string_literal(nv.value_span);
            return None;
        }
        Some((value, nv.value_span))
    }

    pub(crate) fn expect_single_str_allowlist(
        &self,
        args: &ArgParser,
        expected_name: Symbol,
        expected: &[Symbol],
    ) -> Option<Symbol> {
        let (value, value_span) = self.expect_single_str(args, expected_name)?;
        if !expected.contains(&value) {
            self.expected_specific_argument_strings(value_span, expected);
            return None;
        }
        Some(value)
    }

    pub(crate) fn expect_no_args(&self, args: &ArgParser) -> Option<()> {
        if let Err(span) = args.no_args() {
            self.expected_no_args(span);
            return None;
        }
        Some(())
    }
}
