use syn;
use proc_macro2::Span;

use serialize::Size;
pub use parse_helpers::JumpType;


#[derive(Debug, Clone)]
pub enum Register {
    Scalar(RegScalar),
    Vector(RegVector)
}

#[derive(Debug, Clone)]
pub struct RegScalar {
    pub kind: RegKind,
    pub size: Size
}

#[derive(Debug, Clone)]
pub struct RegVector {
    pub kind: RegKind,
    pub element_size: Size,
    pub lanes: Option<u8>,
    pub element: Option<syn::Expr>
}

// Register id without indication of its usage.
#[derive(Debug, Clone)]
pub enum RegKind {
    Static(RegId),
    Dynamic(RegFamily, syn::Expr)
}

// map identifying all architecturally defined registers. Registers that overlap with different sizes
// are given the same ID. the upper 2 bits of the RegId match the RegFamily of said register.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RegId {
    // regular registers. Either 4 or 8 bytes
    X0 = 0x00, X1 = 0x01, X2 = 0x02, X3 = 0x03,
    X4 = 0x04, X5 = 0x05, X6 = 0x06, X7 = 0x07,
    X8 = 0x08, X9 = 0x09, X10= 0x0A, X11= 0x0B,
    X12= 0x0C, X13= 0x0D, X14= 0x0E, X15= 0x0F,
    X16= 0x10, X17= 0x11, X18= 0x12, X19= 0x13,
    X20= 0x14, X21= 0x15, X22= 0x16, X23= 0x17,
    X24= 0x18, X25= 0x19, X26= 0x1A, X27= 0x1B,
    X28= 0x1C, X29= 0x1D, X30= 0x1E,

    // zero register. Either 4 or 8 bytes
    XZR= 0x1F,

    // stack pointer. Either 4 or 8 bytes. the encoding overlaps XZR, and we only differentiate
    // the two of them to provide diagnostics. They count as the same family.
    SP = 0x3F,

    // scalar FP / vector SIMD registers. Can be used as 1, 2, 4, 8 or 16-byte size.
    V0 = 0x40, V1 = 0x41, V2 = 0x42, V3 = 0x43,
    V4 = 0x44, V5 = 0x45, V6 = 0x46, V7 = 0x47,
    V8 = 0x48, V9 = 0x49, V10= 0x4A, V11= 0x4B,
    V12= 0x4C, V13= 0x4D, V14= 0x4E, V15= 0x4F,
    V16= 0x50, V17= 0x51, V18= 0x52, V19= 0x53,
    V20= 0x54, V21= 0x55, V22= 0x56, V23= 0x57,
    V24= 0x58, V25= 0x59, V26= 0x5A, V27= 0x5B,
    V28= 0x5C, V29= 0x5D, V30= 0x5E, V31= 0x5F
}

// what family is this regid of (Scalar = Xn/Wn, Vector = Bn/Hn/Sn/Dn/Qn)
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RegFamily {
    SCALAR   = 0,
    SCALARSP = 1,
    VECTOR   = 2,
}

impl RegId {
    pub fn code(self) -> u8 {
        self as u8 & 0x1F
    }

    pub fn family(self) -> RegFamily {
        match self as u8 >> 5 {
            0 => RegFamily::SCALAR,
            1 => RegFamily::SCALARSP,
            2 => RegFamily::VECTOR,
            _ => unreachable!()
        }
    }
}

impl RegKind {
    pub fn code(&self) -> Option<u8> {
        match self {
            RegKind::Static(code) => Some(code.code()),
            RegKind::Dynamic(_, _) => None
        }
    }

    pub fn encode(&self) -> u8 {
        self.code().unwrap_or(0)
    }

    pub fn family(&self) -> RegFamily {
        match *self {
            RegKind::Static(code) => code.family(),
            RegKind::Dynamic(family, _) => family
        }
    }

    pub fn is_dynamic(&self) -> bool {
        match self {
            RegKind::Static(_) => false,
            RegKind::Dynamic(_, _) => true
        }
    }
}

impl PartialEq<RegKind> for RegKind {
    fn eq(&self, other: &RegKind) -> bool {
        match self {
            RegKind::Static(id) => match other {
                RegKind::Static(other_id) => other_id == id,
                RegKind::Dynamic(_, _) => false,
            },
            RegKind::Dynamic(_, _) => false,
        }
    }
}

impl RegScalar {
    pub fn new_static(size: Size, id: RegId) -> RegScalar {
        RegScalar {size, kind: RegKind::Static(id) }
    }

    pub fn new_dynamic(size: Size, family: RegFamily, id: syn::Expr) -> RegScalar {
        RegScalar {size, kind: RegKind::Dynamic(family, id) }
    }

    pub fn size(&self) -> Size {
        self.size
    }
}

impl RegVector {
    pub fn new_static(id: RegId, element_size: Size, lanes: Option<u8>, element: Option<syn::Expr>) -> RegVector {
        RegVector {kind: RegKind::Static(id), element_size, lanes, element}
    }

