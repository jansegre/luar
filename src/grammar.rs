#![allow(dead_code)]
#![allow(unused_variables)]
/* TMPL: %include */

    use ::ast::*;
/* TMPL: makeheader cruft */


/* TMPL: types */

type YYCODETYPE = i8;
const YYNOCODE: i32 = 88;
type YYACTIONTYPE = u16;
const YYWILDCARD: YYCODETYPE = 0;
enum YYMinorType {
    YY0,
    YY18(Vec<(Expr, Block)>),
    YY36(Block),
    YY40(String),
    YY68(V<Name>),
    YY69(Args),
    YY71(f64),
    YY78(Expr),
    YY79(Vec<Name>),
    YY92(Vec<Var>),
    YY95(Option<P<Block>>),
    YY98(Var),
    YY101(Stmt),
    YY102(Vec<Field>),
    YY104(V<Expr>),
    YY114((V<Name>, bool, P<Block>)),
    YY115(Vec<Stmt>),
    YY116(Call),
    YY117(Vec<Expr>),
    YY120(Name),
    YY131((V<Name>, Option<Name>)),
    YY142(V<Var>),
    YY147(V<Field>),
    YY162(Option<Stmt>),
    YY173((V<Name>, bool)),
    YY174(Field),
}
const YYNSTATE: i32 = 197;
const YYNRULE: i32 = 103;
const YYERRORSYMBOL: i32 = 55;

//const YY_NO_ACTION: i32 = YYNSTATE+YYNRULE+2;
//const YY_ACCEPT_ACTION: i32 = YYNSTATE+YYNRULE+1;
//const YY_ERROR_ACTION: i32 = YYNSTATE+YYNRULE+1;

/* TMPL: action tables */

