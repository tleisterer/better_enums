use syn::{Expr, ExprRange, Ident, Lit, RangeLimits, UnOp};

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
                            syn::Error::new_spanned(
                                expr,
                                "better_enums: value is outside the repr range",
                            )
                        })?;
                        Ok(Self::Unsigned(number))
                    } else {
                        let number = value.base10_parse::<i128>().map_err(|_| {
                            syn::Error::new_spanned(
                                expr,
                                "better_enums: value is outside the repr range",
                            )
                        })?;
                        Ok(Self::Signed(number))
                    }
                }
                _ => Err(syn::Error::new_spanned(
                    expr,
                    "better_enums: expected an integer literal",
                )),
            },
            Expr::Unary(unary) if matches!(unary.op, UnOp::Neg(_)) => {
                let value = Self::parse(&unary.expr, true)?;
                match value {
                    Self::Signed(_) => unreachable!("Negated values is parsed as signed"),
                    Self::Unsigned(value) if value == (i128::MAX as u128) + 1 => {
                        Ok(Self::Signed(i128::MIN))
                    }
                    Self::Unsigned(value) => i128::try_from(value)
                        .map_err(|_| {
                            syn::Error::new_spanned(
                                expr,
                                "better_enums: value is outside the repr range",
                            )
                        })?
                        .checked_neg()
                        .map(Self::Signed)
                        .ok_or_else(|| {
                            syn::Error::new_spanned(
                                expr,
                                "better_enums: value is outside the repr range",
                            )
                        }),
                }
            }
            _ => Err(syn::Error::new_spanned(
                expr,
                "better_enums: expected an integer literal",
            )),
        }
    }

    pub(crate) fn validate(&self, domain: &Domain) -> bool {
        ((domain.unsigned && matches!(self, Self::Unsigned(_)))
            || (!domain.unsigned && matches!(self, Self::Signed(_))))
            && *self >= domain.min
            && *self <= domain.max
    }

    pub(crate) fn into_expr(self) -> Expr {
        let text = match self {
            Self::Signed(value) => value.to_string(),
            Self::Unsigned(value) => value.to_string(),
        };
        syn::parse_str(&text).expect("better_enums: validated discriminant should parse")
    }

    pub(crate) fn checked_add(self, rhs: u128) -> Option<Self> {
        match self {
            Self::Signed(value) => {
                let rhs = i128::try_from(rhs).ok()?;
                value.checked_add(rhs).map(Self::Signed)
            }
            Self::Unsigned(value) => value.checked_add(rhs).map(Self::Unsigned),
        }
    }

    /// Warning: this function will panic if called on a signed value.
    pub(crate) fn is_power_of_two(&self) -> bool {
        match self {
            Self::Signed(_) => panic!("better_enums: is_power_of_two called on signed value"),
            Self::Unsigned(value) => value.is_power_of_two(),
        }
    }

    /// Returns the value as a u128, panicking if the value is signed.
    pub(crate) fn get_unsigned(&self) -> u128 {
        match self {
            Self::Signed(_) => panic!("better_enums: unsigned called on signed value"),
            Self::Unsigned(value) => *value,
        }
    }

    #[allow(unused)]
    /// Returns the value as a i128, panicking if the value is unsigned.
    pub(crate) fn get_signed(&self) -> i128 {
        match self {
            Self::Signed(value) => *value,
            Self::Unsigned(_) => panic!("better_enums: signed called on unsigned value"),
        }
    }
}

#[derive(Clone)]
pub(crate) struct Domain {
    pub(crate) min: Number,
    pub(crate) max: Number,
    pub(crate) unsigned: bool,
    pub(crate) ident: Ident,
}

