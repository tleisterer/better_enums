use proc_macro2::Span;
use syn::{Error as SynError, Expr, ExprRange, Ident, Lit, RangeLimits, UnOp};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Number {
    Signed(i128),
    Unsigned(u128),
}

impl Number {
    pub(crate) fn parse(expr: &Expr, unsigned: bool) -> syn::Result<Self> {
        match expr {
            Expr::Lit(lit) => match &lit.lit {
                Lit::Int(value) => {
                    if unsigned {
                        let number = value.base10_parse::<u128>().map_err(|_| {
                            SynError::new_spanned(
                                expr,
                                "better_enums: value is outside the repr range",
                            )
                        })?;
                        Ok(Self::Unsigned(number))
                    } else {
                        let number = value.base10_parse::<i128>().map_err(|_| {
                            SynError::new_spanned(
                                expr,
                                "better_enums: value is outside the repr range",
                            )
                        })?;
                        Ok(Self::Signed(number))
                    }
                }
                _ => Err(SynError::new_spanned(
                    expr,
                    "better_enums: expected an integer literal",
                )),
            },
            Expr::Unary(unary) if matches!(unary.op, UnOp::Neg(_)) => {
                let value = Self::parse(&unary.expr, true)?;
                match value {
                    Self::Unsigned(value) if value == (i128::MAX as u128) + 1 => {
                        Ok(Self::Signed(i128::MIN))
                    }
                    Self::Unsigned(value) => i128::try_from(value)
                        .map_err(|_| {
                            SynError::new_spanned(
                                expr,
                                "better_enums: value is outside the repr range",
                            )
                        })?
                        .checked_neg()
                        .map(Self::Signed)
                        .ok_or_else(|| {
                            SynError::new_spanned(
                                expr,
                                "better_enums: value is outside the repr range",
                            )
                        }),
                    Self::Signed(_) => unreachable!("Negated values must be parsed as signed"),
                }
            }
            _ => Err(SynError::new_spanned(
                expr,
                "better_enums: expected an integer literal",
            )),
        }
    }

    pub(crate) fn validate(&self, domain: Domain) -> bool {
        (domain.unsigned && matches!(self, Self::Unsigned(_)))
            || (!domain.unsigned && matches!(self, Self::Signed(_)))
                && *self >= domain.min
                && *self <= domain.max
    }