#[derive( Clone 
)]
pub enum Token {
    EOI, //0
    OPENPAREN2, //1
    OPENPAREN, //2
    COMMA2, //3
    COMMA, //4
    NAME( String ), //5
    NUMBER( f64 ), //6
    STRING( String ), //7
    SEMI, //8
    EQ, //9
    BREAK, //10
    GOTO, //11
    DO, //12
    END, //13
    WHILE, //14
    REPEAT, //15
    UNTIL, //16
    IF, //17
    THEN, //18
    FOR, //19
    IN, //20
    FUNCTION, //21
    LOCAL, //22
    ELSEIF, //23
    ELSE, //24
    RETURN, //25
    COLONCOLON, //26
    COLON, //27
    DOT, //28
    OPENBRACKET, //29
    CLOSEBRACKET, //30
    OR, //31
    AND, //32
    LT, //33
    GT, //34
    LE, //35
    GE, //36
    NE, //37
    EQEQ, //38
    DOTDOT, //39
    PLUS, //40
    MINUS, //41
    STAR, //42
    SLASH, //43
    PERCENT, //44
    NOT, //45
    POUND, //46
    CARET, //47
    NIL, //48
    FALSE, //49
    TRUE, //50
    DOTDOTDOT, //51
    CLOSEPAREN, //52
    OPENBRACE, //53
    CLOSEBRACE, //54
}
#[inline]
fn token_major(t: &Token) -> i32 {
    match *t {
        Token::EOI => 0,
        Token::OPENPAREN2(..) => 1,
        Token::OPENPAREN(..) => 2,
        Token::COMMA2(..) => 3,
        Token::COMMA(..) => 4,
        Token::NAME(..) => 5,
        Token::NUMBER(..) => 6,
        Token::STRING(..) => 7,
        Token::SEMI(..) => 8,
        Token::EQ(..) => 9,
        Token::BREAK(..) => 10,
        Token::GOTO(..) => 11,
        Token::DO(..) => 12,
        Token::END(..) => 13,
        Token::WHILE(..) => 14,
        Token::REPEAT(..) => 15,
        Token::UNTIL(..) => 16,
        Token::IF(..) => 17,
        Token::THEN(..) => 18,
        Token::FOR(..) => 19,
        Token::IN(..) => 20,
        Token::FUNCTION(..) => 21,
        Token::LOCAL(..) => 22,
        Token::ELSEIF(..) => 23,
        Token::ELSE(..) => 24,
        Token::RETURN(..) => 25,
        Token::COLONCOLON(..) => 26,
        Token::COLON(..) => 27,
        Token::DOT(..) => 28,
        Token::OPENBRACKET(..) => 29,
        Token::CLOSEBRACKET(..) => 30,
        Token::OR(..) => 31,
        Token::AND(..) => 32,
        Token::LT(..) => 33,
        Token::GT(..) => 34,
        Token::LE(..) => 35,
        Token::GE(..) => 36,
        Token::NE(..) => 37,
        Token::EQEQ(..) => 38,
        Token::DOTDOT(..) => 39,
        Token::PLUS(..) => 40,
        Token::MINUS(..) => 41,
        Token::STAR(..) => 42,
        Token::SLASH(..) => 43,
        Token::PERCENT(..) => 44,
        Token::NOT(..) => 45,
        Token::POUND(..) => 46,
        Token::CARET(..) => 47,
        Token::NIL(..) => 48,
        Token::FALSE(..) => 49,
        Token::TRUE(..) => 50,
        Token::DOTDOTDOT(..) => 51,
        Token::CLOSEPAREN(..) => 52,
        Token::OPENBRACE(..) => 53,
        Token::CLOSEBRACE(..) => 54,
    }
}
#[inline]
fn token_minor(t: Token) -> YYMinorType {
  match t {
        Token::NAME(x) => YYMinorType::YY40(x),
        Token::NUMBER(x) => YYMinorType::YY71(x),
        Token::STRING(x) => YYMinorType::YY40(x),
        _ => YYMinorType::YY0
  }
}
const YY_ACTTAB_COUNT: i32 = 1027;
const YY_ACTION: [YYACTIONTYPE; 1027] = [
 /*     0 */     9,   24,   23,   22,   65,   64,   21,  197,   50,  190,
 /*    10 */    38,   36,   34,   31,   32,   30,   28,   29,   27,   26,
 /*    20 */    25,   24,   23,   22,   12,   51,   21,   38,   36,   34,
 /*    30 */    31,   32,   30,   28,   29,   27,   26,   25,   24,   23,
 /*    40 */    22,  126,    2,   21,   38,   36,   34,   31,   32,   30,
 /*    50 */    28,   29,   27,   26,   25,   24,   23,   22,  236,  235,
 /*    60 */    21,  124,    2,  236,  235,  178,  119,   38,   36,   34,
 /*    70 */    31,   32,   30,   28,   29,   27,   26,   25,   24,   23,
 /*    80 */    22,   55,   21,   21,   36,   34,   31,   32,   30,   28,
 /*    90 */    29,   27,   26,   25,   24,   23,   22,  116,    2,   21,
 /*   100 */    38,   36,   34,   31,   32,   30,   28,   29,   27,   26,
 /*   110 */    25,   24,   23,   22,  166,  179,   21,  191,  166,   53,
 /*   120 */    27,   26,   25,   24,   23,   22,  101,   45,   21,  301,
 /*   130 */   136,    2,   38,   36,   34,   31,   32,   30,   28,   29,
 /*   140 */    27,   26,   25,   24,   23,   22,  174,    4,   21,    7,
 /*   150 */   123,   52,  158,  143,  115,    2,   10,  127,   66,   20,
 /*   160 */   102,  173,  125,  175,   38,   36,   34,   31,   32,   30,
 /*   170 */    28,   29,   27,   26,   25,   24,   23,   22,  114,    2,
 /*   180 */    21,   49,   70,   38,   36,   34,   31,   32,   30,   28,
 /*   190 */    29,   27,   26,   25,   24,   23,   22,  122,    1,   21,
 /*   200 */    38,   36,   34,   31,   32,   30,   28,   29,   27,   26,
 /*   210 */    25,   24,   23,   22,   72,    2,   21,   38,   36,   34,
 /*   220 */    31,   32,   30,   28,   29,   27,   26,   25,   24,   23,
 /*   230 */    22,   20,   57,   21,  172,  175,  185,  184,   34,   31,
 /*   240 */    32,   30,   28,   29,   27,   26,   25,   24,   23,   22,
 /*   250 */   112,   71,   21,  174,  109,  159,  162,  106,  175,   18,
 /*   260 */   175,  127,  159,  163,  127,  151,    2,  150,    2,  111,
 /*   270 */     2,   33,  110,    2,   63,   37,   35,  171,  189,  188,
 /*   280 */   187,  186,   20,    1,  169,   19,  175,  185,  184,  107,
 /*   290 */     2,  275,  168,   17,  140,   16,  275,  160,  196,  195,
 /*   300 */   193,  135,   71,   97,  157,  191,  170,   58,  154,  153,
 /*   310 */    18,  152,   11,  103,  100,   45,  275,  275,  275,    4,
 /*   320 */    14,  149,   33,  148,  158,    6,   37,   35,   48,  189,
 /*   330 */   188,  187,  186,   20,    1,  147,    5,  175,  185,  184,
 /*   340 */   137,  138,  275,   41,   68,   69,   39,   59,   40,  177,
 /*   350 */   175,  176,  164,   71,   47,   20,  155,    8,  302,  175,
 /*   360 */   185,  184,  139,  145,  146,  144,   62,  104,  302,  302,
 /*   370 */     1,  302,  302,   33,  142,   71,  113,   37,   35,  302,
 /*   380 */   189,  188,  187,  186,  161,    1,  302,  302,  302,  302,
 /*   390 */   302,  302,  302,  302,  302,   33,  302,  302,  302,   37,
 /*   400 */    35,  302,  189,  188,  187,  186,   20,    1,  302,  302,
 /*   410 */   175,  302,  302,  194,  302,  156,   67,   56,  302,   15,
 /*   420 */    54,  302,   13,  302,   44,  302,   43,   42,   20,  181,
 /*   430 */     3,   61,  175,  185,  184,  302,  302,  302,  179,  302,
 /*   440 */   121,   86,  302,  302,  302,  302,  302,  302,   71,  180,
 /*   450 */    46,  302,  302,  183,  182,  302,  302,  120,   60,  302,
 /*   460 */   165,  302,  302,  302,  302,  302,  302,  302,   33,  302,
 /*   470 */   302,  302,   37,   35,  302,  189,  188,  187,  186,  302,
 /*   480 */     1,  302,  302,  181,  302,  302,  302,  302,  302,  181,
 /*   490 */   302,  192,  179,  302,  191,   83,  302,  108,  179,  302,
 /*   500 */   191,   83,  302,  180,   46,  302,  118,  183,  182,  180,
 /*   510 */    46,  302,  118,  183,  182,  302,  181,  302,  302,  302,
 /*   520 */   302,  302,  302,  302,  141,  179,  302,  191,   83,  302,
 /*   530 */   302,  302,  302,  302,  181,  302,  180,   46,  302,  118,
 /*   540 */   183,  182,  117,  179,  302,  191,   83,  302,  302,  302,
 /*   550 */   302,  302,  302,  181,  180,   46,  302,  118,  183,  182,
 /*   560 */   181,  105,  179,  302,  191,   83,  302,  302,  302,  179,
 /*   570 */   302,  121,   86,  180,   46,  302,  118,  183,  182,  302,
 /*   580 */   180,   46,  302,  181,  183,  182,  302,  302,  302,  302,
 /*   590 */   181,  167,  179,  302,  191,   81,  302,  302,  302,  179,
 /*   600 */   302,  191,   88,  180,   46,  302,  302,  183,  182,  181,
 /*   610 */   180,   46,  302,  302,  183,  182,  181,  302,  179,  302,
 /*   620 */   191,  134,  302,  302,  302,  179,  302,  191,   89,  180,
 /*   630 */    46,  302,  302,  183,  182,  302,  180,   46,  302,  181,
 /*   640 */   183,  182,  302,  302,  302,  302,  181,  302,  179,  302,
 /*   650 */   191,  133,  302,  302,  302,  179,  302,  191,   90,  180,
 /*   660 */    46,  302,  302,  183,  182,  181,  180,   46,  302,  302,
 /*   670 */   183,  182,  181,  302,  179,  302,  191,  132,  302,  302,
 /*   680 */   302,  179,  302,  191,   96,  180,   46,  302,  302,  183,
 /*   690 */   182,  302,  180,   46,  302,  181,  183,  182,  302,  302,
 /*   700 */   302,  302,  181,  302,  179,  302,  191,   95,  302,  302,
 /*   710 */   302,  179,  302,  191,   94,  180,   46,  302,  302,  183,
 /*   720 */   182,  181,  180,   46,  302,  302,  183,  182,  181,  302,
 /*   730 */   179,  302,  191,   93,  302,  302,  302,  179,  302,  191,
 /*   740 */    92,  180,   46,  302,  302,  183,  182,  302,  180,   46,
 /*   750 */   302,  181,  183,  182,  302,  302,  302,  302,  181,  302,
 /*   760 */   179,  302,  191,   91,  302,  302,  302,  179,  302,  191,
 /*   770 */    99,  180,   46,  302,  302,  183,  182,  181,  180,   46,
 /*   780 */   302,  302,  183,  182,  181,  302,  179,  302,  191,   98,
 /*   790 */   302,  302,  302,  179,  302,  191,  131,  180,   46,  302,
 /*   800 */   302,  183,  182,  302,  180,   46,  302,  181,  183,  182,
 /*   810 */   302,  302,  302,  302,  181,  302,  179,  302,  191,  130,
 /*   820 */   302,  302,  302,  179,  302,  191,  129,  180,   46,  302,
 /*   830 */   302,  183,  182,  181,  180,   46,  302,  302,  183,  182,
 /*   840 */   181,  302,  179,  302,  191,  128,  302,  302,  302,  179,
 /*   850 */   302,  191,   80,  180,   46,  302,  302,  183,  182,  302,
 /*   860 */   180,   46,  302,  181,  183,  182,  302,  302,  302,  302,
 /*   870 */   181,  302,  179,  302,  191,   87,  302,  302,  302,  179,
 /*   880 */   302,  191,   79,  180,   46,  302,  302,  183,  182,  181,
 /*   890 */   180,   46,  302,  302,  183,  182,  181,  302,  179,  302,
 /*   900 */   191,   85,  302,  302,  302,  179,  302,  191,   84,  180,
 /*   910 */    46,  302,  302,  183,  182,  302,  180,   46,  302,  181,
 /*   920 */   183,  182,  302,  302,  302,  302,  181,  302,  179,  302,
 /*   930 */   191,   78,  302,  302,  302,  179,  302,  191,   82,  180,
 /*   940 */    46,  302,  302,  183,  182,  181,  180,   46,  302,  302,
 /*   950 */   183,  182,  181,  302,  179,  302,  191,   77,  302,  302,
 /*   960 */   302,  179,  302,  191,   76,  180,   46,  302,  302,  183,
 /*   970 */   182,  302,  180,   46,  302,  181,  183,  182,  302,  302,
 /*   980 */   302,  302,  181,  302,  179,  302,  191,   75,  302,  302,
 /*   990 */   302,  179,  302,  191,   73,  180,   46,  302,  302,  183,
 /*  1000 */   182,  181,  180,   46,  302,  302,  183,  182,  302,  302,
 /*  1010 */   179,  302,  191,   74,  302,  302,  302,  302,  302,  302,
 /*  1020 */   302,  180,   46,  302,  302,  183,  182,
];
const YY_LOOKAHEAD: [YYCODETYPE; 1027] = [
 /*     0 */     4,   42,   43,   44,   27,   28,   47,    0,   12,   30,
 /*    10 */    31,   32,   33,   34,   35,   36,   37,   38,   39,   40,
 /*    20 */    41,   42,   43,   44,   23,   24,   47,   31,   32,   33,
 /*    30 */    34,   35,   36,   37,   38,   39,   40,   41,   42,   43,
 /*    40 */    44,   57,   58,   47,   31,   32,   33,   34,   35,   36,
 /*    50 */    37,   38,   39,   40,   41,   42,   43,   44,    4,    4,
 /*    60 */    47,   57,   58,    9,    9,   52,   30,   31,   32,   33,
 /*    70 */    34,   35,   36,   37,   38,   39,   40,   41,   42,   43,
 /*    80 */    44,   12,   47,   47,   32,   33,   34,   35,   36,   37,
 /*    90 */    38,   39,   40,   41,   42,   43,   44,   57,   58,   47,
 /*   100 */    31,   32,   33,   34,   35,   36,   37,   38,   39,   40,
 /*   110 */    41,   42,   43,   44,    4,   64,   47,   66,    8,   18,
 /*   120 */    39,   40,   41,   42,   43,   44,   75,   76,   47,   56,
 /*   130 */    57,   58,   31,   32,   33,   34,   35,   36,   37,   38,
 /*   140 */    39,   40,   41,   42,   43,   44,   66,    2,   47,    9,
 /*   150 */    70,   18,    7,   66,   57,   58,    4,   77,   71,    1,
 /*   160 */    73,   13,   82,    5,   31,   32,   33,   34,   35,   36,
 /*   170 */    37,   38,   39,   40,   41,   42,   43,   44,   57,   58,
 /*   180 */    47,   12,    4,   31,   32,   33,   34,   35,   36,   37,
 /*   190 */    38,   39,   40,   41,   42,   43,   44,    3,   53,   47,
 /*   200 */    31,   32,   33,   34,   35,   36,   37,   38,   39,   40,
 /*   210 */    41,   42,   43,   44,   57,   58,   47,   31,   32,   33,
 /*   220 */    34,   35,   36,   37,   38,   39,   40,   41,   42,   43,
 /*   230 */    44,    1,   52,   47,   13,    5,    6,    7,   33,   34,
 /*   240 */    35,   36,   37,   38,   39,   40,   41,   42,   43,   44,
 /*   250 */    66,   21,   47,   66,   70,   80,   81,   70,    5,   29,
 /*   260 */     5,   77,   80,   81,   77,   57,   58,   57,   58,   57,
 /*   270 */    58,   41,   57,   58,   21,   45,   46,   51,   48,   49,
 /*   280 */    50,   51,    1,   53,   54,    9,    5,    6,    7,   57,
 /*   290 */    58,    2,   54,    9,   55,    4,    7,   52,   59,   60,
 /*   300 */    61,   62,   21,   64,   65,   66,   51,   52,   13,   13,
 /*   310 */    29,   13,    9,   74,   75,   76,   27,   28,   29,    2,
 /*   320 */    16,   13,   41,   13,    7,   20,   45,   46,   12,   48,
 /*   330 */    49,   50,   51,    1,   53,   13,    9,    5,    6,    7,
 /*   340 */    26,    8,   53,    4,   27,   28,   29,   68,    2,   72,
 /*   350 */     5,   66,   66,   21,   66,    1,   66,   85,   87,    5,
 /*   360 */     6,    7,    8,   66,   72,   66,   66,   66,   87,   87,
 /*   370 */    53,   87,   87,   41,   72,   21,   69,   45,   46,   87,
 /*   380 */    48,   49,   50,   51,   52,   53,   87,   87,   87,   87,
 /*   390 */    87,   87,   87,   87,   87,   41,   87,   87,   87,   45,
 /*   400 */    46,   87,   48,   49,   50,   51,    1,   53,   87,   87,
 /*   410 */     5,   87,   87,    8,   87,   10,   11,   12,   87,   14,
 /*   420 */    15,   87,   17,   87,   19,   87,   21,   22,    1,   55,
 /*   430 */    25,   26,    5,    6,    7,   87,   87,   87,   64,   87,
 /*   440 */    66,   67,   87,   87,   87,   87,   87,   87,   21,   75,
 /*   450 */    76,   87,   87,   79,   80,   87,   87,   83,   84,   87,
 /*   460 */    86,   87,   87,   87,   87,   87,   87,   87,   41,   87,
 /*   470 */    87,   87,   45,   46,   87,   48,   49,   50,   51,   87,
 /*   480 */    53,   87,   87,   55,   87,   87,   87,   87,   87,   55,
 /*   490 */    87,   63,   64,   87,   66,   67,   87,   63,   64,   87,
 /*   500 */    66,   67,   87,   75,   76,   87,   78,   79,   80,   75,
 /*   510 */    76,   87,   78,   79,   80,   87,   55,   87,   87,   87,
 /*   520 */    87,   87,   87,   87,   63,   64,   87,   66,   67,   87,
 /*   530 */    87,   87,   87,   87,   55,   87,   75,   76,   87,   78,
 /*   540 */    79,   80,   63,   64,   87,   66,   67,   87,   87,   87,
 /*   550 */    87,   87,   87,   55,   75,   76,   87,   78,   79,   80,
 /*   560 */    55,   63,   64,   87,   66,   67,   87,   87,   87,   64,
 /*   570 */    87,   66,   67,   75,   76,   87,   78,   79,   80,   87,
 /*   580 */    75,   76,   87,   55,   79,   80,   87,   87,   87,   87,
 /*   590 */    55,   86,   64,   87,   66,   67,   87,   87,   87,   64,
 /*   600 */    87,   66,   67,   75,   76,   87,   87,   79,   80,   55,
 /*   610 */    75,   76,   87,   87,   79,   80,   55,   87,   64,   87,
 /*   620 */    66,   67,   87,   87,   87,   64,   87,   66,   67,   75,
 /*   630 */    76,   87,   87,   79,   80,   87,   75,   76,   87,   55,
 /*   640 */    79,   80,   87,   87,   87,   87,   55,   87,   64,   87,
 /*   650 */    66,   67,   87,   87,   87,   64,   87,   66,   67,   75,
 /*   660 */    76,   87,   87,   79,   80,   55,   75,   76,   87,   87,
 /*   670 */    79,   80,   55,   87,   64,   87,   66,   67,   87,   87,
 /*   680 */    87,   64,   87,   66,   67,   75,   76,   87,   87,   79,
 /*   690 */    80,   87,   75,   76,   87,   55,   79,   80,   87,   87,
 /*   700 */    87,   87,   55,   87,   64,   87,   66,   67,   87,   87,
 /*   710 */    87,   64,   87,   66,   67,   75,   76,   87,   87,   79,
 /*   720 */    80,   55,   75,   76,   87,   87,   79,   80,   55,   87,
 /*   730 */    64,   87,   66,   67,   87,   87,   87,   64,   87,   66,
 /*   740 */    67,   75,   76,   87,   87,   79,   80,   87,   75,   76,
 /*   750 */    87,   55,   79,   80,   87,   87,   87,   87,   55,   87,
 /*   760 */    64,   87,   66,   67,   87,   87,   87,   64,   87,   66,
 /*   770 */    67,   75,   76,   87,   87,   79,   80,   55,   75,   76,
 /*   780 */    87,   87,   79,   80,   55,   87,   64,   87,   66,   67,
 /*   790 */    87,   87,   87,   64,   87,   66,   67,   75,   76,   87,
 /*   800 */    87,   79,   80,   87,   75,   76,   87,   55,   79,   80,
 /*   810 */    87,   87,   87,   87,   55,   87,   64,   87,   66,   67,
 /*   820 */    87,   87,   87,   64,   87,   66,   67,   75,   76,   87,
 /*   830 */    87,   79,   80,   55,   75,   76,   87,   87,   79,   80,
 /*   840 */    55,   87,   64,   87,   66,   67,   87,   87,   87,   64,
 /*   850 */    87,   66,   67,   75,   76,   87,   87,   79,   80,   87,
 /*   860 */    75,   76,   87,   55,   79,   80,   87,   87,   87,   87,
 /*   870 */    55,   87,   64,   87,   66,   67,   87,   87,   87,   64,
 /*   880 */    87,   66,   67,   75,   76,   87,   87,   79,   80,   55,
 /*   890 */    75,   76,   87,   87,   79,   80,   55,   87,   64,   87,
 /*   900 */    66,   67,   87,   87,   87,   64,   87,   66,   67,   75,
 /*   910 */    76,   87,   87,   79,   80,   87,   75,   76,   87,   55,
 /*   920 */    79,   80,   87,   87,   87,   87,   55,   87,   64,   87,
 /*   930 */    66,   67,   87,   87,   87,   64,   87,   66,   67,   75,
 /*   940 */    76,   87,   87,   79,   80,   55,   75,   76,   87,   87,
 /*   950 */    79,   80,   55,   87,   64,   87,   66,   67,   87,   87,
 /*   960 */    87,   64,   87,   66,   67,   75,   76,   87,   87,   79,
 /*   970 */    80,   87,   75,   76,   87,   55,   79,   80,   87,   87,
 /*   980 */    87,   87,   55,   87,   64,   87,   66,   67,   87,   87,
 /*   990 */    87,   64,   87,   66,   67,   75,   76,   87,   87,   79,
 /*  1000 */    80,   55,   75,   76,   87,   87,   79,   80,   87,   87,
 /*  1010 */    64,   87,   66,   67,   87,   87,   87,   87,   87,   87,
 /*  1020 */    87,   75,   76,   87,   87,   79,   80,
];
const YY_SHIFT_USE_DFLT: i32 = -42;
const YY_SHIFT_COUNT: i32 = 136;
const YY_SHIFT_MIN: i32 = -41;
const YY_SHIFT_MAX: i32 = 427;
const YY_SHIFT_OFST: [i16; 137] = [
 /*     0 */   -42,  230,  405,  354,  332,  427,  427,  427,  281,  427,
 /*    10 */   427,  427,  427,  427,  427,  427,  427,  427,  427,  427,
 /*    20 */   427,  427,  427,  427,  427,  427,  427,  427,  427,  427,
 /*    30 */   427,  427,  427,  427,  427,  427,  427,  427,  427,  427,
 /*    40 */   255,  158,  253,  345,  345,  317,  317,  145,  -42,  -42,
 /*    50 */   -42,  -42,  -42,  -42,  -42,  -42,  -42,  -42,  -42,    1,
 /*    60 */   110,  345,  346,  345,  345,  345,  346,  345,  345,  345,
 /*    70 */   345,  346,  -42,   -4,  169,  152,  133,  101,   69,   36,
 /*    80 */    13,  -21,  186,  186,  186,  186,  186,  186,   52,  205,
 /*    90 */    81,   81,   81,   81,   81,   81,   81,  289,  -41,  -41,
 /*   100 */    55,   54,  -23,  339,  314,  333,  327,  322,  316,  305,
 /*   110 */   310,  308,  303,  298,  304,  296,  295,  245,  291,  284,
 /*   120 */   238,  276,  226,  194,  221,  180,  148,  178,   35,   35,
 /*   130 */    35,   35,   35,   35,   35,  140,    7,
];
const YY_REDUCE_USE_DFLT: i32 = -17;
const YY_REDUCE_COUNT: i32 = 72;
const YY_REDUCE_MIN: i32 = -16;
const YY_REDUCE_MAX: i32 = 946;
const YY_REDUCE_OFST: [i16; 73] = [
 /*     0 */    73,  374,  239,  498,  479,  461,  434,  428,  505,  946,
 /*    10 */   927,  920,  897,  890,  871,  864,  841,  834,  815,  808,
 /*    20 */   785,  778,  759,  752,  729,  722,  703,  696,  673,  666,
 /*    30 */   647,  640,  617,  610,  591,  584,  561,  554,  535,  528,
 /*    40 */    80,   51,  187,   87,  184,  182,  182,  175,  232,  215,
 /*    50 */   212,  210,  208,  157,  121,   97,   40,    4,  -16,  307,
 /*    60 */   272,  301,  302,  300,  299,  297,  292,  290,  288,  286,
 /*    70 */   285,  277,  279,
];
const YY_DEFAULT: [YYACTIONTYPE; 197] = [
 /*     0 */   200,  300,  198,  225,  300,  300,  300,  300,  292,  300,
 /*    10 */   300,  300,  300,  300,  300,  300,  300,  300,  300,  300,
 /*    20 */   300,  300,  300,  300,  300,  300,  300,  300,  300,  300,
 /*    30 */   300,  300,  300,  300,  300,  300,  300,  300,  300,  300,
 /*    40 */   300,  300,  300,  300,  300,  300,  253,  300,  200,  200,
 /*    50 */   200,  200,  200,  200,  200,  200,  200,  200,  200,  223,
 /*    60 */   291,  300,  300,  300,  300,  300,  300,  300,  300,  300,
 /*    70 */   300,  300,  221,  300,  300,  300,  300,  300,  300,  300,
 /*    80 */   300,  300,  211,  244,  245,  297,  295,  296,  258,  259,
 /*    90 */   260,  266,  265,  264,  263,  262,  261,  205,  268,  267,
 /*   100 */   274,  274,  230,  234,  300,  227,  218,  300,  300,  300,
 /*   110 */   300,  300,  241,  300,  300,  300,  300,  300,  243,  300,
 /*   120 */   300,  237,  300,  286,  300,  300,  300,  240,  272,  271,
 /*   130 */   270,  269,  257,  256,  255,  300,  300,  229,  228,  226,
 /*   140 */   220,  219,  217,  232,  233,  231,  216,  215,  214,  213,
 /*   150 */   224,  222,  212,  210,  209,  208,  207,  206,  282,  281,
 /*   160 */   280,  279,  278,  277,  239,  293,  298,  294,  290,  289,
 /*   170 */   287,  288,  285,  284,  241,  299,  242,  283,  276,  275,
 /*   180 */   274,  273,  254,  252,  251,  250,  249,  248,  247,  246,
 /*   190 */   238,  237,  204,  203,  202,  201,  199,
];

