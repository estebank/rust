use rustc_span::hygiene::Transparency;

use super::prelude::*;

pub(crate) struct RustcMacroTransparencyParser;

impl<S: Stage> SingleAttributeParser<S> for RustcMacroTransparencyParser {
    const PATH: &[Symbol] = &[sym::rustc_macro_transparency];
    const ON_DUPLICATE: OnDuplicate<S> = OnDuplicate::Custom(|cx, used, unused| {
        cx.dcx().span_err(vec![used, unused], "multiple macro transparency attributes");
    });
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::MacroDef)]);
    const TEMPLATE: AttributeTemplate =
        template!(NameValueStr: ["transparent", "semiopaque", "opaque"]);

    fn convert(cx: &mut AcceptContext<'_, '_, S>, args: &ArgParser) -> Option<AttributeKind> {
        let name = cx.expect_single_str_allowlist(
            args,
            sym::rustc_macro_transparency,
            &[sym::transparent, sym::semiopaque, sym::opaque],
        );
        Some(AttributeKind::RustcMacroTransparency(match name {
            Some(sym::transparent) => Transparency::Transparent,
            Some(sym::semiopaque) => Transparency::SemiOpaque,
            Some(sym::opaque) => Transparency::Opaque,
            _ => return None,
        }))
    }
}
