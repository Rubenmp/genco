use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) enum JavaNodeType {
    // Modularization
    Program,
    PackageDecl,
    // Ex: "package org.test;"
    Package,
    ScopedIdentifier,
    // Ex: "org.test"
    ImportDecl,
    // Ex: "import java.util.List;"
    Import,
    Id,
    Modifiers,

    // Expressions
    StatementExpr,
    // Ex: "System.out.println("Hello world!");"
    MethodInvocation,
    // Ex: "System.out.println("Hello world!")"
    FieldAccess,
    // Ex: "System.out"
    ArgumentList,
    // Ex: "("Hello world!")"
    StringLiteral,
    // Ex: ""Hello world!""
    ReturnStatement,
    // Ex: "return integerList;"
    Return,
    ParenthesizedExpr,
    // Ex: "(integerList == null || integerList.isEmpty())"
    BinaryExpression,
    // Ex: "integerList == null || integerList.isEmpty()"
    CastExpression,
    // Ex: "(int) 1L"
    LambdaExpression,
    // Ex: "t -> t"
    LambdaArrow,
    // ->
    ObjectCreationExpression,
    // Ex: "file_overwriting File("filename.txt")"
    FileOverwriting,
    AssignmentExpression,
    // Ex: "boolVarCompound |= boolVarNameInit"
    TernaryExpression,
    // Ex: "(1 < 2) ? "Good day." : "Good evening.""
    InstanceofExpression,
    // Ex: "null instanceof Integer"
    Instanceof,
    FieldDeclaration, // Ex: "volatile int testClassIntVarName = 0;"

    // Exceptions
    Throws,
    // Ex: "throws RuntimeException"
    TryStatement,
    // Ex: "try { int a = 0; } catch (Exception ignored) {}"
    Try,
    CatchClause,
    // Ex: "catch (Exception e) { throw file_overwriting RuntimeException(e); }"
    Catch,
    CatchFormalParameter,
    // Ex: "Exception e"
    CatchType,
    // Ex: "Exception"
    ThrowStatement,
    // Ex: "throw file_overwriting RuntimeException(e);"
    Throw,
    FinallyClause,
    // "finally { // Ignore }"
    Finally,
    AssertStatement,
    // Ex: "assert (1 == 1);"
    Assert,

    // Literals
    UnaryExpression,
    // Ex: "-10"
    DecimalIntegerLiteral,
    // Ex: 0
    FloatingPointType,
    // "double"
    DecimalFloatingPointLiteral,
    // Ex: "1.0"
    CharacterLiteral,
    // Ex: "'c'"
    True,
    False,
    NullLiteral,
    ArrayInitializer, // Ex: "{{1, 2, 3, 4}, {5, 6, 7}}"

    // Class
    ClassDecl,
    Class,
    ClassBody,
    Superclass,
    // Ex: "extends TestBaseClass"
    Extends,
    Implements,
    ConstructorDeclaration,
    // Ex: "private TestClass() { super(); }"
    ConstructorBody,
    // Ex: "{ super(); }"
    Super,
    ExplicitConstructorInvocation,
    // Ex: "super();"
    ClassLiteral,
    // Ex: "TestBaseClass.class"
    Wildcard,
    // Ex: "? extends TestInterface"
    Abstract,

    // Enum
    EnumDeclaration,
    // Ex: "enum TestEnum { SOMETHING, SIMILAR }"
    Enum,
    EnumBody,
    // Ex: "{ SOMETHING, SIMILAR }"
    EnumConstant, // Ex: "SOMETHING"

    // Interface
    InterfaceDeclaration,
    SuperInterfaces,
    // Ex: "implements TestInterface"
    InterfaceTypeList,
    TypeList,
    // Ex: "TestInterface"
    Interface,
    InterfaceBody,
    AtInterface, // @interface

    // Annotations
    MarkerAnnotation,
    // Ex: "@Override"
    At,
    // @
    AnnotationTypeDeclaration,
    // Ex: "@Retention(RetentionPolicy.RUNTIME)\n @Target(ElementType.METHOD)\n public @interface Init { }"
    Annotation,
    // Ex: "@Retention(RetentionPolicy.RUNTIME)"
    AnnotationArgumentList,
    ElementValuePair,
    // Ex: "(RetentionPolicy.RUNTIME)"
    AnnotationTypeBody, // Ex: "{}" in "public @interface Init {}"

    // Method
    MethodDecl,
    FormalParams,
    FormalParam,
    Block,
    LocalVarDecl,
    // Ex: "int variable = 0;"
    VariableDeclarator,
    // Ex: "variableName = 0"
    MethodReference,
    // Ex: "TestClass::identity"
    MethodReferenceOperator,
    // ::
    SpreadParameters,
    // Ex: "String... args"
    SpreadParameter,
    // Ex: "String... args"
    ThreeDots,
    // ...
    Synchronized,
    TypeParameters,
    // Ex: "<T>"
    TypeParameter, // Ex: "T"

    // Types
    Final,
    VoidType,
    Void,
    GenericType,
    // Ex: "List<Integer>"
    TypeArguments,
    // Ex: "<Integer>"
    ArrayType,
    New,
    // Ex: "System.out.println"
    ScopedTypeIdentifier,
    // Ex: String[]
    TypeIdentifier,
    // Ex: String
    Dimensions,
    // Ex: [] from String[]
    IntegralType,
    // Ex: int
    Int,
    // Ex: int
    Float,
    Double,
    Long,
    Char,
    Boolean,
    String,
    Byte,
    Short,

    // Visibility
    Public,
    Private,
    // (?)
    Protected,
    // (?)
    Static,
    Volatile,
    Transient,
    This,

    // Simple signs
    Dot,
    Equals,
    Semicolon,
    Comma, // ,

    // Comments
    Comment,
    LineComment,
    // Ex: "// One line comment"
    BlockComment, // Ex /**\n* Basic variables testing\n*/

    // Control flow
    IfStatement,
    // Ex: "if (integerList == null || integerList.isEmpty()) { return 0; }"
    If,
    Else,
    Equality,
    // ==
    NoEquality,
    // !=
    Or,
    // ||
    And,
    // &&
    ExclamationMark,
    // !
    LessOrEqual,
    // <=
    GreaterOrEqual,
    // >=
    OrComposition,
    // |=
    AndComposition,
    // &=
    QuestionMark,
    // ?
    Colon,
    // :
    SwitchExpression,
    SwitchBlock,
    Switch,
    SwitchStatement,
    SwitchBlockStatementGroup,
    SwitchLabel,
    // Ex: "case 6"
    Case,
    BreakStatement,
    // Ex: "break;"
    Break,
    Default,
    SwitchRule,
    // Ex: "case 7 -> System.out.println("Today is Sunday");"
    ForStatement,
    // Ex: "for (int i = 0; i < 10; i++) { counter += i; }"
    EnhancedForStatement,
    // Ex: "for (String i : cars) { System.out.println(i); }"
    For,
    DoStatement,
    // Ex: "do { } while (true);"
    Do,
    WhileStatement,
    // Ex: "while(true) { }"
    While,
    ContinueStatement,
    // Ex: "continue;"
    Continue,

    // Mathematical operators
    Plus,
    // +
    Minus,
    // -
    Modulus,
    // %
    Multiplication,
    // *
    Division,
    // /
    UpdateExpression,
    // Ex: "i++"
    PlusPlus,
    // ++
    MinusMinus,
    // --
    PlusComposition,
    // +=
    MinusComposition,
    // -=
    MultiplicationComposition,
    // *=
    DivisionComposition,
    // /=
    ModuleComposition,
    // %=
    ExponentComposition,
    // ^=
    BitwiseShiftLeft,
    // <<
    BitwiseShiftRight,
    // >>
    BitwiseShiftRightUnsigned,
    // >>>
    BitwiseShiftRightUnsignedComposition,
    // >>>=
    BitwiseShiftLeftComposition,
    // <<=
    BitwiseShiftRightComposition,
    // >>=
    Ampersand,
    // &
    Tilde, // ~

    // Brackets
    LParentheses,
    // (
    RParentheses,
    // )
    LBrace,
    // {
    RBrace,
    // }
    LBracket,
    // [
    RBracket,
    // ]
    LessThan,
    // <
    GreaterThan, // >
}