/* TMPL: fallback tokens */

const YY_FALLBACK: [i32; 5] = [
    0,  /*          $ => nothing */
    0,  /* OPENPAREN2 => nothing */
    1,  /*  OPENPAREN => OPENPAREN2 */
    0,  /*     COMMA2 => nothing */
    3,  /*      COMMA => COMMA2 */
];

/* TMPL: symbol names */


/* TMPL: rules */


/* TMPL: destructors */


/* TMPL: stack-overflow */


/* TMPL: stack-overflow */

const YY_RULE_INFO: [YYCODETYPE; 103] = [
  56,
  57,
  57,
  58,
  58,
  60,
  60,
  61,
  61,
  61,
  61,
  61,
  61,
  61,
  61,
  61,
  61,
  61,
  61,
  61,
  61,
  61,
  61,
  61,
  68,
  68,
  69,
  69,
  59,
  59,
  59,
  59,
  65,
  71,
  71,
  73,
  73,
  62,
  74,
  74,
  75,
  75,
  75,
  70,
  77,
  77,
  63,
  78,
  78,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  67,
  76,
  76,
  76,
  64,
  64,
  81,
  81,
  81,
  81,
  79,
  72,
  72,
  82,
  82,
  82,
  80,
  80,
  83,
  83,
  84,
  84,
  86,
  86,
  86,
  85,
  66,
];