impl TryFrom<&Ident> for Domain {
    type Error = syn::Error;
    fn try_from(repr: &Ident) -> Result<Self, Self::Error> {
        let result = match repr.to_string().as_str() {
            "i8" => Domain {
                min: Number::Signed(i8::MIN as i128),
                max: Number::Signed(i8::MAX as i128),
                unsigned: false,
                ident: repr.clone(),
            },
            "i16" => Domain {
                min: Number::Signed(i16::MIN as i128),
                max: Number::Signed(i16::MAX as i128),
                unsigned: false,
                ident: repr.clone(),
            },
            "i32" => Domain {
                min: Number::Signed(i32::MIN as i128),
                max: Number::Signed(i32::MAX as i128),
                unsigned: false,
                ident: repr.clone(),
            },
            "i64" => Domain {
                min: Number::Signed(i64::MIN as i128),
                max: Number::Signed(i64::MAX as i128),
                unsigned: false,
                ident: repr.clone(),
            },
            "i128" => Domain {
                min: Number::Signed(i128::MIN),
                max: Number::Signed(i128::MAX),
                unsigned: false,
                ident: repr.clone(),
            },
            "isize" => Domain {
                min: Number::Signed(isize::MIN as i128),
                max: Number::Signed(isize::MAX as i128),
                unsigned: false,
                ident: repr.clone(),
            },
            "u8" => Domain {
                min: Number::Unsigned(0),
                max: Number::Unsigned(u8::MAX as u128),
                unsigned: true,
                ident: repr.clone(),
            },
            "u16" => Domain {
                min: Number::Unsigned(0),
                max: Number::Unsigned(u16::MAX as u128),
                unsigned: true,
                ident: repr.clone(),
            },
            "u32" => Domain {
                min: Number::Unsigned(0),
                max: Number::Unsigned(u32::MAX as u128),
                unsigned: true,
                ident: repr.clone(),
            },
            "u64" => Domain {
                min: Number::Unsigned(0),
                max: Number::Unsigned(u64::MAX as u128),
                unsigned: true,
                ident: repr.clone(),
            },
            "u128" => Domain {
                min: Number::Unsigned(0),
                max: Number::Unsigned(u128::MAX),
                unsigned: true,
                ident: repr.clone(),
            },
            "usize" => Domain {
                min: Number::Unsigned(0),
                max: Number::Unsigned(usize::MAX as u128),
                unsigned: true,
                ident: repr.clone(),
            },
            _ => {
                return Err(syn::Error::new_spanned(
                    repr,
                    "better_enums: repr must be an integer type",
                ));
            }
        };
        Ok(result)
    }
}

#[derive(Clone)]
pub(crate) struct RangeValue {
    pub(crate) expr: ExprRange,
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
            if !start_value.validate(&domain) {
                return Err(syn::Error::new_spanned(
                    expr,
                    "better_enums: value is outside the repr range",
                ));
            }
        }
        if let Some(expr) = &end {
            if !end_value.validate(&domain) {
                return Err(syn::Error::new_spanned(
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
            .ok_or_else(|| syn::Error::new_spanned(range, "better_enums: range is empty"))?
        } else {
            end_value
        };

        if start_value > upper {
            return Err(syn::Error::new_spanned(
                range,
                "better_enums: range is empty",
            ));
        }

        Ok(RangeValue {
            expr: range.clone(),
            lower: start_value,
            upper,
        })
    }
}

pub(crate) enum Variant {
    Single { expr: Expr, value: Number },
    Range(RangeValue),
}

impl Variant {
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

    pub(crate) fn parse(expr: &Expr, bounds: &Domain) -> syn::Result<Vec<Self>> {
        match expr {
            Expr::Lit(_) | Expr::Unary(_) => {
                let value = Number::parse(expr, bounds.unsigned)?;
                if !value.validate(bounds) {
                    return Err(syn::Error::new_spanned(
                        expr,
                        "better_enums: value is outside the repr range",
                    ));
                }

                Ok(vec![Variant::Single {
                    expr: expr.clone(),
                    value,
                }])
            }
            Expr::Range(range) => Ok(vec![Variant::Range(RangeValue::parse(
                range,
                bounds.clone(),
            )?)]),
            Expr::Array(array) => array
                .elems
                .iter()
                .map(|element| Self::parse(element, bounds))
                .try_fold(Vec::new(), |mut all, result| {
                    all.extend(result?);
                    Ok(all)
                }),
            _ => Err(syn::Error::new_spanned(
                expr,
                "better_enums: discriminant must be an integer, range, or array thereof",
            )),
        }
    }
}

pub(crate) struct VariantMapping {
    pub(crate) name: Ident,
    pub(crate) mappings: Vec<Variant>,
}

impl VariantMapping {
    pub(crate) fn overlaps(&self, other: &Self) -> bool {
        self.mappings.iter().any(|mapping| {
            other
                .mappings
                .iter()
                .any(|other_mapping| mapping.overlaps(other_mapping))
        })
    }

    pub(crate) fn check_overlap(&self, other: &Self) -> Result<(), syn::Error> {
        if self.overlaps(other) {
            return Err(syn::Error::new_spanned(
                &self.name,
                format!("better_enums: {} overlaps with {}", self.name, other.name),
            ));
        }
        Ok(())
    }

    pub(crate) fn valid(&self) -> bool {
        self.mappings.iter().enumerate().all(|(index, mapping)| {
            self.mappings[index + 1..]
                .iter()
                .all(|other_mapping| !mapping.overlaps(other_mapping))
        })
    }

    pub(crate) fn check_valid(&self) -> Result<(), syn::Error> {
        if !self.valid() {
            return Err(syn::Error::new_spanned(
                &self.name,
                format!("better_enums: {} has overlapping values", self.name),
            ));
        }
        Ok(())
    }
}
