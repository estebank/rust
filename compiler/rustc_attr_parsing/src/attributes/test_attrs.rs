use rustc_hir::attrs::RustcAbiAttrKind;
use rustc_session::lint::builtin::ILL_FORMED_ATTRIBUTE_INPUT;

use super::prelude::*;

pub(crate) struct IgnoreParser;

impl<S: Stage> SingleAttributeParser<S> for IgnoreParser {
    const PATH: &[Symbol] = &[sym::ignore];
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Warn;
    const ALLOWED_TARGETS: AllowedTargets =
        AllowedTargets::AllowListWarnRest(&[Allow(Target::Fn), Error(Target::WherePredicate)]);
    const TEMPLATE: AttributeTemplate = template!(
        Word, NameValueStr: "reason",
        "https://doc.rust-lang.org/reference/attributes/testing.html#the-ignore-attribute"
    );

    fn convert(cx: &mut AcceptContext<'_, '_, S>, args: &ArgParser) -> Option<AttributeKind> {
        Some(AttributeKind::Ignore {
            span: cx.attr_span,
            reason: match args {
                ArgParser::NoArgs => None,
                ArgParser::NameValue(name_value) => {
                    let Some(str_value) = name_value.value_as_str() else {
                        cx.warn_ill_formed_attribute_input(ILL_FORMED_ATTRIBUTE_INPUT);
                        return None;
                    };
                    Some(str_value)
                }
                ArgParser::List(_) => {
                    cx.warn_ill_formed_attribute_input(ILL_FORMED_ATTRIBUTE_INPUT);
                    return None;
                }
            },
        })
    }
}

pub(crate) struct ShouldPanicParser;

impl<S: Stage> SingleAttributeParser<S> for ShouldPanicParser {
    const PATH: &[Symbol] = &[sym::should_panic];
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::WarnButFutureError;
    const ALLOWED_TARGETS: AllowedTargets =
        AllowedTargets::AllowListWarnRest(&[Allow(Target::Fn), Error(Target::WherePredicate)]);
    const TEMPLATE: AttributeTemplate = template!(
        Word, List: &[r#"expected = "reason""#], NameValueStr: "reason",
        "https://doc.rust-lang.org/reference/attributes/testing.html#the-should_panic-attribute"
    );

    fn convert(cx: &mut AcceptContext<'_, '_, S>, args: &ArgParser) -> Option<AttributeKind> {
        Some(AttributeKind::ShouldPanic {
            span: cx.attr_span,
            reason: match args {
                ArgParser::NoArgs => None,
                ArgParser::NameValue(_) => Some(cx.expect_single_str(args, sym::should_panic)?.0),
                ArgParser::List(list) => {
                    let Some(single) = list.single() else {
                        cx.expected_single_argument(list.span);
                        return None;
                    };
                    let Some(single) = single.meta_item() else {
                        cx.expected_name_value(single.span(), Some(sym::expected));
                        return None;
                    };
                    if !single.path().word_is(sym::expected) {
                        cx.expected_specific_argument_strings(list.span, &[sym::expected]);
                        return None;
                    }
                    let Some(nv) = single.args().name_value() else {
                        cx.expected_name_value(single.span(), Some(sym::expected));
                        return None;
                    };
                    let Some(expected) = nv.value_as_str() else {
                        cx.expected_string_literal(nv.value_span, Some(nv.value_as_lit()));
                        return None;
                    };
                    Some(expected)
                }
            },
        })
    }
}

pub(crate) struct ReexportTestHarnessMainParser;

impl<S: Stage> SingleAttributeParser<S> for ReexportTestHarnessMainParser {
    const PATH: &[Symbol] = &[sym::reexport_test_harness_main];
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Error;
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Crate)]);
    const TEMPLATE: AttributeTemplate = template!(NameValueStr: "name");

    fn convert(cx: &mut AcceptContext<'_, '_, S>, args: &ArgParser) -> Option<AttributeKind> {
        let (name, _) = cx.expect_single_str(args, sym::reexport_test_harness_main)?;
        Some(AttributeKind::ReexportTestHarnessMain(name))
    }
}