struct YYStackEntry {
    stateno: i32, /* The state-number */
    major: i32,     /* The major token value.  This is the code
                            ** number for the token at this stack level */
    minor: YYMinorType,    /* The user-supplied minor token value.  This
                            ** is the value of the token  */
}

pub struct Parser {
    yyerrcnt: i32, /* Shifts left before out of the error */
    yystack: Vec<YYStackEntry>,
    chunk: Option<Chunk>,
}

impl Parser {

    pub fn new(
        ) -> Parser {
        //let mut p = Parser { yyerrcnt: -1, yystack: Vec::new(), extra: extra};
        let mut p = Parser { yyerrcnt: -1, yystack: Vec::new(), chunk: None};
        p.yystack.push(YYStackEntry{stateno: 0, major: 0, minor: YYMinorType::YY0});
        p
    }


    pub fn into_chunk(self) -> Option<Chunk> {
        self.chunk
    }

    pub fn parse(&mut self, token: Token) {

        let yymajor = token_major(&token);
        let yyendofinput = yymajor==0;
        let mut yyerrorhit = false;
        while !self.yystack.is_empty() {
            let yyact = self.find_shift_action(yymajor);
            if yyact < YYNSTATE {
                assert!(!yyendofinput);  /* Impossible to shift the $ token */
                let yyminor = token_minor(token);
                self.yy_shift(yyact, yymajor, yyminor);
                self.yyerrcnt -= 1;
                break;
            } else if yyact < YYNSTATE + YYNRULE {
                self.yy_reduce(yyact - YYNSTATE);
            } else {
                /* A syntax error has occurred.
                 ** The response to an error depends upon whether or not the
                 ** grammar defines an error token "ERROR".
                 */
                assert!(yyact == YYNSTATE+YYNRULE);
                if YYERRORSYMBOL != 0 {
                    /* This is what we do if the grammar does define ERROR:
                     **
                     **  * Call the %syntax_error function.
                     **
                     **  * Begin popping the stack until we enter a state where
                     **    it is legal to shift the error symbol, then shift
                     **    the error symbol.
                     **
                     **  * Set the error count to three.
                     **
                     **  * Begin accepting and shifting new tokens.  No new error
                     **    processing will occur until three tokens have been
                     **    shifted successfully.
                     **
                     */
                    if self.yyerrcnt < 0 {
                        self.yy_syntax_error(&token);
                    }
                    let yymx = self.yystack[self.yystack.len() - 1].major;
                    if yymx==YYERRORSYMBOL || yyerrorhit {
                        break;
                    } else {
                        let mut yyact;
                        while !self.yystack.is_empty() {
                            yyact = self.find_reduce_action(YYERRORSYMBOL);
                            if yyact < YYNSTATE {
                                if !yyendofinput {
                                    self.yy_shift(yyact, YYERRORSYMBOL, YYMinorType::YY0);
                                }
                                break;
                            }
                            self.yystack.pop().unwrap();
                        }
                        if self.yystack.is_empty() || yyendofinput {
                            self.yy_parse_failed();
                            break;
                        }
                    }
                    self.yyerrcnt = 3;
                    yyerrorhit = true;
                } else {
                    /* This is what we do if the grammar does not define ERROR:
                     **
                     **  * Report an error message, and throw away the input token.
                     **
                     **  * If the input token is $, then fail the parse.
                     **
                     ** As before, subsequent error messages are suppressed until
                     ** three input tokens have been successfully shifted.
                     */
                    if self.yyerrcnt <= 0 {
                        self.yy_syntax_error(&token);
                    }
                    self.yyerrcnt = 3;
                    if yyendofinput {
                        self.yy_parse_failed();
                    }
                    break;
                }
            }
        }
    }