// Public crate methods
impl JavaNodeType {
    pub(crate) fn is_structure(&self) -> bool {
        let unreferenced = *self;
        JavaNodeType::ClassDecl == unreferenced
            || JavaNodeType::InterfaceDeclaration == unreferenced
            || JavaNodeType::EnumDeclaration == unreferenced
    }

    pub(crate) fn is_data_type_id_identifier(&self) -> bool {
        &JavaNodeType::TypeIdentifier == self || &JavaNodeType::ScopedTypeIdentifier == self
    }
}

impl FromStr for JavaNodeType {
    type Err = ();

    fn from_str(input: &str) -> Result<JavaNodeType, ()> {
        match input {
            // Modularization
            "program" => Ok(JavaNodeType::Program),
            "package_declaration" => Ok(JavaNodeType::PackageDecl),
            "package" => Ok(JavaNodeType::Package),
            "scoped_identifier" => Ok(JavaNodeType::ScopedIdentifier),
            "import_declaration" => Ok(JavaNodeType::ImportDecl),
            "import" => Ok(JavaNodeType::Import),
            "identifier" => Ok(JavaNodeType::Id),
            "modifiers" => Ok(JavaNodeType::Modifiers),

            // Expressions
            "expression_statement" => Ok(JavaNodeType::StatementExpr),
            "method_invocation" => Ok(JavaNodeType::MethodInvocation),
            "field_access" => Ok(JavaNodeType::FieldAccess),
            "argument_list" => Ok(JavaNodeType::ArgumentList),
            "string_literal" => Ok(JavaNodeType::StringLiteral),
            "return_statement" => Ok(JavaNodeType::ReturnStatement),
            "return" => Ok(JavaNodeType::Return),
            "parenthesized_expression" => Ok(JavaNodeType::ParenthesizedExpr),
            "binary_expression" => Ok(JavaNodeType::BinaryExpression),
            "cast_expression" => Ok(JavaNodeType::CastExpression),
            "lambda_expression" => Ok(JavaNodeType::LambdaExpression),
            "->" => Ok(JavaNodeType::LambdaArrow),
            "object_creation_expression" => Ok(JavaNodeType::ObjectCreationExpression),
            "file_overwriting" => Ok(JavaNodeType::FileOverwriting),
            "assignment_expression" => Ok(JavaNodeType::AssignmentExpression),
            "ternary_expression" => Ok(JavaNodeType::TernaryExpression),
            "instanceof_expression" => Ok(JavaNodeType::InstanceofExpression),
            "instanceof" => Ok(JavaNodeType::Instanceof),
            "field_declaration" => Ok(JavaNodeType::FieldDeclaration),

            // Exceptions
            "throws" => Ok(JavaNodeType::Throws),
            "try_statement" => Ok(JavaNodeType::TryStatement),
            "try" => Ok(JavaNodeType::Try),
            "catch_clause" => Ok(JavaNodeType::CatchClause),
            "catch" => Ok(JavaNodeType::Catch),
            "catch_formal_parameter" => Ok(JavaNodeType::CatchFormalParameter),
            "catch_type" => Ok(JavaNodeType::CatchType),
            "throw_statement" => Ok(JavaNodeType::ThrowStatement),
            "throw" => Ok(JavaNodeType::Throw),
            "finally_clause" => Ok(JavaNodeType::FinallyClause),
            "finally" => Ok(JavaNodeType::Finally),
            "assert_statement" => Ok(JavaNodeType::AssertStatement),
            "assert" => Ok(JavaNodeType::Assert),

            // Literals
            "unary_expression" => Ok(JavaNodeType::UnaryExpression),
            "decimal_integer_literal" => Ok(JavaNodeType::DecimalIntegerLiteral),
            "floating_point_type" => Ok(JavaNodeType::FloatingPointType),
            "decimal_floating_point_literal" => Ok(JavaNodeType::DecimalFloatingPointLiteral),
            "character_literal" => Ok(JavaNodeType::CharacterLiteral),
            "true" => Ok(JavaNodeType::True),
            "false" => Ok(JavaNodeType::False),
            "null_literal" => Ok(JavaNodeType::NullLiteral),
            "array_initializer" => Ok(JavaNodeType::ArrayInitializer),

            // Class
            "class_declaration" => Ok(JavaNodeType::ClassDecl),
            "class" => Ok(JavaNodeType::Class),
            "class_body" => Ok(JavaNodeType::ClassBody),
            "superclass" => Ok(JavaNodeType::Superclass),
            "extends" => Ok(JavaNodeType::Extends),
            "implements" => Ok(JavaNodeType::Implements),
            "constructor_declaration" => Ok(JavaNodeType::ConstructorDeclaration),
            "constructor_body" => Ok(JavaNodeType::ConstructorBody),
            "super" => Ok(JavaNodeType::Super),
            "explicit_constructor_invocation" => Ok(JavaNodeType::ExplicitConstructorInvocation),
            "class_literal" => Ok(JavaNodeType::ClassLiteral),
            "wildcard" => Ok(JavaNodeType::Wildcard),
            "abstract" => Ok(JavaNodeType::Abstract),

            // Enum
            "enum_declaration" => Ok(JavaNodeType::EnumDeclaration),
            "enum" => Ok(JavaNodeType::Enum),
            "enum_body" => Ok(JavaNodeType::EnumBody),
            "enum_constant" => Ok(JavaNodeType::EnumConstant),

            // Interface
            "interface_declaration" => Ok(JavaNodeType::InterfaceDeclaration),
            "super_interfaces" => Ok(JavaNodeType::SuperInterfaces),
            "interface_type_list" => Ok(JavaNodeType::InterfaceTypeList),
            "type_list" => Ok(JavaNodeType::TypeList),
            "interface" => Ok(JavaNodeType::Interface),
            "interface_body" => Ok(JavaNodeType::InterfaceBody),
            "@interface" => Ok(JavaNodeType::AtInterface),

            // Annotations
            "marker_annotation" => Ok(JavaNodeType::MarkerAnnotation),
            "@" => Ok(JavaNodeType::At),
            "annotation_type_declaration" => Ok(JavaNodeType::AnnotationTypeDeclaration),
            "annotation" => Ok(JavaNodeType::Annotation),
            "annotation_argument_list" => Ok(JavaNodeType::AnnotationArgumentList),
            "element_value_pair" => Ok(JavaNodeType::ElementValuePair),
            "annotation_type_body" => Ok(JavaNodeType::AnnotationTypeBody),

            // Method
            "method_declaration" => Ok(JavaNodeType::MethodDecl),
            "formal_parameters" => Ok(JavaNodeType::FormalParams),
            "formal_parameter" => Ok(JavaNodeType::FormalParam),
            "block" => Ok(JavaNodeType::Block),
            "local_variable_declaration" => Ok(JavaNodeType::LocalVarDecl),
            "variable_declarator" => Ok(JavaNodeType::VariableDeclarator),
            "method_reference" => Ok(JavaNodeType::MethodReference),
            "::" => Ok(JavaNodeType::MethodReferenceOperator),
            "spread_parameters" => Ok(JavaNodeType::SpreadParameters),
            "spread_parameter" => Ok(JavaNodeType::SpreadParameter),
            "..." => Ok(JavaNodeType::ThreeDots),
            "synchronized" => Ok(JavaNodeType::Synchronized),
            "type_parameters" => Ok(JavaNodeType::TypeParameters),
            "type_parameter" => Ok(JavaNodeType::TypeParameter),

            // Types
            "final" => Ok(JavaNodeType::Final),
            "void_type" => Ok(JavaNodeType::VoidType),
            "void" => Ok(JavaNodeType::Void),
            "generic_type" => Ok(JavaNodeType::GenericType),
            "type_arguments" => Ok(JavaNodeType::TypeArguments),
            "array_type" => Ok(JavaNodeType::ArrayType),
            "new" => Ok(JavaNodeType::New),
            "scoped_type_identifier" => Ok(JavaNodeType::ScopedTypeIdentifier),
            "type_identifier" => Ok(JavaNodeType::TypeIdentifier),
            "dimensions" => Ok(JavaNodeType::Dimensions),
            "integral_type" => Ok(JavaNodeType::IntegralType),
            "int" => Ok(JavaNodeType::Int),
            "float" => Ok(JavaNodeType::Float),
            "double" => Ok(JavaNodeType::Double),
            "long" => Ok(JavaNodeType::Long),
            "char" => Ok(JavaNodeType::Char),
            "boolean_type" => Ok(JavaNodeType::Boolean),
            "string" => Ok(JavaNodeType::String),
            "byte" => Ok(JavaNodeType::Byte),
            "short" => Ok(JavaNodeType::Short),

            // Visibility
            "public" => Ok(JavaNodeType::Public),
            "private" => Ok(JavaNodeType::Private),
            "protected" => Ok(JavaNodeType::Protected),
            "static" => Ok(JavaNodeType::Static),
            "volatile" => Ok(JavaNodeType::Volatile),
            "transient" => Ok(JavaNodeType::Transient),
            "this" => Ok(JavaNodeType::This),

            // Simple signs
            "." => Ok(JavaNodeType::Dot),
            "=" => Ok(JavaNodeType::Equals),
            ";" => Ok(JavaNodeType::Semicolon),
            "," => Ok(JavaNodeType::Comma),

            // Comments
            "comment" => Ok(JavaNodeType::Comment),
            "line_comment" => Ok(JavaNodeType::LineComment),
            "block_comment" => Ok(JavaNodeType::BlockComment),

            // Control flow
            "if_statement" => Ok(JavaNodeType::IfStatement),
            "if" => Ok(JavaNodeType::If),
            "else" => Ok(JavaNodeType::Else),
            "==" => Ok(JavaNodeType::Equality),
            "!=" => Ok(JavaNodeType::NoEquality),
            "||" => Ok(JavaNodeType::Or),
            "&&" => Ok(JavaNodeType::And),
            "!" => Ok(JavaNodeType::ExclamationMark),
            "<=" => Ok(JavaNodeType::LessOrEqual),
            ">=" => Ok(JavaNodeType::GreaterOrEqual),
            "|=" => Ok(JavaNodeType::OrComposition),
            "&=" => Ok(JavaNodeType::AndComposition),
            "?" => Ok(JavaNodeType::QuestionMark),
            ":" => Ok(JavaNodeType::Colon),
            "switch_expression" => Ok(JavaNodeType::SwitchExpression),
            "switch_block" => Ok(JavaNodeType::SwitchBlock),
            "switch" => Ok(JavaNodeType::Switch),
            "switch_statement" => Ok(JavaNodeType::SwitchStatement),
            "switch_block_statement_group" => Ok(JavaNodeType::SwitchBlockStatementGroup),
            "switch_label" => Ok(JavaNodeType::SwitchLabel),
            "case" => Ok(JavaNodeType::Case),
            "break_statement" => Ok(JavaNodeType::BreakStatement),
            "break" => Ok(JavaNodeType::Break),
            "default" => Ok(JavaNodeType::Default),
            "switch_rule" => Ok(JavaNodeType::SwitchRule),
            "for_statement" => Ok(JavaNodeType::ForStatement),
            "enhanced_for_statement" => Ok(JavaNodeType::EnhancedForStatement),
            "for" => Ok(JavaNodeType::For),
            "do_statement" => Ok(JavaNodeType::DoStatement),
            "do" => Ok(JavaNodeType::Do),
            "while_statement" => Ok(JavaNodeType::WhileStatement),
            "while" => Ok(JavaNodeType::While),
            "continue_statement" => Ok(JavaNodeType::ContinueStatement),
            "continue" => Ok(JavaNodeType::Continue),

            // Mathematical operators
            "+" => Ok(JavaNodeType::Plus),
            "-" => Ok(JavaNodeType::Minus),
            "%" => Ok(JavaNodeType::Modulus),
            "*" => Ok(JavaNodeType::Multiplication),
            "/" => Ok(JavaNodeType::Division),
            "update_expression" => Ok(JavaNodeType::UpdateExpression),
            "++" => Ok(JavaNodeType::PlusPlus),
            "--" => Ok(JavaNodeType::MinusMinus),
            "+=" => Ok(JavaNodeType::PlusComposition),
            "-=" => Ok(JavaNodeType::MinusComposition),
            "*=" => Ok(JavaNodeType::MultiplicationComposition),
            "/=" => Ok(JavaNodeType::DivisionComposition),
            "%=" => Ok(JavaNodeType::ModuleComposition),
            "^=" => Ok(JavaNodeType::ExponentComposition),
            "<<" => Ok(JavaNodeType::BitwiseShiftLeft),
            ">>" => Ok(JavaNodeType::BitwiseShiftRight),
            ">>>" => Ok(JavaNodeType::BitwiseShiftRightUnsigned),
            ">>>=" => Ok(JavaNodeType::BitwiseShiftRightUnsignedComposition),
            "<<=" => Ok(JavaNodeType::BitwiseShiftLeftComposition),
            ">>=" => Ok(JavaNodeType::BitwiseShiftRightComposition),
            "&" => Ok(JavaNodeType::Ampersand),
            "~" => Ok(JavaNodeType::Tilde),

            // Brackets
            "(" => Ok(JavaNodeType::LParentheses),
            ")" => Ok(JavaNodeType::RParentheses),
            "{" => Ok(JavaNodeType::LBrace),
            "}" => Ok(JavaNodeType::RBrace),
            "[" => Ok(JavaNodeType::LBracket),
            "]" => Ok(JavaNodeType::RBracket),
            "<" => Ok(JavaNodeType::LessThan),
            ">" => Ok(JavaNodeType::GreaterThan),

            _ => Err(()),
        }
    }
}

pub(crate) fn is_visibility(node_type_opt: &Option<JavaNodeType>) -> bool {
    if let Some(node_type) = node_type_opt {
        return &JavaNodeType::Private == node_type
            || &JavaNodeType::Public == node_type
            || &JavaNodeType::Protected == node_type;
    }
    false
}

impl fmt::Display for JavaNodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
