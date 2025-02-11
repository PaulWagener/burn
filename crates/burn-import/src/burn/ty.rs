use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;

use crate::burn::ToTokens;

#[derive(Debug, Clone)]
pub struct TensorType {
    pub name: Ident,
    pub dim: usize,
    pub kind: TensorKind,
    pub shape: Option<Vec<usize>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TensorKind {
    Int,
    Float,
    Bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScalarKind {
    Int32,
    Int64,
    Float32,
    Float64,
    Bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScalarType {
    pub name: Ident,
    pub kind: ScalarKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShapeType {
    pub name: Ident,
    pub dim: usize,
}

#[derive(Debug, Clone)]
pub struct OtherType {
    pub name: Ident,
    pub ty: TokenStream,
}

#[derive(Debug, Clone)]
pub enum Type {
    /// Tensor type.
    Tensor(TensorType),

    /// Scalar type.
    Scalar(ScalarType),

    /// Shape type.
    Shape(ShapeType),

    // Other type (more flexible type).
    Other(OtherType),
}

impl Type {
    pub fn name(&self) -> &Ident {
        match self {
            Type::Tensor(tensor) => &tensor.name,
            Type::Scalar(scalar) => &scalar.name,
            Type::Shape(shape) => &shape.name,
            Type::Other(other) => &other.name,
        }
    }
    pub fn ty(&self) -> TokenStream {
        match self {
            Type::Tensor(tensor) => tensor.ty(),
            Type::Scalar(scalar) => scalar.ty(),
            Type::Shape(shape) => shape.ty(),
            Type::Other(other) => other.ty(),
        }
    }
}

impl ScalarType {
    pub fn new<S: AsRef<str>>(name: S, kind: ScalarKind) -> Self {
        if name.as_ref().is_empty() {
            panic!("Scalar of Type {:?} was passed with empty name", kind);
        }
        Self {
            name: Ident::new(name.as_ref(), Span::call_site()),
            kind,
        }
    }
    pub fn ty(&self) -> TokenStream {
        match self.kind {
            ScalarKind::Int32 => quote! { i32 },
            ScalarKind::Int64 => quote! { i64 },
            ScalarKind::Float32 => quote! { f32 },
            ScalarKind::Float64 => quote! { f64 },
            ScalarKind::Bool => quote! { bool },
        }
    }
}

impl ShapeType {
    pub fn new<S: AsRef<str>>(name: S, dim: usize) -> Self {
        if name.as_ref().is_empty() {
            panic!("Shape was passed with empty name");
        }
        Self {
            name: Ident::new(name.as_ref(), Span::call_site()),
            dim,
        }
    }
    pub fn ty(&self) -> TokenStream {
        let dim = self.dim.to_tokens();
        quote! { [usize; #dim] }
    }
}

impl TensorType {
    // This is used, because Tensors might have number literal name, which cannot be
    // used as a variable name.
    pub fn format_name(name: &str) -> String {
        let name_is_number = name.bytes().all(|digit| digit.is_ascii_digit());
        if name_is_number {
            format!("_{}", name)
        } else {
            name.to_string()
        }
    }

    pub fn new<S: AsRef<str>>(
        name: S,
        dim: usize,
        kind: TensorKind,
        shape: Option<Vec<usize>>,
    ) -> Self {
        if name.as_ref().is_empty() {
            panic!(
                "Tensor of Kind {:?} with dim shape {:?} was passed with empty name",
                kind, shape
            );
        }
        let formatted_name = Self::format_name(name.as_ref());
        assert_ne!(
            dim, 0,
            "Trying to create TensorType with dim = 0 - should be a Scalar instead!"
        );
        Self {
            name: Ident::new(&formatted_name, Span::call_site()),
            dim,
            kind,
            shape,
        }
    }
    pub fn new_float<S: AsRef<str>>(name: S, dim: usize) -> Self {
        Self::new_float_with_shape(name, dim, None)
    }

    pub fn new_float_with_shape<S: AsRef<str>>(
        name: S,
        dim: usize,
        shape: Option<Vec<usize>>,
    ) -> Self {
        Self::new(name, dim, TensorKind::Float, shape)
    }

    pub fn new_int<S: AsRef<str>>(name: S, dim: usize) -> Self {
        Self::new_int_with_shape(name, dim, None)
    }

    pub fn new_int_with_shape<S: AsRef<str>>(
        name: S,
        dim: usize,
        shape: Option<Vec<usize>>,
    ) -> Self {
        Self::new(name, dim, TensorKind::Int, shape)
    }

    pub fn new_bool<S: AsRef<str>>(name: S, dim: usize) -> Self {
        Self::new_bool_with_shape(name, dim, None)
    }

    pub fn new_bool_with_shape<S: AsRef<str>>(
        name: S,
        dim: usize,
        shape: Option<Vec<usize>>,
    ) -> Self {
        Self::new(name, dim, TensorKind::Bool, shape)
    }

    pub fn ty(&self) -> TokenStream {
        let dim = self.dim.to_tokens();
        match self {
            TensorType {
                kind: TensorKind::Float,
                ..
            } => quote! {
                Tensor<B, #dim>
            },
            TensorType {
                kind: TensorKind::Int,
                ..
            } => quote! {
                Tensor<B, #dim, Int>
            },
            TensorType {
                kind: TensorKind::Bool,
                ..
            } => quote! {
                Tensor<B, #dim, Bool>
            },
        }
    }
}

impl OtherType {
    pub fn new<S: AsRef<str>>(name: S, tokens: TokenStream) -> Self {
        if name.as_ref().is_empty() {
            panic!(
                "Other type with tokens {:?} was passed with empty name",
                tokens
            );
        }
        Self {
            name: Ident::new(name.as_ref(), Span::call_site()),
            ty: tokens,
        }
    }
    pub fn ty(&self) -> TokenStream {
        self.ty.clone()
    }
}
