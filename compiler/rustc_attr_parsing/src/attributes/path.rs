use super::prelude::*;

pub(crate) struct PathParser;

impl<S: Stage> SingleAttributeParser<S> for PathParser {
    const PATH: &[Symbol] = &[sym::path];
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::WarnButFutureError;
    const ALLOWED_TARGETS: AllowedTargets =
        AllowedTargets::AllowListWarnRest(&[Allow(Target::Mod), Error(Target::Crate)]);
    const TEMPLATE: AttributeTemplate = template!(
        NameValueStr: "file",
        "https://doc.rust-lang.org/reference/items/modules.html#the-path-attribute"
    );

    fn convert(cx: &mut AcceptContext<'_, '_, S>, args: &ArgParser) -> Option<AttributeKind> {
        Some(AttributeKind::Path(cx.expect_single_str(args, sym::path)?.0, cx.attr_span))
    }
}
