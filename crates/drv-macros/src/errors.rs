use std::fmt::Display;

use proc_macro2::Span;

use syn::Error as SynError;

#[derive(Debug)]
pub(crate) enum MacroError {
    IncorrectMacroUsage(String, Span),
    IncorrectEntityType(String, Span),
    IncorrectInputType(String, Span),
    UnnamedField(Span),
    UsedGeneric(String, Span),
}

impl MacroError {
    pub(crate) fn to_syn_error(self) -> SynError {
        self.into()
    }
}

impl Display for MacroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MacroError::IncorrectInputType(msg, _) => write!(f, "Incorrect input type: {}", msg),
            MacroError::IncorrectMacroUsage(msg, _) => write!(f, "Incorrect macro usage: {}", msg),
            MacroError::UnnamedField(_) => {
                write!(f, "Unnamed field detected, when only named are expected")
            }
            MacroError::UsedGeneric(msg, _) => {
                write!(f, "Generic {} can not be used in accounts", msg)
            }

            MacroError::IncorrectEntityType(msg, _) => {
                write!(f, "Incorrect given entity type: {}", msg)
            }
        }
    }
}

impl Into<SynError> for MacroError {
    fn into(self) -> SynError {
        match self {
            MacroError::IncorrectInputType(msg, span) => {
                SynError::new(span, format!("Incorrect input type: {}", msg))
            }
            MacroError::IncorrectMacroUsage(msg, span) => {
                SynError::new(span, format!("Incorrect macros usage: {}", msg))
            }
            MacroError::UsedGeneric(msg, span) => SynError::new(
                span,
                format!(
                    "Generic {} is used here, while DrvAccounts does not allow generics",
                    msg
                ),
            ),

            MacroError::UnnamedField(span) => {
                SynError::new(span, "Unnamed field detected, when only named are expected")
            }

            MacroError::IncorrectEntityType(msg, span) => {
                SynError::new(span, format!("Incorrect given entity type: {}", msg))
            }
        }
    }
}

impl std::error::Error for MacroError {}
