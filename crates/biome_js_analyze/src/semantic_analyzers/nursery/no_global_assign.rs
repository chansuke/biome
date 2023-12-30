use crate::globals::browser::BROWSER;
use crate::globals::node::NODE;
use crate::globals::runtime::{BUILTIN, ES_2021};

use crate::semantic_services::SemanticServices;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{JsSyntaxKind, TextRange};

declare_rule! {
    /// Disallow assignments to native objects and read-only global variables.
    ///
    /// _JavaScript environments contain numerous built-in global variables, such as `window` in browsers and `process` in _Node.js.
    /// Assigning values to these global variables can be problematic as it can overwride essential functionality.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-global-assign
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// Object = null;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// window = 1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// undefined = 1;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// let a = 1;
    /// let b = 1;
    /// b = 100;
    /// a = 100;
    /// ```
    ///
    pub(crate) NoGlobalAssign {
        version: "next",
        name: "noGlobalAssign",
        recommended: false,
    }
}

impl Rule for NoGlobalAssign {
    type Query = SemanticServices;
    type State = TextRange;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let global_refs = ctx.query().all_unresolved_references();
        let mut result = vec![];

        for global_ref in global_refs {
            let is_write = global_ref.syntax().kind() == JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT;
            let identifier = global_ref.syntax().text_trimmed();
            let text = identifier.to_string();

            if is_write {
                let is_global_var = NODE.binary_search(&text.as_str()).is_ok()
                    || BROWSER.binary_search(&text.as_str()).is_ok()
                    || BUILTIN.binary_search(&text.as_str()).is_ok()
                    || ES_2021.binary_search(&text.as_str()).is_ok();
                if is_global_var {
                    result.push(*global_ref.range());
                }
            }
        }

        result
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "A global variable should not be reassigned"
                },
            )
            .note(markup! {
                "Assigning to a global variable can override essential functionality."
            }),
        )
    }
}
