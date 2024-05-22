use crate::kind_src::KindsSrc;

pub const GRAPHQL_KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        ("!", "BANG"),
        ("$", "DOLLAR"),
        ("&", "AMP"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("...", "DOT3"),
        (":", "COLON"),
        ("=", "EQ"),
        ("@", "AT"),
        ("[", "L_BRACK"),
        ("]", "R_BRACK"),
        ("{", "L_CURLY"),
        ("|", "PIPE"),
        ("}", "R_CURLY"),
    ],
    keywords: &[
        "true",
        "false",
        "query",
        "mutation",
        "subscription",
        "fragment",
        "on",
        "null",
        "schema",
        "extend",
        "scalar",
        "type",
        "implements",
        "interface",
        "union",
        "enum",
        "input",
        "directive",
        "repeatable",
        "QUERY",
        "MUTATION",
        "SUBSCRIPTION",
        "FIELD",
        "FRAGMENT_DEFINITION",
        "FRAGMENT_SPREAD",
        "INLINE_FRAGMENT",
        "VARIABLE_DEFINITION",
        "SCHEMA",
        "SCALAR",
        "OBJECT",
        "FIELD_DEFINITION",
        "ARGUMENT_DEFINITION",
        "INTERFACE",
        "UNION",
        "ENUM",
        "ENUM_VALUE",
        "INPUT_OBJECT",
        "INPUT_FIELD_DEFINITION",
    ],
    literals: &[
        "GRAPHQL_STRING_LITERAL",
        "GRAPHQL_FLOAT_LITERAL",
        "GRAPHQL_INT_LITERAL",
    ],
    tokens: &[
        "ERROR_TOKEN",
        "NEWLINE",
        "WHITESPACE",
        "GRAPHQL_NAME",
        "COMMENT",
        "COMMA",
    ],
    nodes: &[
        "GRAPHQL_ROOT",
        "GRAPHQL_DEFINITION_LIST",
        "GRAPHQL_FRAGMENT_DEFINITION",
        "GRAPHQL_DIRECTIVE_DEFINITION",
        "GRAPHQL_SCHEMA_DEFINITION",
        "GRAPHQL_SCALAR_TYPE_DEFINITION",
        "GRAPHQL_OBJECT_TYPE_DEFINITION",
        "GRAPHQL_INTERFACE_TYPE_DEFINITION",
        "GRAPHQL_UNION_TYPE_DEFINITION",
        "GRAPHQL_ENUM_TYPE_DEFINITION",
        "GRAPHQL_INPUT_OBJECT_TYPE_DEFINITION",
        "GRAPHQL_SCALAR_TYPE_EXTENSION",
        "GRAPHQL_OPERATION_DEFINITION",
        "GRAPHQL_OPERATION_TYPE",
        "GRAPHQL_SELECTION_SET",
        "GRAPHQL_SELECTION_LIST",
        "GRAPHQL_FIELD",
        "GRAPHQL_ALIAS",
        "GRAPHQL_ARGUMENTS",
        "GRAPHQL_ARGUMENT_LIST",
        "GRAPHQL_ARGUMENT",
        "GRAPHQL_FRAGMENT_SPREAD",
        "GRAPHQL_INLINE_FRAGMENT",
        "GRAPHQL_FRAGMENT_NAME",
        "GRAPHQL_TYPE_CONDITION",
        "GRAPHQL_VARIABLE",
        "GRAPHQL_ENUM_VALUE",
        "GRAPHQL_LIST_VALUE",
        "GRAPHQL_LIST_VALUE_ELEMENT_LIST",
        "GRAPHQL_OBJECT_VALUE",
        "GRAPHQL_OBJECT_VALUE_MEMBER_LIST",
        "GRAPHQL_OBJECT_FIELD",
        "GRAPHQL_VARIABLE_DEFINITIONS",
        "GRAPHQL_VARIABLE_DEFINITION_LIST",
        "GRAPHQL_VARIABLE_DEFINITION",
        "GRAPHQL_DEFAULT_VALUE",
        "GRAPHQL_NON_NULL_TYPE",
        "GRAPHQL_NAMED_TYPE",
        "GRAPHQL_LIST_TYPE",
        "GRAPHQL_DIRECTIVE_LIST",
        "GRAPHQL_DIRECTIVE",
        "GRAPHQL_ROOT_OPERATION_TYPES",
        "GRAPHQL_ROOT_OPERATION_TYPE_DEFINITION_LIST",
        "GRAPHQL_ROOT_OPERATION_TYPE_DEFINITION",
        "GRAPHQL_SCHEMA_EXTENSION",
        "GRAPHQL_DESCRIPTION",
        "GRAPHQL_OBJECT_TYPE_EXTENSION",
        "GRAPHQL_IMPLEMENTS_INTERFACES",
        "GRAPHQL_IMPLEMENTS_INTERFACE_LIST",
        "GRAPHQL_FIELDS_DEFINITION",
        "GRAPHQL_FIELD_DEFINITION_LIST",
        "GRAPHQL_FIELD_DEFINITION",
        "GRAPHQL_ARGUMENTS_DEFINITION",
        "GRAPHQL_ARGUMENT_DEFINITION_LIST",
        "GRAPHQL_INPUT_VALUE_DEFINITION",
        "GRAPHQL_INTERFACE_TYPE_EXTENSION_WITH_FIELDS",
        "GRAPHQL_INTERFACE_TYPE_EXTENSION_WITH_DIRECTIVES",
        "GRAPHQL_INTERFACE_TYPE_EXTENSION",
        "GRAPHQL_UNION_MEMBER_TYPES",
        "GRAPHQL_UNION_MEMBER_TYPE_LIST",
        "GRAPHQL_UNION_TYPE_EXTENSION_WITH_MEMBERS",
        "GRAPHQL_UNION_TYPE_EXTENSION",
        "GRAPHQL_ENUM_VALUES_DEFINITION",
        "GRAPHQL_ENUM_VALUE_LIST",
        "GRAPHQL_ENUM_VALUE_DEFINITION",
        "GRAPHQL_ENUM_TYPE_EXTENSION_WITH_VALUES",
        "GRAPHQL_ENUM_TYPE_EXTENSION",
        "GRAPHQL_INPUT_FIELDS_DEFINITION",
        "GRAPHQL_INPUT_FIELD_LIST",
        "GRAPHQL_INPUT_OBJECT_TYPE_EXTENSION_WITH_FIELDS",
        "GRAPHQL_INPUT_OBJECT_TYPE_EXTENSION",
        "GRAPHQL_DIRECTIVE_LOCATION_LIST",
        "GRAPHQL_DIRECTIVE_LOCATION",
        // literal wrappers:
        "GRAPHQL_STRING_VALUE",
        "GRAPHQL_FLOAT_VALUE",
        "GRAPHQL_INT_VALUE",
        "GRAPHQL_BOOLEAN_VALUE",
        "GRAPHQL_NULL_VALUE",
        // Bogus nodes
        "GRAPHQL_BOGUS",
        "GRAPHQL_BOGUS_DEFINITION",
        "GRAPHQL_BOGUS_SELECTION",
        "GRAPHQL_BOGUS_VALUE",
        "GRAPHQL_BOGUS_TYPE",
        "GRAPHQL_BOGUS_EXTENSION",
    ],
};