    /*
    ** Find the appropriate action for a parser given the terminal
    ** look-ahead token look_ahead.
    */
    fn find_shift_action(&self, look_ahead: i32) -> i32 {

        let stateno = self.yystack[self.yystack.len() - 1].stateno;

        if stateno > YY_SHIFT_COUNT {
            return YY_DEFAULT[stateno as usize] as i32;
        }
        let i = YY_SHIFT_OFST[stateno as usize] as i32;
        if i == YY_SHIFT_USE_DFLT {
            return YY_DEFAULT[stateno as usize] as i32;
        }
        assert!(look_ahead != YYNOCODE);
        let i = i + look_ahead;

        if i < 0 || i >= YY_ACTTAB_COUNT || YY_LOOKAHEAD[i as usize] as i32 != look_ahead {
            if look_ahead > 0 {
                if (look_ahead as usize) < YY_FALLBACK.len() {
                    let fallback = YY_FALLBACK[look_ahead as usize];
                    if fallback != 0 {
                        //println!("FALLBACK");
                        return self.find_shift_action(fallback);
                    }
                }
                if YYWILDCARD > 0 {
                    let j = i - look_ahead + (YYWILDCARD as i32);
                    if j >= 0 && j < YY_ACTTAB_COUNT && YY_LOOKAHEAD[j as usize]==YYWILDCARD {
                        println!("WILDCARD");
                        return YY_ACTION[j as usize] as i32;
                    }
                }
            }
            return YY_DEFAULT[stateno as usize] as i32;
        } else {
            return YY_ACTION[i as usize] as i32;
        }
    }

    /*
    ** Find the appropriate action for a parser given the non-terminal
    ** look-ahead token iLookAhead.
    */
    fn find_reduce_action(&self, look_ahead: i32) -> i32 {
        let stateno = self.yystack[self.yystack.len() - 1].stateno;
        if YYERRORSYMBOL != 0 && stateno > YY_REDUCE_COUNT {
            return YY_DEFAULT[stateno as usize] as i32;
        }
        assert!(stateno <= YY_REDUCE_COUNT);
        let i = YY_REDUCE_OFST[stateno as usize] as i32;
        assert!(i != YY_REDUCE_USE_DFLT);
        assert!(look_ahead != YYNOCODE );
        let i = i + look_ahead;
        if YYERRORSYMBOL != 0 && (i < 0 || i >= YY_ACTTAB_COUNT || YY_LOOKAHEAD[i as usize] as i32 != look_ahead) {
            return YY_DEFAULT[stateno as usize] as i32;
        }
        assert!(i >= 0 && i < YY_ACTTAB_COUNT);
        assert!(YY_LOOKAHEAD[i as usize] as i32 == look_ahead);
        return YY_ACTION[i as usize] as i32;
    }

    fn yy_shift(&mut self, new_state: i32, major: i32, minor: YYMinorType) {
        self.yystack.push(YYStackEntry{stateno: new_state, major: major, minor: minor});
    }