    pub(crate) fn into_expr(self) -> Expr {
        let text = match self {
            Number::Signed(value) => value.to_string(),
            Number::Unsigned(value) => value.to_string(),
        };
        syn::parse_str(&text).expect("validated discriminant should parse")
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Domain {
    pub(crate) min: Number,
    pub(crate) max: Number,
    pub(crate) unsigned: bool,
    pub(crate) span: Span,
}

impl TryFrom<&Ident> for Domain {
    type Error = syn::Error;
    fn try_from(repr: &Ident) -> Result<Self, Self::Error> {
        let result = match repr.to_string().as_str() {
            "i8" => Domain {
                min: Number::Signed(i8::MIN as i128),
                max: Number::Signed(i8::MAX as i128),
                unsigned: false,
                span: repr.span(),
            },
            "i16" => Domain {
                min: Number::Signed(i16::MIN as i128),
                max: Number::Signed(i16::MAX as i128),
                unsigned: false,
                span: repr.span(),
            },
            "i32" => Domain {
                min: Number::Signed(i32::MIN as i128),
                max: Number::Signed(i32::MAX as i128),
                unsigned: false,
                span: repr.span(),
            },
            "i64" => Domain {
                min: Number::Signed(i64::MIN as i128),
                max: Number::Signed(i64::MAX as i128),
                unsigned: false,
                span: repr.span(),
            },
            "i128" => Domain {
                min: Number::Signed(i128::MIN),
                max: Number::Signed(i128::MAX),
                unsigned: false,
                span: repr.span(),
            },
            "isize" => Domain {
                min: Number::Signed(isize::MIN as i128),
                max: Number::Signed(isize::MAX as i128),
                unsigned: false,
                span: repr.span(),
            },
            "u8" => Domain {
                min: Number::Unsigned(0),
                max: Number::Unsigned(u8::MAX as u128),
                unsigned: true,
                span: repr.span(),
            },
            "u16" => Domain {
                min: Number::Unsigned(0),
                max: Number::Unsigned(u16::MAX as u128),
                unsigned: true,
                span: repr.span(),
            },
            "u32" => Domain {
                min: Number::Unsigned(0),
                max: Number::Unsigned(u32::MAX as u128),
                unsigned: true,
                span: repr.span(),
            },
            "u64" => Domain {
                min: Number::Unsigned(0),
                max: Number::Unsigned(u64::MAX as u128),
                unsigned: true,
                span: repr.span(),
            },
            "u128" => Domain {
                min: Number::Unsigned(0),
                max: Number::Unsigned(u128::MAX),
                unsigned: true,
                span: repr.span(),
            },
            "usize" => Domain {
                min: Number::Unsigned(0),
                max: Number::Unsigned(usize::MAX as u128),
                unsigned: true,
                span: repr.span(),
            },
            _ => {
                return Err(SynError::new(
                    repr.span(),
                    "better_enums: repr must be an integer type",
                ));
            }
        };
        Ok(result)
    }
}

#[derive(Clone)]
pub(crate) struct RangeValue {
    pub(crate) start: Option<Expr>,
    pub(crate) end: Option<Expr>,
    pub(crate) inclusive: bool,
    pub(crate) lower: Number,
    pub(crate) upper: Number,
}

impl RangeValue {
    pub(crate) fn parse(range: &ExprRange, domain: Domain) -> syn::Result<Self> {
        let start = range.start.as_deref().cloned();
        let end = range.end.as_deref().cloned();
        let start_value = start
            .as_ref()
            .map(|expr| Number::parse(expr, domain.unsigned))
            .transpose()?
            .unwrap_or(domain.min);
        let end_value = end
            .as_ref()
            .map(|expr| Number::parse(expr, domain.unsigned))
            .transpose()?
            .unwrap_or(domain.max);

        if let Some(expr) = &start {
            if !start_value.validate(domain) {
                return Err(SynError::new_spanned(
                    expr,
                    "better_enums: value is outside the repr range",
                ));
            }
        }
        if let Some(expr) = &end {
            if !end_value.validate(domain) {
                return Err(SynError::new_spanned(
                    expr,
                    "better_enums: value is outside the repr range",
                ));
            }
        }

        let upper = if end.is_some() && matches!(range.limits, RangeLimits::HalfOpen(_)) {
            match end_value {
                Number::Signed(value) => value.checked_sub(1).map(Number::Signed),
                Number::Unsigned(value) => value.checked_sub(1).map(Number::Unsigned),
            }
            .ok_or_else(|| SynError::new_spanned(range, "better_enums: range is empty"))?
        } else {
            end_value
        };

        if start_value > upper {
            return Err(SynError::new_spanned(range, "better_enums: range is empty"));
        }

        Ok(RangeValue {
            start,
            end,
            inclusive: matches!(range.limits, RangeLimits::Closed(_)),
            lower: start_value,
            upper,
        })
    }
}

#[derive(Clone)]
pub(crate) enum RangeMapping {
    Single { expr: Expr, value: Number },
    Range(Box<RangeValue>),
}

impl RangeMapping {
    pub(crate) fn lower(&self) -> Number {
        match self {
            Self::Single { value, .. } => *value,
            Self::Range(range) => range.lower,
        }
    }

    pub(crate) fn upper(&self) -> Number {
        match self {
            Self::Single { value, .. } => *value,
            Self::Range(range) => range.upper,
        }
    }

    pub(crate) fn overlaps(&self, other: &Self) -> bool {
        self.lower() <= other.upper() && other.lower() <= self.upper()
    }

    pub(crate) fn parse(expr: &Expr, bounds: Domain) -> syn::Result<Vec<Self>> {
        match expr {
            Expr::Lit(_) | Expr::Unary(_) => {
                let value = Number::parse(expr, bounds.unsigned)?;
                if !value.validate(bounds) {
                    return Err(SynError::new_spanned(
                        expr,
                        "better_enums: value is outside the repr range",
                    ));
                }

                Ok(vec![RangeMapping::Single {
                    expr: expr.clone(),
                    value,
                }])
            }
            Expr::Range(range) => Ok(vec![RangeMapping::Range(Box::new(RangeValue::parse(
                range, bounds,
            )?))]),
            Expr::Array(array) => array
                .elems
                .iter()
                .map(|element| Self::parse(element, bounds))
                .try_fold(Vec::new(), |mut all, result| {
                    all.extend(result?);
                    Ok(all)
                }),
            _ => Err(SynError::new_spanned(
                expr,
                "better_enums: discriminant must be an integer, range, or array thereof",
            )),
        }
    }
}

pub(crate) struct RangeVariantMapping {
    pub(crate) name: Ident,
    pub(crate) span: Span,
    pub(crate) mappings: Vec<RangeMapping>,
}

impl RangeVariantMapping {
    pub(crate) fn overlaps(&self, other: &Self) -> bool {
        self.mappings.iter().any(|mapping| {
            other
                .mappings
                .iter()
                .any(|other_mapping| mapping.overlaps(other_mapping))
        })
    }

    pub(crate) fn valid(&self) -> bool {
        self.mappings.iter().enumerate().all(|(index, mapping)| {
            self.mappings[index + 1..]
                .iter()
                .all(|other_mapping| !mapping.overlaps(other_mapping))
        })
    }
}
