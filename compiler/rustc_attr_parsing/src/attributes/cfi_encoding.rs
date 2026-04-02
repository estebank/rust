use super::prelude::*;
pub(crate) struct CfiEncodingParser;
impl<S: Stage> SingleAttributeParser<S> for CfiEncodingParser {
    const PATH: &[Symbol] = &[sym::cfi_encoding];
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowListWarnRest(&[
        Allow(Target::Struct),
        Allow(Target::ForeignTy),
        Allow(Target::Enum),
        Allow(Target::Union),
    ]);
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Error;
    const TEMPLATE: AttributeTemplate = template!(NameValueStr: "encoding");

    fn convert(cx: &mut AcceptContext<'_, '_, S>, args: &ArgParser) -> Option<AttributeKind> {
        let (encoding, _) = cx.expect_single_non_empty_str(args, sym::cfi_encoding)?;
        Some(AttributeKind::CfiEncoding { encoding })
    }
}
