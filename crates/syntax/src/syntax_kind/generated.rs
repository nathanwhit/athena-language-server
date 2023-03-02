//! Generated by `sourcegen_ast`, do not edit by hand.

#![allow(bad_style, missing_docs, unreachable_pub)]
/// The kind of syntax node, e.g. `IDENT`, `USE_KW`, or `STRUCT`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc(hidden)]
    EOF,
    L_PAREN,
    R_PAREN,
    L_CURLY,
    R_CURLY,
    L_BRACK,
    R_BRACK,
    QUESTION,
    PIPE,
    UNDERSCORE,
    COLON,
    FAT_ARROW,
    BANG,
    AMP2,
    PIPE2,
    COLON_EQ,
    SINGLE_QUOTE,
    SEMI,
    THIN_ARROW,
    WHILE_KW,
    LET_KW,
    LETREC_KW,
    TRY_KW,
    CHECK_KW,
    LAMBDA_KW,
    METHOD_KW,
    MATCH_KW,
    CELL_KW,
    SETBANG_KW,
    REF_KW,
    MAKE_VECTOR_KW,
    VECTOR_SUB_KW,
    VECTOR_SETBANG_KW,
    SEQ_KW,
    APPLY_METHOD_KW,
    CONCLUDE_KW,
    ASSUME_KW,
    SUPPOSE_ABSURD_KW,
    GENERALIZE_OVER_KW,
    PICK_ANY_KW,
    WITH_WITNESS_KW,
    PICK_WITNESS_KW,
    PICK_WITNESSES_KW,
    BY_INDUCTION_KW,
    DATATYPE_CASES_KW,
    SOME_VAR_KW,
    SOME_SENT_CON_KW,
    SOME_QUANT_KW,
    SOME_TERM_KW,
    SOME_ATOM_KW,
    SOME_SENTENCE_KW,
    SOME_LIST_KW,
    SOME_CELL_KW,
    SOME_VECTOR_KW,
    SOME_PROC_KW,
    SOME_METHOD_KW,
    SOME_SYMBOL_KW,
    SOME_TABLE_KW,
    SOME_MAP_KW,
    SOME_SUB_KW,
    SOME_CHAR_KW,
    SPLIT_KW,
    WHERE_KW,
    LIST_OF_KW,
    VAL_OF_KW,
    AS_KW,
    BIND_KW,
    FOR_KW,
    DEFINE_KW,
    MODULE_KW,
    DECLARE_KW,
    DOMAIN_KW,
    DOMAINS_KW,
    INT_NUMBER,
    STRING,
    ERROR,
    IDENT,
    WHITESPACE,
    COMMENT,
    SOURCE_FILE,
    IDENTIFIER,
    LITERAL,
    META_IDENT,
    UNIT,
    IDENT_SORT,
    VAR_SORT,
    COMPOUND_SORT,
    EXPR_PHRASE,
    DED_PHRASE,
    IDENT_EXPR,
    TERM_VAR_EXPR,
    CHECK_EXPR,
    LAMBDA_EXPR,
    APPLICATION_EXPR,
    LIST_EXPR,
    METHOD_EXPR,
    LET_EXPR,
    LET_REC_EXPR,
    MATCH_EXPR,
    TRY_EXPR,
    CELL_EXPR,
    SET_EXPR,
    REF_EXPR,
    WHILE_EXPR,
    MAKE_VECTOR_EXPR,
    VECTOR_SUB_EXPR,
    VECTOR_SET_EXPR,
    SEQ_EXPR,
    AND_EXPR,
    OR_EXPR,
    METHOD_CALL_DED,
    BANG_METHOD_CALL_DED,
    ASSUME_DED,
    NAMED_ASSUME_DED,
    PROOF_BY_CONTRA_DED,
    UNIVERSAL_GENERAL_DED,
    EXISTENTIAL_INSTANT_DED,
    INDUCT_DED,
    CASES_DED,
    CHECK_DED,
    MATCH_DED,
    LET_DED,
    LET_REC_DED,
    TRY_DED,
    TRY_DED_PART,
    MATCH_DED_ARM,
    CHECK_DED_ARM,
    RESTRICTED_MATCH_DED,
    RESTRICTED_NAMED_PAT,
    RESTRICTED_APPLY_PAT,
    ASSUME_PART,
    IDENT_PAT,
    ANNOTATED_IDENT_PAT,
    VAR_PAT,
    META_IDENT_PAT,
    LITERAL_PAT,
    UNIT_PAT,
    WILDCARD_PAT,
    NAMED_PAT,
    VAL_OF_PAT,
    LIST_OF_PAT,
    SPLIT_PAT,
    LIST_PAT,
    COMPOUND_PAT,
    WHERE_PAT,
    SOME_THING_PAT,
    SOME_THING,
    MATCH_ARM,
    TRY_ARM,
    LET_PART,
    CHECK_ARM,
    LET_REC_PART,
    FUNC_SORTS,
    SORT_VARS_DECL,
    COMPOUND_SORT_DECL,
    DEFINE_DIR,
    DEFINE_MULTI_DIR,
    DEFINE_PROC_DIR,
    DOMAIN_DIR,
    DOMAINS_DIR,
    DECLARE_DIR,
    MODULE_DIR,
    DIR_STMT,
    PHRASE_STMT,
    #[doc(hidden)]
    __LAST,
}
use self::SyntaxKind::*;
impl SyntaxKind {
    pub fn is_keyword(self) -> bool {
        matches!(
            self, WHILE_KW | LET_KW | LETREC_KW | TRY_KW | CHECK_KW | LAMBDA_KW |
            METHOD_KW | MATCH_KW | CELL_KW | SETBANG_KW | REF_KW | MAKE_VECTOR_KW |
            VECTOR_SUB_KW | VECTOR_SETBANG_KW | SEQ_KW | APPLY_METHOD_KW | CONCLUDE_KW |
            ASSUME_KW | SUPPOSE_ABSURD_KW | GENERALIZE_OVER_KW | PICK_ANY_KW |
            WITH_WITNESS_KW | PICK_WITNESS_KW | PICK_WITNESSES_KW | BY_INDUCTION_KW |
            DATATYPE_CASES_KW | SOME_VAR_KW | SOME_SENT_CON_KW | SOME_QUANT_KW |
            SOME_TERM_KW | SOME_ATOM_KW | SOME_SENTENCE_KW | SOME_LIST_KW | SOME_CELL_KW
            | SOME_VECTOR_KW | SOME_PROC_KW | SOME_METHOD_KW | SOME_SYMBOL_KW |
            SOME_TABLE_KW | SOME_MAP_KW | SOME_SUB_KW | SOME_CHAR_KW | SPLIT_KW |
            WHERE_KW | LIST_OF_KW | VAL_OF_KW | AS_KW | BIND_KW | FOR_KW | DEFINE_KW |
            MODULE_KW | DECLARE_KW | DOMAIN_KW | DOMAINS_KW
        )
    }
    pub fn is_punct(self) -> bool {
        matches!(
            self, L_PAREN | R_PAREN | L_CURLY | R_CURLY | L_BRACK | R_BRACK | QUESTION |
            PIPE | UNDERSCORE | COLON | FAT_ARROW | BANG | AMP2 | PIPE2 | COLON_EQ |
            SINGLE_QUOTE | SEMI | THIN_ARROW
        )
    }
    pub fn is_literal(self) -> bool {
        matches!(self, INT_NUMBER | STRING)
    }
    pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
        let kw = match ident {
            "while" => WHILE_KW,
            "let" => LET_KW,
            "letrec" => LETREC_KW,
            "try" => TRY_KW,
            "check" => CHECK_KW,
            "lambda" => LAMBDA_KW,
            "method" => METHOD_KW,
            "match" => MATCH_KW,
            "cell" => CELL_KW,
            "set!" => SETBANG_KW,
            "ref" => REF_KW,
            "make-vector" => MAKE_VECTOR_KW,
            "vector-sub" => VECTOR_SUB_KW,
            "vector-set!" => VECTOR_SETBANG_KW,
            "seq" => SEQ_KW,
            "apply-method" => APPLY_METHOD_KW,
            "conclude" => CONCLUDE_KW,
            "assume" => ASSUME_KW,
            "suppose-absurd" => SUPPOSE_ABSURD_KW,
            "generalize-over" => GENERALIZE_OVER_KW,
            "pick-any" => PICK_ANY_KW,
            "with-witness" => WITH_WITNESS_KW,
            "pick-witness" => PICK_WITNESS_KW,
            "pick-witnesses" => PICK_WITNESSES_KW,
            "by-induction" => BY_INDUCTION_KW,
            "datatype-cases" => DATATYPE_CASES_KW,
            "some-var" => SOME_VAR_KW,
            "some-sent-con" => SOME_SENT_CON_KW,
            "some-quant" => SOME_QUANT_KW,
            "some-term" => SOME_TERM_KW,
            "some-atom" => SOME_ATOM_KW,
            "some-sentence" => SOME_SENTENCE_KW,
            "some-list" => SOME_LIST_KW,
            "some-cell" => SOME_CELL_KW,
            "some-vector" => SOME_VECTOR_KW,
            "some-proc" => SOME_PROC_KW,
            "some-method" => SOME_METHOD_KW,
            "some-symbol" => SOME_SYMBOL_KW,
            "some-table" => SOME_TABLE_KW,
            "some-map" => SOME_MAP_KW,
            "some-sub" => SOME_SUB_KW,
            "some-char" => SOME_CHAR_KW,
            "split" => SPLIT_KW,
            "where" => WHERE_KW,
            "list-of" => LIST_OF_KW,
            "val-of" => VAL_OF_KW,
            "as" => AS_KW,
            "bind" => BIND_KW,
            "for" => FOR_KW,
            "define" => DEFINE_KW,
            "module" => MODULE_KW,
            "declare" => DECLARE_KW,
            "domain" => DOMAIN_KW,
            "domains" => DOMAINS_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub fn from_contextual_keyword(ident: &str) -> Option<SyntaxKind> {
        #[allow(unused_variables, unreachable_code)]
        {
            let kw = match ident {
                _ => return None,
            };
            Some(kw)
        }
    }
    pub fn from_char(c: char) -> Option<SyntaxKind> {
        let tok = match c {
            '(' => L_PAREN,
            ')' => R_PAREN,
            '{' => L_CURLY,
            '}' => R_CURLY,
            '[' => L_BRACK,
            ']' => R_BRACK,
            '?' => QUESTION,
            '|' => PIPE,
            '_' => UNDERSCORE,
            ':' => COLON,
            '!' => BANG,
            '\'' => SINGLE_QUOTE,
            ';' => SEMI,
            _ => return None,
        };
        Some(tok)
    }
}
#[macro_export]
macro_rules! T {
    ['('] => {
        $crate ::SyntaxKind::L_PAREN
    };
    [')'] => {
        $crate ::SyntaxKind::R_PAREN
    };
    ['{'] => {
        $crate ::SyntaxKind::L_CURLY
    };
    ['}'] => {
        $crate ::SyntaxKind::R_CURLY
    };
    ['['] => {
        $crate ::SyntaxKind::L_BRACK
    };
    [']'] => {
        $crate ::SyntaxKind::R_BRACK
    };
    [?] => {
        $crate ::SyntaxKind::QUESTION
    };
    [|] => {
        $crate ::SyntaxKind::PIPE
    };
    [_] => {
        $crate ::SyntaxKind::UNDERSCORE
    };
    [:] => {
        $crate ::SyntaxKind::COLON
    };
    [=>] => {
        $crate ::SyntaxKind::FAT_ARROW
    };
    [!] => {
        $crate ::SyntaxKind::BANG
    };
    [&&] => {
        $crate ::SyntaxKind::AMP2
    };
    [||] => {
        $crate ::SyntaxKind::PIPE2
    };
    [:=] => {
        $crate ::SyntaxKind::COLON_EQ
    };
    ['\''] => {
        $crate ::SyntaxKind::SINGLE_QUOTE
    };
    [;] => {
        $crate ::SyntaxKind::SEMI
    };
    [->] => {
        $crate ::SyntaxKind::THIN_ARROW
    };
    [while] => {
        $crate ::SyntaxKind::WHILE_KW
    };
    [let] => {
        $crate ::SyntaxKind::LET_KW
    };
    [letrec] => {
        $crate ::SyntaxKind::LETREC_KW
    };
    [try] => {
        $crate ::SyntaxKind::TRY_KW
    };
    [check] => {
        $crate ::SyntaxKind::CHECK_KW
    };
    [lambda] => {
        $crate ::SyntaxKind::LAMBDA_KW
    };
    [method] => {
        $crate ::SyntaxKind::METHOD_KW
    };
    [match] => {
        $crate ::SyntaxKind::MATCH_KW
    };
    [cell] => {
        $crate ::SyntaxKind::CELL_KW
    };
    [setbang] => {
        $crate ::SyntaxKind::SETBANG_KW
    };
    [ref] => {
        $crate ::SyntaxKind::REF_KW
    };
    [make_vector] => {
        $crate ::SyntaxKind::MAKE_VECTOR_KW
    };
    [vector_sub] => {
        $crate ::SyntaxKind::VECTOR_SUB_KW
    };
    [vector_setbang] => {
        $crate ::SyntaxKind::VECTOR_SETBANG_KW
    };
    [seq] => {
        $crate ::SyntaxKind::SEQ_KW
    };
    [apply_method] => {
        $crate ::SyntaxKind::APPLY_METHOD_KW
    };
    [conclude] => {
        $crate ::SyntaxKind::CONCLUDE_KW
    };
    [assume] => {
        $crate ::SyntaxKind::ASSUME_KW
    };
    [suppose_absurd] => {
        $crate ::SyntaxKind::SUPPOSE_ABSURD_KW
    };
    [generalize_over] => {
        $crate ::SyntaxKind::GENERALIZE_OVER_KW
    };
    [pick_any] => {
        $crate ::SyntaxKind::PICK_ANY_KW
    };
    [with_witness] => {
        $crate ::SyntaxKind::WITH_WITNESS_KW
    };
    [pick_witness] => {
        $crate ::SyntaxKind::PICK_WITNESS_KW
    };
    [pick_witnesses] => {
        $crate ::SyntaxKind::PICK_WITNESSES_KW
    };
    [by_induction] => {
        $crate ::SyntaxKind::BY_INDUCTION_KW
    };
    [datatype_cases] => {
        $crate ::SyntaxKind::DATATYPE_CASES_KW
    };
    [some_var] => {
        $crate ::SyntaxKind::SOME_VAR_KW
    };
    [some_sent_con] => {
        $crate ::SyntaxKind::SOME_SENT_CON_KW
    };
    [some_quant] => {
        $crate ::SyntaxKind::SOME_QUANT_KW
    };
    [some_term] => {
        $crate ::SyntaxKind::SOME_TERM_KW
    };
    [some_atom] => {
        $crate ::SyntaxKind::SOME_ATOM_KW
    };
    [some_sentence] => {
        $crate ::SyntaxKind::SOME_SENTENCE_KW
    };
    [some_list] => {
        $crate ::SyntaxKind::SOME_LIST_KW
    };
    [some_cell] => {
        $crate ::SyntaxKind::SOME_CELL_KW
    };
    [some_vector] => {
        $crate ::SyntaxKind::SOME_VECTOR_KW
    };
    [some_proc] => {
        $crate ::SyntaxKind::SOME_PROC_KW
    };
    [some_method] => {
        $crate ::SyntaxKind::SOME_METHOD_KW
    };
    [some_symbol] => {
        $crate ::SyntaxKind::SOME_SYMBOL_KW
    };
    [some_table] => {
        $crate ::SyntaxKind::SOME_TABLE_KW
    };
    [some_map] => {
        $crate ::SyntaxKind::SOME_MAP_KW
    };
    [some_sub] => {
        $crate ::SyntaxKind::SOME_SUB_KW
    };
    [some_char] => {
        $crate ::SyntaxKind::SOME_CHAR_KW
    };
    [split] => {
        $crate ::SyntaxKind::SPLIT_KW
    };
    [where] => {
        $crate ::SyntaxKind::WHERE_KW
    };
    [list_of] => {
        $crate ::SyntaxKind::LIST_OF_KW
    };
    [val_of] => {
        $crate ::SyntaxKind::VAL_OF_KW
    };
    [as] => {
        $crate ::SyntaxKind::AS_KW
    };
    [bind] => {
        $crate ::SyntaxKind::BIND_KW
    };
    [for] => {
        $crate ::SyntaxKind::FOR_KW
    };
    [define] => {
        $crate ::SyntaxKind::DEFINE_KW
    };
    [module] => {
        $crate ::SyntaxKind::MODULE_KW
    };
    [declare] => {
        $crate ::SyntaxKind::DECLARE_KW
    };
    [domain] => {
        $crate ::SyntaxKind::DOMAIN_KW
    };
    [domains] => {
        $crate ::SyntaxKind::DOMAINS_KW
    };
    [lifetime_ident] => {
        $crate ::SyntaxKind::LIFETIME_IDENT
    };
    [ident] => {
        $crate ::SyntaxKind::IDENT
    };
    [shebang] => {
        $crate ::SyntaxKind::SHEBANG
    };
}
pub use T;
