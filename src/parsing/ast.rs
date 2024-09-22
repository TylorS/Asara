use std::collections::HashMap;
use crate::lexing::token::Span;

/**
 * TODO:
 * Effect
 * Handlers
 * Match
 * Perform
 * Resume
 * Yield
 */

// Basic types and literals
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum RecordKey<'a> {
    String(&'a str, Span),
    Symbol(&'a str, Span),
}

// Expressions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'a> {
    // Literals
    String {
        value: &'a str,
        span: Span,
    },
    Integer {
        value: &'a i64,
        span: Span,
    },
    Decimal {
        integer: &'a i64,
        fractional: &'a i64,
        span: Span,
    },
    BigInteger {
        value: &'a i128,
        span: Span,
    },
    BigDecimal {
        integer: &'a i128,
        fractional: &'a i128,
        span: Span,
    },
    Boolean {
        value: &'a bool,
        span: Span,
    },
    Array {
        elements: Vec<Expression<'a>>,
        span: Span,
    },
    Tuple {
        elements: Vec<Expression<'a>>,
        span: Span,
    },
    Record {
        fields: HashMap<RecordKey<'a>, Expression<'a>>,
        span: Span,
    },
    Symbol {
        name: &'a str,
        span: Span,
    },

    // Operators
    Binary {
        left: Box<Expression<'a>>,
        op: BinaryOp,
        right: Box<Expression<'a>>,
        span: Span,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expression<'a>>,
        span: Span,
    },

    FunctionCall {
        function: Box<Expression<'a>>,
        type_arguments: Vec<Type<'a>>,
        arguments: Vec<Expression<'a>>,
        span: Span,
    },

    Resume {
        expression: Box<Expression<'a>>,
        span: Span,
    },

    Yield {
        expression: Box<Expression<'a>>,
        span: Span,
    },

    Perform {
        expression: Box<Expression<'a>>,
        span: Span,
    },

    Handle {
        effect: Box<Expression<'a>>,
        expression: Box<Expression<'a>>,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Addition(Span),
    Subtraction(Span),
    Multiplication(Span),
    Division(Span),
    Modulus(Span),
    Exponentiation(Span),
    Equal(Span),
    NotEqual(Span),
    LessThan(Span),
    LessThanOrEqual(Span),
    GreaterThan(Span),
    GreaterThanOrEqual(Span),
    LogicalAnd(Span),
    LogicalOr(Span),
    BitwiseAnd(Span),
    BitwiseOr(Span),
    BitwiseXor(Span),
    LeftShift(Span),
    RightShift(Span),
    NullishCoalescing(Span),
    PipeOperator(Span),
    OptionalChaining(Span),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    Negation(Span),
    LogicalNot(Span),
    BitwiseNot(Span),
    PreIncrement(Span),
    PostIncrement(Span),
    PreDecrement(Span),
    PostDecrement(Span),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type<'a> {
    // Primitive types
    Integer {
        span: Span,
    },
    Float {
        span: Span,
    },
    BigInteger {
        span: Span,
    },
    BigDecimal {
        span: Span,
    },
    Boolean {
        span: Span,
    },
    String {
        span: Span,
    },
    Symbol {
        name: &'a str,
        span: Span,
    },

    StringLiteral {
        value: &'a str,
        span: Span,
    },
    IntegerLiteral {
        value: &'a i64,
        span: Span,
    },
    DecimalLiteral {
        integer: &'a i64,
        fraction: &'a i64,
        span: Span,
    },
    BigIntegerLiteral {
        value: &'a i128,
        span: Span,
    },
    BigDecimalLiteral {
        integer: &'a i128,
        fraction: &'a i128,
        span: Span,
    },
    BooleanLiteral {
        value: &'a bool,
        span: Span,
    },
    ArrayLiteral {
        element_type: Box<Type<'a>>,
        elements: Vec<Type<'a>>,
        span: Span,
    },
    TupleLiteral {
        elements: Vec<Type<'a>>,
        span: Span,
    },
    RecordLiteral {
        fields: HashMap<RecordKey<'a>, Type<'a>>,
        span: Span,
    },
    SymbolLiteral {
        name: &'a str,
        span: Span,
    },

    // Complex types
    Array {
        element_type: Box<Type<'a>>,
        span: Span,
    },
    Tuple {
        elements: Vec<Type<'a>>,
        span: Span,
    },
    Record {
        fields: HashMap<RecordKey<'a>, Type<'a>>,
        span: Span,
    },

    // Generic types
    TypeVariable {
        name: &'a str,
        constraint: Option<TypeConstraint<'a>>,
        variance: Option<Variance>,
        span: Span,
    },

    HigherKindedType {
        name: &'a str,
        parameters: Vec<TypeParameter<'a>>,
        span: Span,
    },

    // Union and intersection types
    Union {
        types: Vec<Type<'a>>,
        span: Span,
    },
    Intersection {
        types: Vec<Type<'a>>,
        span: Span,
    },

    // Function types
    Function {
        parameters: Vec<TypeParameter<'a>>,
        return_type: Box<Type<'a>>,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeConstraint<'a> {
    Subtype { ty: Box<Type<'a>>, span: Span },
    Supertype { ty: Box<Type<'a>>, span: Span },
    Invariant { ty: Box<Type<'a>>, span: Span },
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Variance {
    In,
    Out,
    Invariant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeParameter<'a> {
    Placeholder {
        span: Span,
    },
    Generic {
        name: &'a str,
        constraint: Option<TypeConstraint<'a>>,
        variance: Option<Variance>,
        span: Span,
    },
    HigherKinded {
        name: &'a str,
        parameters: Vec<TypeParameter<'a>>,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<'a> {
    Expression {
        expr: Box<Expression<'a>>,
        span: Span,
    },
    Return {
        expr: Box<Expression<'a>>,
        span: Span,
    },
    Break {
        span: Span,
    },
    Continue {
        span: Span,
    },
    If {
        condition: Box<Expression<'a>>,
        then_branch: Vec<Statement<'a>>,
        else_if_branches: Vec<(Box<Expression<'a>>, Vec<Statement<'a>>)>,
        else_branch: Option<Vec<Statement<'a>>>,
        span: Span,
    },
    While {
        condition: Box<Expression<'a>>,
        body: Vec<Statement<'a>>,
        span: Span,
    },
    For {
        initializer: Box<Expression<'a>>,
        condition: Box<Expression<'a>>,
        increment: Box<Expression<'a>>,
        body: Vec<Statement<'a>>,
        span: Span,
    },
    ForOf {
        variable: Box<Expression<'a>>,
        iterable: Box<Expression<'a>>,
        body: Vec<Statement<'a>>,
        span: Span,
    },
    ForIn {
        variable: Box<Expression<'a>>,
        iterable: Box<Expression<'a>>,
        body: Vec<Statement<'a>>,
        span: Span,
    },
    Import {
        module: &'a str,
        declaration: ImportDeclaration<'a>,
        span: Span,
    },
    Declaration(Declaration<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration<'a> {
    Function {
        exported: bool,
        name: &'a str,
        type_parameters: Vec<TypeParameter<'a>>,
        parameters: Vec<Field<'a>>,
        return_type: Option<Type<'a>>,
        body: Vec<Statement<'a>>,
        span: Span,
    },
    Brand {
        exported: bool,
        name: &'a str,
        type_parameters: Vec<TypeParameter<'a>>,
        data_constructors: Vec<DataConstructor<'a>>,
        span: Span,
    },
    Data {
        exported: bool,
        name: &'a str,
        type_parameters: Vec<TypeParameter<'a>>,
        data_constructors: Vec<DataConstructor<'a>>,
        span: Span,
    },
    Let {
        exported: bool,
        name: &'a str,
        annotation: Option<Type<'a>>,
        value: Expression<'a>,
        span: Span,
    },
    TypeAlias {
        exported: bool,
        name: &'a str,
        type_parameters: Vec<TypeParameter<'a>>,
        alias: Type<'a>,
        span: Span,
    },
    Effect {
        exported: bool,
        name: &'a str,
        type_parameters: Vec<TypeParameter<'a>>,
        fields: Vec<EffectField<'a>>,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataConstructor<'a> {
    Void {
        name: &'a str,
        span: Span,
    },
    Tuple {
        name: &'a str,
        fields: Vec<Field<'a>>,
        span: Span,
    },
    Record {
        name: &'a str,
        fields: HashMap<RecordKey<'a>, Field<'a>>,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportDeclaration<'a> {
    NamedImports {
        imports: Vec<NamedImport<'a>>,
        span: Span,
    },
    NamespaceImport {
        name: &'a str,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedImport<'a> {
    name: &'a str,
    alias: Option<String>,
    span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field<'a> {
    Named {
        name: &'a str,
        annotation: Option<Type<'a>>,
        default: Option<Expression<'a>>,
        span: Span,
    },
    Typed {
        index: u32,
        annotation: Type<'a>,
        default: Option<Expression<'a>>,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectField<'a> {
    pub name: &'a str,
    pub declaration: Type<'a>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>,
    pub span: Span,
}

pub trait Spanned {
    fn span(&self) -> Span;
}

impl<T: Spanned> Spanned for Option<T> {
    fn span(&self) -> Span {
        self.as_ref().map(|t| t.span()).unwrap_or(Span::default())
    }
}

impl Spanned for Expression<'_> {
    fn span(&self) -> Span {
        match self {
            Expression::String { span, .. } => span.clone(),
            Expression::Integer { span, .. } => span.clone(),
            Expression::Decimal { span, .. } => span.clone(),
            Expression::BigInteger { span, .. } => span.clone(),
            Expression::BigDecimal { span, .. } => span.clone(),
            Expression::Boolean { span, .. } => span.clone(),
            Expression::Array { span, .. } => span.clone(),
            Expression::Tuple { span, .. } => span.clone(),
            Expression::Record { span, .. } => span.clone(),
            Expression::Symbol { span, .. } => span.clone(),
            Expression::Binary { span, .. } => span.clone(),
            Expression::Unary { span, .. } => span.clone(),
            Expression::FunctionCall { span, .. } => span.clone(),
            Expression::Resume { span, .. } => span.clone(),
            Expression::Yield { span, .. } => span.clone(),
            Expression::Perform { span, .. } => span.clone(),
            Expression::Handle { span, .. } => span.clone(),
        }
    }
}

impl Spanned for Statement<'_> {
    fn span(&self) -> Span {
        match self {
            Statement::Expression { span, .. } => span.clone(),
            Statement::Return { span, .. } => span.clone(),
            Statement::Break { span, .. } => span.clone(),
            Statement::Continue { span, .. } => span.clone(),
            Statement::If { span, .. } => span.clone(),
            Statement::While { span, .. } => span.clone(),
            Statement::For { span, .. } => span.clone(),
            Statement::ForOf { span, .. } => span.clone(),
            Statement::ForIn { span, .. } => span.clone(),
            Statement::Import { span, .. } => span.clone(),
            Statement::Declaration(decl) => decl.span(),
        }
    }
}

impl Spanned for Declaration<'_> {
    fn span(&self) -> Span {
        match self {
            Declaration::Function { span, .. } => span.clone(),
            Declaration::Brand { span, .. } => span.clone(),
            Declaration::Data { span, .. } => span.clone(),
            Declaration::Let { span, .. } => span.clone(),
            Declaration::TypeAlias { span, .. } => span.clone(),
            Declaration::Effect { span, .. } => span.clone(),
        }
    }
}

impl Spanned for DataConstructor<'_> {
    fn span(&self) -> Span {
        match self {
            DataConstructor::Void { span, .. } => span.clone(),
            DataConstructor::Tuple { span, .. } => span.clone(),
            DataConstructor::Record { span, .. } => span.clone(),
        }
    }
}

impl Spanned for Field<'_> {
    fn span(&self) -> Span {
        match self {
            Field::Named { span, .. } => span.clone(),
            Field::Typed { span, .. } => span.clone(),
        }
    }
}

impl Spanned for EffectField<'_> {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Spanned for ImportDeclaration<'_> {
    fn span(&self) -> Span {
        match self {
            ImportDeclaration::NamedImports { span, .. } => span.clone(),
            ImportDeclaration::NamespaceImport { span, .. } => span.clone(),
        }
    }
}

impl Spanned for NamedImport<'_> {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Spanned for Type<'_> {
    fn span(&self) -> Span {
        match self {
            Type::Integer { span, .. } => span.clone(),
            Type::Float { span, .. } => span.clone(),
            Type::BigInteger { span, .. } => span.clone(),
            Type::BigDecimal { span, .. } => span.clone(),
            Type::Boolean { span, .. } => span.clone(),
            Type::String { span, .. } => span.clone(),
            Type::Symbol { span, .. } => span.clone(),

            Type::StringLiteral { span, .. } => span.clone(),
            Type::IntegerLiteral { span, .. } => span.clone(),
            Type::DecimalLiteral { span, .. } => span.clone(),
            Type::BigIntegerLiteral { span, .. } => span.clone(),
            Type::BigDecimalLiteral { span, .. } => span.clone(),
            Type::BooleanLiteral { span, .. } => span.clone(),
            Type::ArrayLiteral { span, .. } => span.clone(),
            Type::TupleLiteral { span, .. } => span.clone(),
            Type::RecordLiteral { span, .. } => span.clone(),
            Type::SymbolLiteral { span, .. } => span.clone(),

            Type::Array { span, .. } => span.clone(),
            Type::Tuple { span, .. } => span.clone(),
            Type::Record { span, .. } => span.clone(),

            Type::TypeVariable { span, .. } => span.clone(),
            Type::HigherKindedType { span, .. } => span.clone(),
            Type::Union { span, .. } => span.clone(),
            Type::Intersection { span, .. } => span.clone(),
            Type::Function { span, .. } => span.clone(),
        }
    }
}

impl Spanned for TypeConstraint<'_> {
    fn span(&self) -> Span {
        match self {
            TypeConstraint::Subtype { span, .. } => span.clone(),
            TypeConstraint::Supertype { span, .. } => span.clone(),
            TypeConstraint::Invariant { span, .. } => span.clone(),
        }
    }
}

impl Spanned for TypeParameter<'_> {
    fn span(&self) -> Span {
        match self {
            TypeParameter::Placeholder { span, .. } => span.clone(),
            TypeParameter::Generic { span, .. } => span.clone(),
            TypeParameter::HigherKinded { span, .. } => span.clone(),
        }
    }
}
