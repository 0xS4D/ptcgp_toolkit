use crate::unity::generated::il2cpp_2022333f1::root::TypeIndex;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct ComplexTypeArgs {
    pub args: Vec<ComplexType>,
}

impl From<Vec<ComplexType>> for ComplexTypeArgs {
    fn from(args: Vec<ComplexType>) -> Self {
        Self { args }
    }
}

impl ComplexTypeArgs {
    pub fn get_name_str(&self, with_namespace: bool) -> Result<String, std::fmt::Error> {
        self.args
            .iter()
            .map(|arg| arg.get_name_str(with_namespace))
            .collect::<Result<Vec<_>, _>>()
            .map(|v| v.join(", "))
    }

    pub fn get_module_name(&self) -> Option<String> {
        self.args.iter().find_map(|arg| match arg {
            ComplexType::Simple {
                module: Some(m), ..
            } => Some(m.clone()),
            _ => None,
        })
    }
}

impl Display for ComplexTypeArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name_str(true)?)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComplexTypeNamespace {
    Simple(String),
    Complex(Box<ComplexType>),
}

impl Display for ComplexTypeNamespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple(s) => f.write_str(s),
            Self::Complex(inner) => write!(f, "{}", inner),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComplexType {
    Simple {
        module: Option<String>,
        namespace: Option<ComplexTypeNamespace>,
        type_index: Option<TypeIndex>,
        name: String,
    },
    Pointer(Box<ComplexType>),
    Array(Box<ComplexType>),
    Generic {
        base: Box<ComplexType>,
        args: ComplexTypeArgs,
    },
}

impl ComplexType {
    pub fn get_namespace(&self) -> Option<&str> {
        match self {
            Self::Simple {
                namespace: Some(ns),
                ..
            } => match ns {
                ComplexTypeNamespace::Simple(s) => Some(s),
                ComplexTypeNamespace::Complex(inner) => inner.get_namespace(),
            },
            Self::Pointer(inner) | Self::Array(inner) | Self::Generic { base: inner, .. } => {
                inner.get_namespace()
            }
            _ => None,
        }
    }

    pub fn get_root_namespace(&self) -> Option<&str> {
        match self {
            Self::Simple {
                namespace: Some(ns),
                ..
            } => match ns {
                ComplexTypeNamespace::Simple(s) => Some(s),
                ComplexTypeNamespace::Complex(inner) => inner.get_root_namespace(),
            },
            Self::Simple { name, .. } => Some(name),
            Self::Pointer(inner) | Self::Array(inner) | Self::Generic { base: inner, .. } => {
                inner.get_root_namespace()
            }
        }
    }

    pub fn get_name_str(&self, with_namespace: bool) -> Result<String, std::fmt::Error> {
        use std::fmt::Write;
        let mut s = String::with_capacity(64);

        match self {
            Self::Simple {
                namespace, name, ..
            } => {
                if with_namespace {
                    if let Some(ns) = namespace {
                        write!(s, "{}.{}", ns, name)?;
                    } else {
                        s.push_str(name);
                    }
                } else {
                    s.push_str(name);
                }
            }
            Self::Pointer(inner) => write!(s, "{}*", inner)?,
            Self::Array(inner) => write!(s, "{}[]", inner)?,
            Self::Generic { base, args } => {
                write!(s, "{}<{args}>", base.get_name_str(with_namespace)?)?
            }
        }

        Ok(s)
    }

    pub fn get_type_index(&self) -> Option<TypeIndex> {
        match self {
            Self::Simple { type_index, .. } => *type_index,
            Self::Pointer(inner) | Self::Array(inner) => inner.get_type_index(),
            Self::Generic { .. } => unimplemented!("Nested type indexes aren't yet supported"),
        }
    }
}

impl Display for ComplexType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name_str(true)?)
    }
}