pub(crate) struct RustcAbiParser;

impl<S: Stage> SingleAttributeParser<S> for RustcAbiParser {
    const PATH: &[Symbol] = &[sym::rustc_abi];
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Warn;
    const TEMPLATE: AttributeTemplate = template!(OneOf: &[sym::debug, sym::assert_eq]);
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[
        Allow(Target::TyAlias),
        Allow(Target::Fn),
        Allow(Target::ForeignFn),
        Allow(Target::Method(MethodKind::Inherent)),
        Allow(Target::Method(MethodKind::Trait { body: true })),
        Allow(Target::Method(MethodKind::Trait { body: false })),
        Allow(Target::Method(MethodKind::TraitImpl)),
    ]);

    fn convert(cx: &mut AcceptContext<'_, '_, S>, args: &ArgParser) -> Option<AttributeKind> {
        let symbol = cx.expect_single_ident(args, &[sym::assert_eq, sym::debug]);
        let kind: RustcAbiAttrKind = match symbol {
            Ok(sym::assert_eq) => RustcAbiAttrKind::AssertEq,
            Ok(sym::debug) => RustcAbiAttrKind::Debug,
            _ => return None,
        };
        Some(AttributeKind::RustcAbi { attr_span: cx.attr_span, kind })
    }
}

pub(crate) struct RustcDelayedBugFromInsideQueryParser;

impl<S: Stage> NoArgsAttributeParser<S> for RustcDelayedBugFromInsideQueryParser {
    const PATH: &[Symbol] = &[sym::rustc_delayed_bug_from_inside_query];
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Warn;
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Fn)]);
    const CREATE: fn(Span) -> AttributeKind = |_| AttributeKind::RustcDelayedBugFromInsideQuery;
}

pub(crate) struct RustcEvaluateWhereClausesParser;

impl<S: Stage> NoArgsAttributeParser<S> for RustcEvaluateWhereClausesParser {
    const PATH: &[Symbol] = &[sym::rustc_evaluate_where_clauses];
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Warn;
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[
        Allow(Target::Fn),
        Allow(Target::Method(MethodKind::Inherent)),
        Allow(Target::Method(MethodKind::Trait { body: true })),
        Allow(Target::Method(MethodKind::TraitImpl)),
        Allow(Target::Method(MethodKind::Trait { body: false })),
    ]);
    const CREATE: fn(Span) -> AttributeKind = |_| AttributeKind::RustcEvaluateWhereClauses;
}

pub(crate) struct TestRunnerParser;

impl<S: Stage> SingleAttributeParser<S> for TestRunnerParser {
    const PATH: &[Symbol] = &[sym::test_runner];
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Error;
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::Crate)]);
    const TEMPLATE: AttributeTemplate = template!(List: &["path"]);

    fn convert(cx: &mut AcceptContext<'_, '_, S>, args: &ArgParser) -> Option<AttributeKind> {
        let Some(list) = args.list() else {
            cx.expected_list(cx.attr_span, args);
            return None;
        };

        let Some(single) = list.single() else {
            cx.expected_single_argument(list.span);
            return None;
        };

        let Some(meta) = single.meta_item() else {
            cx.unexpected_literal(single.span());
            return None;
        };

        Some(AttributeKind::TestRunner(meta.path().0.clone()))
    }
}

pub(crate) struct RustcTestMarkerParser;

impl<S: Stage> SingleAttributeParser<S> for RustcTestMarkerParser {
    const PATH: &[Symbol] = &[sym::rustc_test_marker];
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Warn;
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[
        Allow(Target::Const),
        Allow(Target::Fn),
        Allow(Target::Static),
    ]);
    const TEMPLATE: AttributeTemplate = template!(NameValueStr: "test_path");

    fn convert(cx: &mut AcceptContext<'_, '_, S>, args: &ArgParser) -> Option<AttributeKind> {
        let (value_str, _) = cx.expect_single_non_empty_str(args, sym::rustc_test_marker)?;
        Some(AttributeKind::RustcTestMarker(value_str))
    }
}