    pub fn new_dynamic(id: syn::Expr, element_size: Size, lanes: Option<u8>, element: Option<syn::Expr>) -> RegVector {
        RegVector {kind: RegKind::Dynamic(RegFamily::VECTOR, id), element_size, lanes, element}
    }

    pub fn size(&self) -> Size {
        self.element_size
    }
}

impl Register {
    pub fn size(&self) -> Size {
        match self {
            Register::Scalar(s) => s.size(),
            Register::Vector(v) => v.size()
        }
    }

    pub fn kind(&self) -> &RegKind {
        match self {
            Register::Scalar(s) => &s.kind,
            Register::Vector(v) => &v.kind
        }
    }

    pub fn family(&self) -> RegFamily {
        match self {
            Register::Scalar(s) => s.kind.family(),
            Register::Vector(v) => v.kind.family()
        }
    }

    pub fn is_dynamic(&self) -> bool {
        match self {
            Register::Scalar(s) => s.kind.is_dynamic(),
            Register::Vector(v) => v.kind.is_dynamic()
        }
    }

    pub fn is_vector(&self) -> bool {
        match self {
            Register::Scalar(_) => false,
            Register::Vector(_) => true
        }
    }

    pub fn assume_vector(&self) -> &RegVector {
        match self {
            Register::Scalar(_) => panic!("That wasn't a vector register"),
            Register::Vector(v) => v
        }
    }

    pub fn assume_scalar(&self) -> &RegScalar {
        match self {
            Register::Scalar(s) => s,
            Register::Vector(_) => panic!("That wasn't a vector register"),
        }
    }
}

/**
 * Modifier types
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modifier {
    LSL,
    LSR,
    ASR,
    ROR,
    SXTX,
    SXTW,
    SXTH,
    SXTB,
    UXTX,
    UXTW,
    UXTH,
    UXTB,
}

#[derive(Debug, Clone)]
pub struct ModifyExpr {
    pub op: Modifier,
    pub expr: Option<syn::Expr>
}

impl ModifyExpr {
    pub fn new(op: Modifier, expr: Option<syn::Expr>) -> ModifyExpr {
        ModifyExpr {
            op,
            expr
        }
    }
}

/**
 * Condition codes
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Condition {
    EQ,
    NE,
    CS,
    CC,
    MI,
    PL,
    VS,
    VC,
    HI,
    LS,
    GE,
    LT,
    GT,
    LE,
    AL,
    NV,
}

/**
 * Memory ref item types
 */

#[derive(Debug)]
pub enum RefItem {
    Direct {
        span: Span,
        reg: Register
    },
    Immediate {
        value: syn::Expr
    },
    Modifier {
        span: Span,
        modifier: ModifyExpr
    }
}

// basic parse results, before we start doing any kind of checking
#[derive(Debug)]
pub enum RawArg {
    // A memory reference
    Reference {
        span: Span,
        items: Vec<RefItem>,
        bang: bool
    },
    // A register list, defined as first - last
    DashList {
        span: Span,
        first: Register,
        last: Register,
        element: Option<syn::Expr>
    },
    // A register list, defined as item, item, item, item
    CommaList{
        span: Span,
        items: Vec<Register>,
        element: Option<syn::Expr>
    },
    AmountList {
        span: Span,
        first: Register,
        amount: syn::Expr,
        element: Option<syn::Expr>
    },
    // direct register reference
    Direct {
        span: Span,
        reg: Register
    },
    // jump target. Also used by PC-rel loads etc
    JumpTarget {
        type_: JumpType
    },
    // just an arbitrary expression
    Immediate {
        prefixed: bool,
        value: syn::Expr
    },
    // a modifier
    Modifier {
        span: Span,
        modifier: ModifyExpr
    },
    // a dot
    Dot {
        span: Span
    },
    // used to not block the parser on a parsing error in a single arg
    Invalid
}

// Contains the actual instruction mnemnonic.
#[derive(Debug)]
pub struct Instruction {
    pub span: Span,
    pub ident: syn::Ident
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndexMethod {
    None,
    PreIndexed,
    PostIndexed,
}

#[derive(Debug)]
pub enum RefKind {
    Base,
    Offset(syn::Expr),
    Indexed(Register, Option<ModifyExpr>),
    PreIndexed(syn::Expr),
}

// sanitized parse results
#[derive(Debug)]
pub enum CleanArg {
    Reference {
        span: Span,
        base: Register,
        kind: RefKind
    },
    RegList {
        span: Span,
        first: Register,
        amount: u8,
        element: Option<syn::Expr>
    },
    Direct {
        span: Span,
        reg: Register
    },
    JumpTarget {
        type_: JumpType
    },
    Immediate {
        prefixed: bool,
        value: syn::Expr,
    },
    Modifier {
        span: Span,
        modifier: ModifyExpr
    },
    Dot {
        span: Span
    }
}

// flat arg list after matching, for encoding
#[derive(Debug)]
pub enum FlatArg {
    Direct {
        span: Span,
        reg: Register
    },
    Immediate {
        value: syn::Expr,
    },
    Modifier {
        span: Span,
        modifier: Modifier,
    },
    JumpTarget {
        type_: JumpType
    }
}