    fn yy_reduce(&mut self, yyruleno: i32) {

        let yygotominor: YYMinorType = match yyruleno {
            /* Beginning here are the reduction cases.  */
            0 /* chunk ::= block */
            => 
{
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY36(yy0),) => {
 self.chunk = Some(Chunk { block: yy0 }) 
},    _ => unreachable!() };
 YYMinorType::YY0
}
            ,
            1 /* block ::= stats_ */
            => 
{
let yyres :  Block ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY115(yy0),) => {
 yyres = Block { stmts: yy0.into_boxed_slice(), ret: None }; 
},    _ => unreachable!() };
 YYMinorType::YY36(yyres)
}
            ,
            2 /* block ::= stats_ retstat */
            => 
{
let yyres :  Block ;
let yyp1 = self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp1.minor,) {
 (YYMinorType::YY115(yy0),YYMinorType::YY104(yy1),) => {
 yyres = Block { stmts: yy0.into_boxed_slice(), ret: Some(yy1) }; 
},    _ => unreachable!() };
 YYMinorType::YY36(yyres)
}
            ,
            3 /* stats_ ::= */
            => 
{
let yyres :  Vec<Stmt> ;
match () {
 () => {
 yyres = vec![]; 
} };
 YYMinorType::YY115(yyres)
}
            ,
            4 /* stats_ ::= stats_ stat_ */
            => 
{
let yyres :  Vec<Stmt> ;
let yyp1 = self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp1.minor,) {
 (YYMinorType::YY115(yy0),YYMinorType::YY162(yy1),) => {
 yyres = if let Some(s) = yy1 { let mut v = yy0; v.push(s); v } else { yy0 } 
},    _ => unreachable!() };
 YYMinorType::YY115(yyres)
}
            ,
            5 /* stat_ ::= SEMI */
            => 
{
let yyres :  Option<Stmt> ;
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = None; 
} };
 YYMinorType::YY162(yyres)
}
            ,
            6 /* stat_ ::= stat */
            => 
{
let yyres :  Option<Stmt> ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY101(yy0),) => {
 yyres = Some(yy0); 
},    _ => unreachable!() };
 YYMinorType::YY162(yyres)
}
            ,
            7 /* stat ::= varlist EQ explist */
            => 
{
let yyres :  Stmt ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY142(yy0),YYMinorType::YY104(yy2),) => {
 yyres = StmtSet(yy0, yy2); 
},    _ => unreachable!() };
 YYMinorType::YY101(yyres)
}
            ,
            8 /* stat ::= functioncall */
            => 
{
let yyres :  Stmt ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY116(yy0),) => {
 yyres = StmtCall(P(yy0)); 
},    _ => unreachable!() };
 YYMinorType::YY101(yyres)
}
            ,
            9 /* stat ::= label */
            => 
{
let yyres :  Stmt ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY120(yy0),) => {
 yyres = StmtLabel(yy0); 
},    _ => unreachable!() };
 YYMinorType::YY101(yyres)
}
            ,
            10 /* stat ::= BREAK */
            => 
{
let yyres :  Stmt ;
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = StmtBreak; 
} };
 YYMinorType::YY101(yyres)
}
            ,
            11 /* stat ::= GOTO name */
            => 
{
let yyres :  Stmt ;
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY120(yy1),) => {
 yyres = StmtGoto(yy1); 
},    _ => unreachable!() };
 YYMinorType::YY101(yyres)
}
            ,
            12 /* stat ::= DO block END */
            => 
{
let yyres :  Stmt ;
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY36(yy1),) => {
 yyres = StmtDo(P(yy1)); 
},    _ => unreachable!() };
 YYMinorType::YY101(yyres)
}
            ,
            13 /* stat ::= WHILE exp DO block END */
            => 
{
let yyres :  Stmt ;
self.yystack.pop().unwrap();
let yyp3 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,yyp3.minor,) {
 (YYMinorType::YY78(yy1),YYMinorType::YY36(yy3),) => {
 yyres = StmtWhile(P(yy1), P(yy3)); 
},    _ => unreachable!() };
 YYMinorType::YY101(yyres)
}
            ,
            14 /* stat ::= REPEAT block UNTIL exp */
            => 
{
let yyres :  Stmt ;
let yyp3 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,yyp3.minor,) {
 (YYMinorType::YY36(yy1),YYMinorType::YY78(yy3),) => {
 yyres = StmtRepeat(P(yy1), P(yy3)); 
},    _ => unreachable!() };
 YYMinorType::YY101(yyres)
}
            ,
            15 /* stat ::= IF exp THEN block elsifs_ else_ END */
            => 
{
let yyres :  Stmt ;
self.yystack.pop().unwrap();
let yyp5 = self.yystack.pop().unwrap();
let yyp4 = self.yystack.pop().unwrap();
let yyp3 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,yyp3.minor,yyp4.minor,yyp5.minor,) {
 (YYMinorType::YY78(yy1),YYMinorType::YY36(yy3),YYMinorType::YY18(yy4),YYMinorType::YY95(yy5),) => {
 yyres = StmtIf(P(yy1), P(yy3), yy4.into_boxed_slice(), yy5); 
},    _ => unreachable!() };
 YYMinorType::YY101(yyres)
}
            ,
            16 /* stat ::= FOR name EQ exp COMMA exp DO block END */
            => 
{
let yyres :  Stmt ;
self.yystack.pop().unwrap();
let yyp7 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp5 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp3 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,yyp3.minor,yyp5.minor,yyp7.minor,) {
 (YYMinorType::YY120(yy1),YYMinorType::YY78(yy3),YYMinorType::YY78(yy5),YYMinorType::YY36(yy7),) => {
 yyres = StmtForNum(yy1, P(yy3), P(yy5), None, P(yy7)); 
},    _ => unreachable!() };
 YYMinorType::YY101(yyres)
}
            ,
            17 /* stat ::= FOR name EQ exp COMMA exp COMMA exp DO block END */
            => 
{
let yyres :  Stmt ;
self.yystack.pop().unwrap();
let yyp9 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp7 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp5 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp3 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,yyp3.minor,yyp5.minor,yyp7.minor,yyp9.minor,) {
 (YYMinorType::YY120(yy1),YYMinorType::YY78(yy3),YYMinorType::YY78(yy5),YYMinorType::YY78(yy7),YYMinorType::YY36(yy9),) => {
 yyres = StmtForNum(yy1, P(yy3), P(yy5), Some(P(yy7)), P(yy9)); 
},    _ => unreachable!() };
 YYMinorType::YY101(yyres)
}
            ,
            18 /* stat ::= FOR namelist IN explist DO block END */
            => 
{
let yyres :  Stmt ;
self.yystack.pop().unwrap();
let yyp5 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp3 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,yyp3.minor,yyp5.minor,) {
 (YYMinorType::YY68(yy1),YYMinorType::YY104(yy3),YYMinorType::YY36(yy5),) => {
 yyres = StmtForIn(yy1, yy3, P(yy5)); 
},    _ => unreachable!() };
 YYMinorType::YY101(yyres)
}
            ,
            19 /* stat ::= FUNCTION funcname funcbody */
            => 
{
let yyres :  Stmt ;
let yyp2 = self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,yyp2.minor,) {
 (YYMinorType::YY131(yy1),YYMinorType::YY114(yy2),) => {
 yyres = StmtFunction(yy1.0, yy1.1, yy2.0, yy2.1, yy2.2); 
},    _ => unreachable!() };
 YYMinorType::YY101(yyres)
}
            ,
            20 /* stat ::= LOCAL FUNCTION name funcbody */
            => 
{
let yyres :  Stmt ;
let yyp3 = self.yystack.pop().unwrap();
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp2.minor,yyp3.minor,) {
 (YYMinorType::YY120(yy2),YYMinorType::YY114(yy3),) => {
 yyres = StmtLocalFunction(yy2, yy3.0, yy3.1, yy3.2); 
},    _ => unreachable!() };
 YYMinorType::YY101(yyres)
}
            ,
            21 /* stat ::= LOCAL namelist */
            => 
{
let yyres :  Stmt ;
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY68(yy1),) => {
 yyres = StmtLocal(yy1, None); 
},    _ => unreachable!() };
 YYMinorType::YY101(yyres)
}
            ,
            22 /* stat ::= LOCAL namelist EQ explist */
            => 
{
let yyres :  Stmt ;
let yyp3 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,yyp3.minor,) {
 (YYMinorType::YY68(yy1),YYMinorType::YY104(yy3),) => {
 yyres = StmtLocal(yy1, Some(yy3)); 
},    _ => unreachable!() };
 YYMinorType::YY101(yyres)
}
            ,
            23 /* stat ::= error */
            => 
{
let yyres :  Stmt ;
let yyp0 = self.yystack.pop().unwrap();
match () {
 () => {
 yyres = StmtInvalid 
} };
 YYMinorType::YY101(yyres)
}
            ,
            24 /* elsifs_ ::= */
            => 
{
let yyres :  Vec<(Expr, Block)> ;
match () {
 () => {
 yyres = vec![]; 
} };
 YYMinorType::YY18(yyres)
}
            ,
            25 /* elsifs_ ::= elsifs_ ELSEIF exp THEN block */
            => 
{
let yyres :  Vec<(Expr, Block)> ;
let yyp4 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,yyp4.minor,) {
 (YYMinorType::YY18(yy0),YYMinorType::YY78(yy2),YYMinorType::YY36(yy4),) => {
 yyres = { let mut v = yy0; v.push((yy2, yy4)); v }; 
},    _ => unreachable!() };
 YYMinorType::YY18(yyres)
}
            ,
            26 /* else_ ::= */
            => 
{
let yyres :  Option<P<Block>> ;
match () {
 () => {
 yyres = None; 
} };
 YYMinorType::YY95(yyres)
}
            ,
            27 /* else_ ::= ELSE block */
            => 
{
let yyres :  Option<P<Block>> ;
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY36(yy1),) => {
 yyres = Some(P(yy1)); 
},    _ => unreachable!() };
 YYMinorType::YY95(yyres)
}
            ,
            28 /* retstat ::= RETURN */
            => 
{
let yyres :  V<Expr> ;
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = P([]); 
} };
 YYMinorType::YY104(yyres)
}
            ,
            29 /* retstat ::= RETURN SEMI */
            => 
{
let yyres :  V<Expr> ;
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = P([]); 
} };
 YYMinorType::YY104(yyres)
}
            ,
            30 /* retstat ::= RETURN explist */
            => 
{
let yyres :  V<Expr> ;
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY104(yy1),) => {
 yyres = yy1; 
},    _ => unreachable!() };
 YYMinorType::YY104(yyres)
}
            ,
            31 /* retstat ::= RETURN explist SEMI */
            => 
{
let yyres :  V<Expr> ;
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY104(yy1),) => {
 yyres = yy1; 
},    _ => unreachable!() };
 YYMinorType::YY104(yyres)
}
            ,
            32 /* label ::= COLONCOLON name COLONCOLON */
            => 
{
let yyres :  Name ;
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY120(yy1),) => {
 yyres = yy1; 
},    _ => unreachable!() };
 YYMinorType::YY120(yyres)
}
            ,
            33 /* funcname ::= dotnames_ */
            => 
{
let yyres :  (V<Name>, Option<Name>) ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY79(yy0),) => {
 yyres = (yy0.into_boxed_slice(), None); 
},    _ => unreachable!() };
 YYMinorType::YY131(yyres)
}
            ,
            34 /* funcname ::= dotnames_ COLON name */
            => 
{
let yyres :  (V<Name>, Option<Name>) ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY79(yy0),YYMinorType::YY120(yy2),) => {
 yyres = (yy0.into_boxed_slice(), Some(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY131(yyres)
}
            ,
            35 /* dotnames_ ::= name */
          | 44 /* namelist_ ::= name */
            => 
{
let yyres :  Vec<Name> ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY120(yy0),) => {
 yyres = vec![yy0]; 
},    _ => unreachable!() };
 YYMinorType::YY79(yyres)
}
            ,
            36 /* dotnames_ ::= dotnames_ DOT name */
            => 
{
let yyres :  Vec<Name> ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY79(yy0),YYMinorType::YY120(yy2),) => {
 yyres = { let mut v = yy0; v.push(yy2); v }; 
},    _ => unreachable!() };
 YYMinorType::YY79(yyres)
}
            ,
            37 /* varlist ::= varlist_ */
            => 
{
let yyres :  V<Var> ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY92(yy0),) => {
 yyres = yy0.into_boxed_slice(); 
},    _ => unreachable!() };
 YYMinorType::YY142(yyres)
}
            ,
            38 /* varlist_ ::= var */
            => 
{
let yyres :  Vec<Var> ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY98(yy0),) => {
 yyres = vec![yy0]; 
},    _ => unreachable!() };
 YYMinorType::YY92(yyres)
}
            ,
            39 /* varlist_ ::= varlist_ COMMA var */
            => 
{
let yyres :  Vec<Var> ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY92(yy0),YYMinorType::YY98(yy2),) => {
 yyres = { let mut v = yy0; v.push(yy2); v } 
},    _ => unreachable!() };
 YYMinorType::YY92(yyres)
}
            ,
            40 /* var ::= name */
            => 
{
let yyres :  Var ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY120(yy0),) => {
 yyres = VarName(yy0); 
},    _ => unreachable!() };
 YYMinorType::YY98(yyres)
}
            ,
            41 /* var ::= prefixexp OPENBRACKET exp CLOSEBRACKET */
            => 
{
let yyres :  Var ;
self.yystack.pop().unwrap();
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY78(yy0),YYMinorType::YY78(yy2),) => {
 yyres = VarProperty(P(yy0), P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY98(yyres)
}
            ,
            42 /* var ::= prefixexp DOT name */
            => 
{
let yyres :  Var ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY78(yy0),YYMinorType::YY120(yy2),) => {
 yyres = VarMember(P(yy0), yy2); 
},    _ => unreachable!() };
 YYMinorType::YY98(yyres)
}
            ,
            43 /* namelist ::= namelist_ */
            => 
{
let yyres :  V<Name> ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY79(yy0),) => {
 yyres = yy0.into_boxed_slice(); 
},    _ => unreachable!() };
 YYMinorType::YY68(yyres)
}
            ,
            45 /* namelist_ ::= namelist_ COMMA name */
            => 
{
let yyres :  Vec<Name> ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY79(yy0),YYMinorType::YY120(yy2),) => {
 yyres = { let mut v = yy0; v.push(yy2); v } 
},    _ => unreachable!() };
 YYMinorType::YY79(yyres)
}
            ,
            46 /* explist ::= explist_ */
            => 
{
let yyres :  V<Expr> ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY117(yy0),) => {
 yyres = yy0.into_boxed_slice(); 
},    _ => unreachable!() };
 YYMinorType::YY104(yyres)
}
            ,
            47 /* explist_ ::= exp */
            => 
{
let yyres :  Vec<Expr> ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY78(yy0),) => {
 yyres = vec![yy0]; 
},    _ => unreachable!() };
 YYMinorType::YY117(yyres)
}
            ,
            48 /* explist_ ::= explist_ COMMA exp */
            => 
{
let yyres :  Vec<Expr> ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY117(yy0),YYMinorType::YY78(yy2),) => {
 yyres = { let mut v = yy0; v.push(yy2); v } 
},    _ => unreachable!() };
 YYMinorType::YY117(yyres)
}
            ,
            49 /* exp ::= NIL */
            => 
{
let yyres :  Expr ;
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = ExprNil; 
} };
 YYMinorType::YY78(yyres)
}
            ,
            50 /* exp ::= FALSE */
            => 
{
let yyres :  Expr ;
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = ExprFalse; 
} };
 YYMinorType::YY78(yyres)
}
            ,
            51 /* exp ::= TRUE */
            => 
{
let yyres :  Expr ;
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = ExprTrue; 
} };
 YYMinorType::YY78(yyres)
}
            ,
            52 /* exp ::= DOTDOTDOT */
            => 
{
let yyres :  Expr ;
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = ExprEllipsis; 
} };
 YYMinorType::YY78(yyres)
}
            ,
            53 /* exp ::= NUMBER */
            => 
{
let yyres :  Expr ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY71(yy0),) => {
 yyres = ExprNumber(yy0); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            54 /* exp ::= STRING */
            => 
{
let yyres :  Expr ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY40(yy0),) => {
 yyres = ExprString(yy0); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            55 /* exp ::= functiondef */
            => 
{
let yyres :  Expr ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY114(yy0),) => {
 yyres = ExprFunction(yy0.0, yy0.1, yy0.2); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            56 /* exp ::= prefixexp */
            => 
{
let yyres :  Expr ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY78(yy0),) => {
 yyres = yy0; 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            57 /* exp ::= tableconstructor */
            => 
{
let yyres :  Expr ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY147(yy0),) => {
 yyres = ExprTable(yy0); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            58 /* exp ::= NOT exp */
            => 
{
let yyres :  Expr ;
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY78(yy1),) => {
 yyres = ExprUnary(UnNot, P(yy1)); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            59 /* exp ::= POUND exp */
            => 
{
let yyres :  Expr ;
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY78(yy1),) => {
 yyres = ExprUnary(UnLen, P(yy1)); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            60 /* exp ::= MINUS exp */
            => 
{
let yyres :  Expr ;
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY78(yy1),) => {
 yyres = ExprUnary(UnNeg, P(yy1)); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            61 /* exp ::= exp OR exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY78(yy0),YYMinorType::YY78(yy2),) => {
 yyres = ExprBinary(P(yy0), BinOr, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            62 /* exp ::= exp AND exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY78(yy0),YYMinorType::YY78(yy2),) => {
 yyres = ExprBinary(P(yy0), BinAnd, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            63 /* exp ::= exp LT exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY78(yy0),YYMinorType::YY78(yy2),) => {
 yyres = ExprBinary(P(yy0), BinLt, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            64 /* exp ::= exp LE exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY78(yy0),YYMinorType::YY78(yy2),) => {
 yyres = ExprBinary(P(yy0), BinLe, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            65 /* exp ::= exp GT exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY78(yy0),YYMinorType::YY78(yy2),) => {
 yyres = ExprBinary(P(yy0), BinGt, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            66 /* exp ::= exp GE exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY78(yy0),YYMinorType::YY78(yy2),) => {
 yyres = ExprBinary(P(yy0), BinGe, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            67 /* exp ::= exp EQEQ exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY78(yy0),YYMinorType::YY78(yy2),) => {
 yyres = ExprBinary(P(yy0), BinEq, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            68 /* exp ::= exp NE exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY78(yy0),YYMinorType::YY78(yy2),) => {
 yyres = ExprBinary(P(yy0), BinNe, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            69 /* exp ::= exp DOTDOT exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY78(yy0),YYMinorType::YY78(yy2),) => {
 yyres = ExprBinary(P(yy0), BinCon, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            70 /* exp ::= exp PLUS exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY78(yy0),YYMinorType::YY78(yy2),) => {
 yyres = ExprBinary(P(yy0), BinAdd, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            71 /* exp ::= exp MINUS exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY78(yy0),YYMinorType::YY78(yy2),) => {
 yyres = ExprBinary(P(yy0), BinSub, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            72 /* exp ::= exp STAR exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY78(yy0),YYMinorType::YY78(yy2),) => {
 yyres = ExprBinary(P(yy0), BinMul, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            73 /* exp ::= exp SLASH exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY78(yy0),YYMinorType::YY78(yy2),) => {
 yyres = ExprBinary(P(yy0), BinDiv, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            74 /* exp ::= exp PERCENT exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY78(yy0),YYMinorType::YY78(yy2),) => {
 yyres = ExprBinary(P(yy0), BinMod, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            75 /* exp ::= exp CARET exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY78(yy0),YYMinorType::YY78(yy2),) => {
 yyres = ExprBinary(P(yy0), BinPow, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            76 /* exp ::= error */
            => 
{
let yyres :  Expr ;
let yyp0 = self.yystack.pop().unwrap();
match () {
 () => {
 yyres = ExprInvalid 
} };
 YYMinorType::YY78(yyres)
}
            ,
            77 /* prefixexp ::= var */
            => 
{
let yyres :  Expr ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY98(yy0),) => {
 yyres = ExprVar(P(yy0)); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            78 /* prefixexp ::= functioncall */
            => 
{
let yyres :  Expr ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY116(yy0),) => {
 yyres = ExprCall(P(yy0)); 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            79 /* prefixexp ::= OPENPAREN2 exp CLOSEPAREN */
            => 
{
let yyres :  Expr ;
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY78(yy1),) => {
 yyres = yy1; 
},    _ => unreachable!() };
 YYMinorType::YY78(yyres)
}
            ,
            80 /* functioncall ::= prefixexp args */
            => 
{
let yyres :  Call ;
let yyp1 = self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp1.minor,) {
 (YYMinorType::YY78(yy0),YYMinorType::YY69(yy1),) => {
 yyres = CallFunction(P(yy0), P(yy1)); 
},    _ => unreachable!() };
 YYMinorType::YY116(yyres)
}
            ,
            81 /* functioncall ::= prefixexp COLON name args */
            => 
{
let yyres :  Call ;
let yyp3 = self.yystack.pop().unwrap();
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,yyp3.minor,) {
 (YYMinorType::YY78(yy0),YYMinorType::YY120(yy2),YYMinorType::YY69(yy3),) => {
 yyres = CallMethod(P(yy0), yy2, P(yy3)); 
},    _ => unreachable!() };
 YYMinorType::YY116(yyres)
}
            ,
            82 /* args ::= OPENPAREN CLOSEPAREN */
            => 
{
let yyres :  Args ;
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = ArgsParen(P([])); 
} };
 YYMinorType::YY69(yyres)
}
            ,
            83 /* args ::= OPENPAREN explist CLOSEPAREN */
            => 
{
let yyres :  Args ;
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY104(yy1),) => {
 yyres = ArgsParen(yy1); 
},    _ => unreachable!() };
 YYMinorType::YY69(yyres)
}
            ,
            84 /* args ::= tableconstructor */
            => 
{
let yyres :  Args ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY147(yy0),) => {
 yyres = ArgsTable(yy0); 
},    _ => unreachable!() };
 YYMinorType::YY69(yyres)
}
            ,
            85 /* args ::= STRING */
            => 
{
let yyres :  Args ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY40(yy0),) => {
 yyres = ArgsString(yy0); 
},    _ => unreachable!() };
 YYMinorType::YY69(yyres)
}
            ,
            86 /* functiondef ::= FUNCTION funcbody */
            => 
{
let yyres :  (V<Name>, bool, P<Block>) ;
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY114(yy1),) => {
 yyres = yy1; 
},    _ => unreachable!() };
 YYMinorType::YY114(yyres)
}
            ,
            87 /* funcbody ::= OPENPAREN CLOSEPAREN block END */
            => 
{
let yyres :  (V<Name>, bool, P<Block>) ;
self.yystack.pop().unwrap();
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp2.minor,) {
 (YYMinorType::YY36(yy2),) => {
 yyres = (P([]), false, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY114(yyres)
}
            ,
            88 /* funcbody ::= OPENPAREN parlist CLOSEPAREN block END */
            => 
{
let yyres :  (V<Name>, bool, P<Block>) ;
self.yystack.pop().unwrap();
let yyp3 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,yyp3.minor,) {
 (YYMinorType::YY173(yy1),YYMinorType::YY36(yy3),) => {
 yyres = (yy1.0, yy1.1, P(yy3)); 
},    _ => unreachable!() };
 YYMinorType::YY114(yyres)
}
            ,
            89 /* parlist ::= namelist */
            => 
{
let yyres :  (V<Name>, bool) ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY68(yy0),) => {
 yyres = (yy0, false); 
},    _ => unreachable!() };
 YYMinorType::YY173(yyres)
}
            ,
            90 /* parlist ::= DOTDOTDOT */
            => 
{
let yyres :  (V<Name>, bool) ;
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = (P([]), true); 
} };
 YYMinorType::YY173(yyres)
}
            ,
            91 /* parlist ::= namelist COMMA2 DOTDOTDOT */
            => 
{
let yyres :  (V<Name>, bool) ;
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY68(yy0),) => {
 yyres = (yy0, true); 
},    _ => unreachable!() };
 YYMinorType::YY173(yyres)
}
            ,
            92 /* tableconstructor ::= OPENBRACE CLOSEBRACE */
            => 
{
let yyres :  V<Field> ;
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = P([]); 
} };
 YYMinorType::YY147(yyres)
}
            ,
            93 /* tableconstructor ::= OPENBRACE fieldlist CLOSEBRACE */
            => 
{
let yyres :  V<Field> ;
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY147(yy1),) => {
 yyres = yy1; 
},    _ => unreachable!() };
 YYMinorType::YY147(yyres)
}
            ,
            94 /* fieldlist ::= fieldlist_ */
            => 
{
let yyres :  V<Field> ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY102(yy0),) => {
 yyres = yy0.into_boxed_slice(); 
},    _ => unreachable!() };
 YYMinorType::YY147(yyres)
}
            ,
            95 /* fieldlist ::= fieldlist_ fieldsep */
            => 
{
let yyres :  V<Field> ;
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY102(yy0),) => {
 yyres = yy0.into_boxed_slice(); 
},    _ => unreachable!() };
 YYMinorType::YY147(yyres)
}
            ,
            96 /* fieldlist_ ::= field */
            => 
{
let yyres :  Vec<Field> ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY174(yy0),) => {
 yyres = vec![yy0]; 
},    _ => unreachable!() };
 YYMinorType::YY102(yyres)
}
            ,
            97 /* fieldlist_ ::= fieldlist_ fieldsep field */
            => 
{
let yyres :  Vec<Field> ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY102(yy0),YYMinorType::YY174(yy2),) => {
 yyres = { let mut v = yy0; v.push(yy2); v }; 
},    _ => unreachable!() };
 YYMinorType::YY102(yyres)
}
            ,
            98 /* field ::= exp */
            => 
{
let yyres :  Field ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY78(yy0),) => {
 yyres = FieldAuto(P(yy0)); 
},    _ => unreachable!() };
 YYMinorType::YY174(yyres)
}
            ,
            99 /* field ::= name EQ exp */
            => 
{
let yyres :  Field ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY120(yy0),YYMinorType::YY78(yy2),) => {
 yyres = FieldNamed(yy0, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY174(yyres)
}
            ,
            100 /* field ::= OPENBRACKET exp CLOSEBRACKET EQ exp */
            => 
{
let yyres :  Field ;
let yyp4 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,yyp4.minor,) {
 (YYMinorType::YY78(yy1),YYMinorType::YY78(yy4),) => {
 yyres = FieldExpr(P(yy1), P(yy4)); 
},    _ => unreachable!() };
 YYMinorType::YY174(yyres)
}
            ,
            101 /* fieldsep ::= COMMA|SEMI */
            => 
{
self.yystack.pop().unwrap();
match () {
 () => {


} };
 YYMinorType::YY0
}
            ,
            102 /* name ::= NAME */
            => 
{
let yyres :  Name ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY40(yy0),) => {
 yyres = Name::new(yy0); 
},    _ => unreachable!() };
 YYMinorType::YY120(yyres)
}
            ,
            _ => unreachable!(),
        };
        let yygoto = YY_RULE_INFO[yyruleno as usize] as i32;
        let yyact = self.find_reduce_action(yygoto);
        if yyact < YYNSTATE {
            self.yy_shift(yyact, yygoto, yygotominor);
        } else {
            assert!(yyact == YYNSTATE + YYNRULE + 1);
            self.yy_accept();
        }
    }

    fn yy_parse_failed(&mut self) {
        self.yystack.clear();
 println!("giving up... failed!"); 
    }

    fn yy_syntax_error(&mut self, token: &Token) {

    let token: ::token::Token = token.clone().into();
    println!("syntax error, unexpected token {:?}", token);
    }

    fn yy_accept(&mut self) {
        self.yystack.clear();
 println!("accepted!"); 
    }
}

