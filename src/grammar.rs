#![allow(dead_code)]
#![allow(unused_variables)]
/* TMPL: %include */
 use ::ast::*; 
use std::collections::HashSet;
/* TMPL: makeheader cruft */


/* TMPL: types */

type YYCODETYPE = i8;
const YYNOCODE: i32 = 92;
type YYACTIONTYPE = u16;
const YYWILDCARD: YYCODETYPE = 0;
enum YYMinorType {
    YY0,
    YY12(Vec<(Expr, Block)>),
    YY14(Vec<Var>),
    YY22(Option<Stmt>),
    YY30(V<Expr>),
    YY35(Stmt),
    YY38(Var),
    YY41(Vec<Field>),
    YY53(f64),
    YY55(Option<P<Block>>),
    YY66(String),
    YY71((V<Name>, bool)),
    YY74(Vec<Expr>),
    YY76(Block),
    YY86(Args),
    YY95(Call),
    YY99((V<Name>, Option<Name>)),
    YY104(V<Name>),
    YY122(V<Var>),
    YY130(Expr),
    YY132((V<Name>, bool, P<Block>)),
    YY135(Vec<Stmt>),
    YY147(Field),
    YY148(Vec<Name>),
    YY152(Name),
    YY157(V<Field>),
}
const YYNSTATE: i32 = 210;
const YYNRULE: i32 = 107;
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
        Token::NAME(x) => YYMinorType::YY66(x),
        Token::NUMBER(x) => YYMinorType::YY53(x),
        Token::STRING(x) => YYMinorType::YY66(x),
        _ => YYMinorType::YY0
  }
}
const YY_ACTTAB_COUNT: i32 = 996;
const YY_ACTION: [YYACTIONTYPE; 996] = [
 /*     0 */     9,   24,   23,   22,  137,   82,   21,    2,   47,  202,
 /*    10 */    38,   36,   34,   31,   32,   30,   28,   29,   27,   26,
 /*    20 */    25,   24,   23,   22,   65,   64,   21,   38,   36,   34,
 /*    30 */    31,   32,   30,   28,   29,   27,   26,   25,   24,   23,
 /*    40 */    22,   12,   75,   21,   38,   36,   34,   31,   32,   30,
 /*    50 */    28,   29,   27,   26,   25,   24,   23,   22,  135,   82,
 /*    60 */    21,    2,  171,  174,  210,  190,  130,   38,   36,   34,
 /*    70 */    31,   32,   30,   28,   29,   27,   26,   25,   24,   23,
 /*    80 */    22,   52,  187,   21,   36,   34,   31,   32,   30,   28,
 /*    90 */    29,   27,   26,   25,   24,   23,   22,    7,   59,   21,
 /*   100 */    38,   36,   34,   31,   32,   30,   28,   29,   27,   26,
 /*   110 */    25,   24,   23,   22,  127,   82,   21,    2,   21,   78,
 /*   120 */    27,   26,   25,   24,   23,   22,  252,   70,   21,  171,
 /*   130 */   175,  252,   38,   36,   34,   31,   32,   30,   28,   29,
 /*   140 */    27,   26,   25,   24,   23,   22,   54,    4,   21,  117,
 /*   150 */   251,   76,  170,  123,  120,  251,   10,  185,  318,  147,
 /*   160 */    82,  138,    2,  184,   38,   36,   34,   31,   32,   30,
 /*   170 */    28,   29,   27,   26,   25,   24,   23,   22,  178,  133,
 /*   180 */    21,   46,  178,   38,   36,   34,   31,   32,   30,   28,
 /*   190 */    29,   27,   26,   25,   24,   23,   22,   19,    1,   21,
 /*   200 */    38,   36,   34,   31,   32,   30,   28,   29,   27,   26,
 /*   210 */    25,   24,   23,   22,  183,  180,   21,   38,   36,   34,
 /*   220 */    31,   32,   30,   28,   29,   27,   26,   25,   24,   23,
 /*   230 */    22,   20,   17,   21,  172,  187,  197,  196,   34,   31,
 /*   240 */    32,   30,   28,   29,   27,   26,   25,   24,   23,   22,
 /*   250 */   186,   71,   21,  186,  191,  116,  203,   16,  134,   18,
 /*   260 */   126,   82,  138,    2,  153,  138,  111,   56,   72,   20,
 /*   270 */   136,   33,  166,  187,  165,   37,   35,  164,  201,  200,
 /*   280 */   199,  198,   20,    1,  181,   14,  187,  197,  196,  125,
 /*   290 */    82,  151,    2,   11,    6,   77,   82,  208,    2,  207,
 /*   300 */   205,  146,   71,  107,  169,  203,  163,   82,   45,    2,
 /*   310 */    18,  155,  161,  160,  113,  110,   56,   73,  291,  112,
 /*   320 */   159,  316,   33,  291,    5,  149,   37,   35,  148,  201,
 /*   330 */   200,  199,  198,   20,    1,   41,  209,  187,  197,  196,
 /*   340 */   187,   53,  193,  291,  291,  291,  162,   82,   15,    2,
 /*   350 */    51,   60,   50,   71,  191,   20,  203,   91,  189,  187,
 /*   360 */   197,  196,  150,    4,   40,   49,  192,   57,  170,  291,
 /*   370 */   195,  194,   48,   33,  188,   71,   42,   37,   35,  319,
 /*   380 */   201,  200,  199,  198,  173,    1,  182,   55,   68,   69,
 /*   390 */    39,  122,   82,  187,    2,   33,  176,   58,   66,   37,
 /*   400 */    35,   63,  201,  200,  199,  198,   20,    1,    8,  167,
 /*   410 */   187,  319,  157,  206,    1,  168,   67,   81,  158,   80,
 /*   420 */    79,  156,   13,  154,   74,  114,   44,   43,   20,  193,
 /*   430 */     3,   62,  187,  197,  196,  121,   82,  124,    2,  118,
 /*   440 */    82,  191,    2,  132,   96,  319,  319,  319,   71,  319,
 /*   450 */   319,  319,  319,  192,   57,  319,  319,  195,  194,  319,
 /*   460 */   319,  131,   61,  319,  177,  319,  319,  319,   33,  319,
 /*   470 */   319,  193,   37,   35,  319,  201,  200,  199,  198,  319,
 /*   480 */     1,  319,  204,  191,  319,  203,   93,  319,  319,  319,
 /*   490 */   193,  319,  319,  319,  319,  192,   57,  193,  129,  195,
 /*   500 */   194,  119,  191,  319,  203,   93,  319,  319,  152,  191,
 /*   510 */   319,  203,   93,  319,  192,   57,  319,  129,  195,  194,
 /*   520 */   193,  192,   57,  319,  129,  195,  194,  193,  319,  319,
 /*   530 */   319,  128,  191,  319,  203,   93,  319,  319,  115,  191,
 /*   540 */   319,  203,   93,  319,  192,   57,  193,  129,  195,  194,
 /*   550 */   319,  192,   57,  319,  129,  195,  194,  319,  191,  319,
 /*   560 */   132,   96,  319,  319,  193,  319,  319,  319,  319,  319,
 /*   570 */   192,   57,  319,  319,  195,  194,  191,  319,  203,   98,
 /*   580 */   319,  179,  193,  319,  319,  319,  319,  319,  192,   57,
 /*   590 */   193,  319,  195,  194,  191,  319,  203,  145,  319,  319,
 /*   600 */   319,  319,  191,  319,  203,   99,  192,   57,  193,  319,
 /*   610 */   195,  194,  319,  319,  192,   57,  193,  319,  195,  194,
 /*   620 */   191,  319,  203,  144,  319,  319,  319,  319,  191,  319,
 /*   630 */   203,  100,  192,   57,  193,  319,  195,  194,  319,  319,
 /*   640 */   192,   57,  319,  319,  195,  194,  191,  319,  203,  143,
 /*   650 */   319,  319,  193,  319,  319,  319,  319,  319,  192,   57,
 /*   660 */   193,  319,  195,  194,  191,  319,  203,  106,  319,  319,
 /*   670 */   319,  319,  191,  319,  203,  105,  192,   57,  193,  319,
 /*   680 */   195,  194,  319,  319,  192,   57,  193,  319,  195,  194,
 /*   690 */   191,  319,  203,  104,  319,  319,  319,  319,  191,  319,
 /*   700 */   203,  103,  192,   57,  193,  319,  195,  194,  319,  319,
 /*   710 */   192,   57,  319,  319,  195,  194,  191,  319,  203,  102,
 /*   720 */   319,  319,  193,  319,  319,  319,  319,  319,  192,   57,
 /*   730 */   193,  319,  195,  194,  191,  319,  203,  101,  319,  319,
 /*   740 */   319,  319,  191,  319,  203,  109,  192,   57,  193,  319,
 /*   750 */   195,  194,  319,  319,  192,   57,  193,  319,  195,  194,
 /*   760 */   191,  319,  203,  108,  319,  319,  319,  319,  191,  319,
 /*   770 */   203,  142,  192,   57,  193,  319,  195,  194,  319,  319,
 /*   780 */   192,   57,  319,  319,  195,  194,  191,  319,  203,  141,
 /*   790 */   319,  319,  193,  319,  319,  319,  319,  319,  192,   57,
 /*   800 */   193,  319,  195,  194,  191,  319,  203,  140,  319,  319,
 /*   810 */   319,  319,  191,  319,  203,  139,  192,   57,  193,  319,
 /*   820 */   195,  194,  319,  319,  192,   57,  193,  319,  195,  194,
 /*   830 */   191,  319,  203,   90,  319,  319,  319,  319,  191,  319,
 /*   840 */   203,   97,  192,   57,  193,  319,  195,  194,  319,  319,
 /*   850 */   192,   57,  319,  319,  195,  194,  191,  319,  203,   89,
 /*   860 */   319,  319,  193,  319,  319,  319,  319,  319,  192,   57,
 /*   870 */   193,  319,  195,  194,  191,  319,  203,   95,  319,  319,
 /*   880 */   319,  319,  191,  319,  203,   94,  192,   57,  193,  319,
 /*   890 */   195,  194,  319,  319,  192,   57,  193,  319,  195,  194,
 /*   900 */   191,  319,  203,   88,  319,  319,  319,  319,  191,  319,
 /*   910 */   203,   92,  192,   57,  193,  319,  195,  194,  319,  319,
 /*   920 */   192,   57,  319,  319,  195,  194,  191,  319,  203,   87,
 /*   930 */   319,  319,  193,  319,  319,  319,  319,  319,  192,   57,
 /*   940 */   193,  319,  195,  194,  191,  319,  203,   86,  319,  319,
 /*   950 */   319,  319,  191,  319,  203,   85,  192,   57,  193,  319,
 /*   960 */   195,  194,  319,  319,  192,   57,  193,  319,  195,  194,
 /*   970 */   191,  319,  203,   83,  319,  319,  319,  319,  191,  319,
 /*   980 */   203,   84,  192,   57,  319,  319,  195,  194,  319,  319,
 /*   990 */   192,   57,  319,  319,  195,  194,
];
const YY_LOOKAHEAD: [YYCODETYPE; 996] = [
 /*     0 */     4,   42,   43,   44,   57,   58,   47,   60,   12,   30,
 /*    10 */    31,   32,   33,   34,   35,   36,   37,   38,   39,   40,
 /*    20 */    41,   42,   43,   44,   27,   28,   47,   31,   32,   33,
 /*    30 */    34,   35,   36,   37,   38,   39,   40,   41,   42,   43,
 /*    40 */    44,   23,   24,   47,   31,   32,   33,   34,   35,   36,
 /*    50 */    37,   38,   39,   40,   41,   42,   43,   44,   57,   58,
 /*    60 */    47,   60,   84,   85,    0,   52,   30,   31,   32,   33,
 /*    70 */    34,   35,   36,   37,   38,   39,   40,   41,   42,   43,
 /*    80 */    44,   12,    5,   47,   32,   33,   34,   35,   36,   37,
 /*    90 */    38,   39,   40,   41,   42,   43,   44,    9,   21,   47,
 /*   100 */    31,   32,   33,   34,   35,   36,   37,   38,   39,   40,
 /*   110 */    41,   42,   43,   44,   57,   58,   47,   60,   47,   18,
 /*   120 */    39,   40,   41,   42,   43,   44,    4,    4,   47,   84,
 /*   130 */    85,    9,   31,   32,   33,   34,   35,   36,   37,   38,
 /*   140 */    39,   40,   41,   42,   43,   44,   52,    2,   47,   69,
 /*   150 */     4,   18,    7,   73,   74,    9,    4,   13,   56,   57,
 /*   160 */    58,   81,   60,   13,   31,   32,   33,   34,   35,   36,
 /*   170 */    37,   38,   39,   40,   41,   42,   43,   44,    4,    3,
 /*   180 */    47,   12,    8,   31,   32,   33,   34,   35,   36,   37,
 /*   190 */    38,   39,   40,   41,   42,   43,   44,    9,   53,   47,
 /*   200 */    31,   32,   33,   34,   35,   36,   37,   38,   39,   40,
 /*   210 */    41,   42,   43,   44,   51,   54,   47,   31,   32,   33,
 /*   220 */    34,   35,   36,   37,   38,   39,   40,   41,   42,   43,
 /*   230 */    44,    1,    9,   47,   52,    5,    6,    7,   33,   34,
 /*   240 */    35,   36,   37,   38,   39,   40,   41,   42,   43,   44,
 /*   250 */    69,   21,   47,   69,   67,   74,   69,    4,   74,   29,
 /*   260 */    57,   58,   81,   60,   69,   81,   79,   80,   73,    1,
 /*   270 */    86,   41,   13,    5,   13,   45,   46,   13,   48,   49,
 /*   280 */    50,   51,    1,   53,   54,   16,    5,    6,    7,   57,
 /*   290 */    58,   55,   60,    9,   20,   57,   58,   61,   60,   63,
 /*   300 */    64,   65,   21,   67,   68,   69,   57,   58,   12,   60,
 /*   310 */    29,   69,   13,   13,   78,   79,   80,   75,    2,   77,
 /*   320 */    13,    9,   41,    7,    9,    8,   45,   46,   26,   48,
 /*   330 */    49,   50,   51,    1,   53,    4,   59,    5,    6,    7,
 /*   340 */     5,   62,   55,   27,   28,   29,   57,   58,   62,   60,
 /*   350 */    62,   71,   62,   21,   67,    1,   69,   70,   76,    5,
 /*   360 */     6,    7,    8,    2,    2,   62,   79,   80,    7,   53,
 /*   370 */    83,   84,   62,   41,   69,   21,   62,   45,   46,   91,
 /*   380 */    48,   49,   50,   51,   52,   53,   51,   52,   27,   28,
 /*   390 */    29,   57,   58,    5,   60,   41,   69,   69,   62,   45,
 /*   400 */    46,   62,   48,   49,   50,   51,    1,   53,   89,   69,
 /*   410 */     5,   91,   69,    8,   53,   10,   11,   12,   76,   14,
 /*   420 */    15,   69,   17,   76,   19,   69,   21,   22,    1,   55,
 /*   430 */    25,   26,    5,    6,    7,   57,   58,   72,   60,   57,
 /*   440 */    58,   67,   60,   69,   70,   91,   91,   91,   21,   91,
 /*   450 */    91,   91,   91,   79,   80,   91,   91,   83,   84,   91,
 /*   460 */    91,   87,   88,   91,   90,   91,   91,   91,   41,   91,
 /*   470 */    91,   55,   45,   46,   91,   48,   49,   50,   51,   91,
 /*   480 */    53,   91,   66,   67,   91,   69,   70,   91,   91,   91,
 /*   490 */    55,   91,   91,   91,   91,   79,   80,   55,   82,   83,
 /*   500 */    84,   66,   67,   91,   69,   70,   91,   91,   66,   67,
 /*   510 */    91,   69,   70,   91,   79,   80,   91,   82,   83,   84,
 /*   520 */    55,   79,   80,   91,   82,   83,   84,   55,   91,   91,
 /*   530 */    91,   66,   67,   91,   69,   70,   91,   91,   66,   67,
 /*   540 */    91,   69,   70,   91,   79,   80,   55,   82,   83,   84,
 /*   550 */    91,   79,   80,   91,   82,   83,   84,   91,   67,   91,
 /*   560 */    69,   70,   91,   91,   55,   91,   91,   91,   91,   91,
 /*   570 */    79,   80,   91,   91,   83,   84,   67,   91,   69,   70,
 /*   580 */    91,   90,   55,   91,   91,   91,   91,   91,   79,   80,
 /*   590 */    55,   91,   83,   84,   67,   91,   69,   70,   91,   91,
 /*   600 */    91,   91,   67,   91,   69,   70,   79,   80,   55,   91,
 /*   610 */    83,   84,   91,   91,   79,   80,   55,   91,   83,   84,
 /*   620 */    67,   91,   69,   70,   91,   91,   91,   91,   67,   91,
 /*   630 */    69,   70,   79,   80,   55,   91,   83,   84,   91,   91,
 /*   640 */    79,   80,   91,   91,   83,   84,   67,   91,   69,   70,
 /*   650 */    91,   91,   55,   91,   91,   91,   91,   91,   79,   80,
 /*   660 */    55,   91,   83,   84,   67,   91,   69,   70,   91,   91,
 /*   670 */    91,   91,   67,   91,   69,   70,   79,   80,   55,   91,
 /*   680 */    83,   84,   91,   91,   79,   80,   55,   91,   83,   84,
 /*   690 */    67,   91,   69,   70,   91,   91,   91,   91,   67,   91,
 /*   700 */    69,   70,   79,   80,   55,   91,   83,   84,   91,   91,
 /*   710 */    79,   80,   91,   91,   83,   84,   67,   91,   69,   70,
 /*   720 */    91,   91,   55,   91,   91,   91,   91,   91,   79,   80,
 /*   730 */    55,   91,   83,   84,   67,   91,   69,   70,   91,   91,
 /*   740 */    91,   91,   67,   91,   69,   70,   79,   80,   55,   91,
 /*   750 */    83,   84,   91,   91,   79,   80,   55,   91,   83,   84,
 /*   760 */    67,   91,   69,   70,   91,   91,   91,   91,   67,   91,
 /*   770 */    69,   70,   79,   80,   55,   91,   83,   84,   91,   91,
 /*   780 */    79,   80,   91,   91,   83,   84,   67,   91,   69,   70,
 /*   790 */    91,   91,   55,   91,   91,   91,   91,   91,   79,   80,
 /*   800 */    55,   91,   83,   84,   67,   91,   69,   70,   91,   91,
 /*   810 */    91,   91,   67,   91,   69,   70,   79,   80,   55,   91,
 /*   820 */    83,   84,   91,   91,   79,   80,   55,   91,   83,   84,
 /*   830 */    67,   91,   69,   70,   91,   91,   91,   91,   67,   91,
 /*   840 */    69,   70,   79,   80,   55,   91,   83,   84,   91,   91,
 /*   850 */    79,   80,   91,   91,   83,   84,   67,   91,   69,   70,
 /*   860 */    91,   91,   55,   91,   91,   91,   91,   91,   79,   80,
 /*   870 */    55,   91,   83,   84,   67,   91,   69,   70,   91,   91,
 /*   880 */    91,   91,   67,   91,   69,   70,   79,   80,   55,   91,
 /*   890 */    83,   84,   91,   91,   79,   80,   55,   91,   83,   84,
 /*   900 */    67,   91,   69,   70,   91,   91,   91,   91,   67,   91,
 /*   910 */    69,   70,   79,   80,   55,   91,   83,   84,   91,   91,
 /*   920 */    79,   80,   91,   91,   83,   84,   67,   91,   69,   70,
 /*   930 */    91,   91,   55,   91,   91,   91,   91,   91,   79,   80,
 /*   940 */    55,   91,   83,   84,   67,   91,   69,   70,   91,   91,
 /*   950 */    91,   91,   67,   91,   69,   70,   79,   80,   55,   91,
 /*   960 */    83,   84,   91,   91,   79,   80,   55,   91,   83,   84,
 /*   970 */    67,   91,   69,   70,   91,   91,   91,   91,   67,   91,
 /*   980 */    69,   70,   79,   80,   91,   91,   83,   84,   91,   91,
 /*   990 */    79,   80,   91,   91,   83,   84,
];
const YY_SHIFT_USE_DFLT: i32 = -42;
const YY_SHIFT_COUNT: i32 = 147;
const YY_SHIFT_MIN: i32 = -41;
const YY_SHIFT_MAX: i32 = 427;
const YY_SHIFT_OFST: [i16; 148] = [
 /*     0 */   -42,  230,  405,  354,  332,  427,  427,  427,  281,  427,
 /*    10 */   427,  427,  427,  427,  427,  427,  427,  427,  427,  427,
 /*    20 */   427,  427,  427,  427,  427,  427,  427,  427,  427,  427,
 /*    30 */   427,  427,  427,  427,  427,  427,  427,  427,  427,  427,
 /*    40 */   335,  268,  388,   77,  388,  -42,  -42,  -42,  -42,  -42,
 /*    50 */   -42,  -42,  -42,  -42,  -42,  -42,  361,  361,  145,  388,
 /*    60 */    18,  174,  388,  362,  388,  388,  362,  388,  388,  388,
 /*    70 */   388,  362,  -42,  -42,  -42,  -42,  -42,  -42,  -42,  -42,
 /*    80 */   -42,  -42,  -42,   -4,  169,  152,  133,  101,   69,   36,
 /*    90 */    13,  -21,  186,  186,  186,  186,  186,  186,   52,  205,
 /*   100 */    81,   81,   81,   81,   81,   81,   81,  316,  -41,  -41,
 /*   110 */   146,  122,   -3,  331,  302,  317,  315,  312,  307,  296,
 /*   120 */   274,  300,  299,  284,  264,  269,  261,  259,  182,  253,
 /*   130 */   223,  161,  188,  163,  176,  150,   94,  144,  123,   71,
 /*   140 */    71,   71,   71,   71,   71,   71,   88,   64,
];
const YY_REDUCE_USE_DFLT: i32 = -54;
const YY_REDUCE_COUNT: i32 = 82;
const YY_REDUCE_MIN: i32 = -53;
const YY_REDUCE_MAX: i32 = 911;
const YY_REDUCE_OFST: [i16; 83] = [
 /*     0 */   102,  374,  236,  472,  465,  442,  435,  416,  491,  911,
 /*    10 */   903,  885,  877,  859,  841,  833,  815,  807,  789,  771,
 /*    20 */   763,  745,  737,  719,  701,  693,  675,  667,  649,  631,
 /*    30 */   623,  605,  597,  579,  561,  553,  535,  527,  509,  287,
 /*    40 */   184,  187,   80,  181,  242,  382,  378,  334,  289,  249,
 /*    50 */   238,  232,  203,   57,    1,  -53,   45,   45,  -22,  195,
 /*    60 */   365,  319,  356,  347,  352,  343,  342,  340,  328,  327,
 /*    70 */   305,  282,  339,  336,  314,  310,  303,  280,  290,  288,
 /*    80 */   286,  279,  277,
];
const YY_DEFAULT: [YYACTIONTYPE; 210] = [
 /*     0 */   216,  317,  212,  241,  317,  317,  317,  317,  308,  317,
 /*    10 */   317,  317,  317,  317,  317,  317,  317,  317,  317,  317,
 /*    20 */   317,  317,  317,  317,  317,  317,  317,  317,  317,  317,
 /*    30 */   317,  317,  317,  317,  317,  317,  317,  317,  317,  317,
 /*    40 */   317,  317,  317,  317,  317,  216,  216,  216,  216,  216,
 /*    50 */   216,  216,  216,  216,  216,  216,  317,  269,  317,  317,
 /*    60 */   239,  307,  317,  317,  317,  317,  317,  317,  317,  317,
 /*    70 */   317,  317,  214,  214,  214,  214,  214,  237,  214,  214,
 /*    80 */   214,  214,  215,  317,  317,  317,  317,  317,  317,  317,
 /*    90 */   317,  317,  227,  260,  261,  313,  311,  312,  274,  275,
 /*   100 */   276,  282,  281,  280,  279,  278,  277,  221,  284,  283,
 /*   110 */   290,  290,  246,  250,  317,  243,  234,  257,  317,  317,
 /*   120 */   317,  317,  317,  317,  317,  317,  317,  317,  317,  259,
 /*   130 */   317,  317,  253,  317,  302,  317,  317,  317,  256,  288,
 /*   140 */   287,  286,  285,  273,  272,  271,  317,  317,  245,  244,
 /*   150 */   242,  236,  235,  316,  233,  248,  249,  247,  232,  231,
 /*   160 */   230,  229,  240,  238,  228,  226,  225,  224,  223,  222,
 /*   170 */   298,  297,  296,  295,  294,  293,  255,  309,  314,  310,
 /*   180 */   306,  305,  303,  304,  301,  300,  257,  315,  258,  299,
 /*   190 */   292,  291,  290,  289,  270,  268,  267,  266,  265,  264,
 /*   200 */   263,  262,  254,  253,  220,  219,  218,  217,  213,  211,
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

const YY_RULE_INFO: [YYCODETYPE; 107] = [
  56,
  57,
  58,
  58,
  62,
  59,
  60,
  60,
  63,
  63,
  64,
  64,
  64,
  64,
  64,
  64,
  64,
  64,
  64,
  64,
  64,
  64,
  64,
  64,
  64,
  64,
  64,
  71,
  71,
  72,
  72,
  61,
  61,
  61,
  61,
  68,
  75,
  75,
  77,
  77,
  65,
  78,
  78,
  79,
  79,
  79,
  74,
  81,
  81,
  66,
  82,
  82,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  70,
  80,
  80,
  80,
  67,
  67,
  85,
  85,
  85,
  85,
  83,
  76,
  76,
  86,
  86,
  86,
  84,
  84,
  87,
  87,
  88,
  88,
  90,
  90,
  90,
  89,
  69,
  73,
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
    last_error: Option<Token>,
    last_error_count: u8,
    soft_error_count: u8,
    recoverable: bool,
    accepted: bool,
    scopes: Vec<HashSet<Box<str>>>,
}

impl Parser {

    #[inline]
    pub fn new(
        ) -> Parser {
        let mut p = Parser {
            yyerrcnt: -1,
            yystack: Vec::new(),
            //extra: extra,
            chunk: None,
            last_error: None,
            last_error_count: 0,
            soft_error_count: 0,
            recoverable: true,
            accepted: false,
            scopes: vec![HashSet::new()],
        };
        p.yystack.push(YYStackEntry{stateno: 0, major: 0, minor: YYMinorType::YY0});
        p
    }


    #[inline]
    pub fn into_chunk(self) -> Option<Chunk> {
        self.chunk
    }

    #[inline]
    pub fn is_recoverable(&self) -> bool {
        self.recoverable
    }

    #[inline]
    pub fn is_accepted(&self) -> bool {
        self.accepted
    }

    #[inline]
    pub fn last_token_error(&self) -> Option<Token> {
        self.last_error.clone()
    }

    #[inline]
    pub fn error_count(&self) -> u8 {
        self.last_error_count
    }

    #[inline]
    pub fn error_count_soft(&self) -> u8 {
        self.soft_error_count
    }

    fn current_scope(&mut self) -> &mut HashSet<Box<str>> {
        self.scopes.last_mut().unwrap()
    }

    fn top_scope(&mut self) -> &mut HashSet<Box<str>> {
        self.scopes.first_mut().unwrap()
    }

    fn insert_local_name(&mut self, name: &::ast::Name) -> bool {
        let n = name.name.clone();
        {
            let scope = self.current_scope();
            if !scope.contains(&n) { return scope.insert(n) }
            println!("Redeclaration of variable `{}`", n);
        }
        self.soft_error_count += 1;
        false
    }

    fn insert_global_name(&mut self, name: &::ast::Name) -> bool {
        let n = name.name.clone();
        {
            let scope = self.top_scope();
            if !scope.contains(&n) { return scope.insert(n) }
            println!("Redeclaration of variable `{}`", n);
        }
        self.soft_error_count += 1;
        false
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashSet::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn is_in_scope(&mut self, name: &::ast::Name) -> bool {
        let n = name.name.clone();
        for scope in self.scopes.iter() {
            if scope.contains(&n) { return true }
        }
        println!("Use of undeclared variable `{}`", n);
        self.soft_error_count += 1;
        false
    }

    #[inline]
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
 (YYMinorType::YY76(yy0),) => {
 self.chunk = Some(Chunk { block: yy0 }) 
},    _ => unreachable!() };
 YYMinorType::YY0
}
            ,
            1 /* block ::= block_ pop_scope_ */
            => 
{
let yyres :  Block ;
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY76(yy0),) => {
 yyres = yy0; 
},    _ => unreachable!() };
 YYMinorType::YY76(yyres)
}
            ,
            2 /* block_ ::= stats_ */
            => 
{
let yyres :  Block ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY135(yy0),) => {
 yyres = Block { stmts: yy0.into_boxed_slice(), ret: None }; 
},    _ => unreachable!() };
 YYMinorType::YY76(yyres)
}
            ,
            3 /* block_ ::= stats_ retstat */
            => 
{
let yyres :  Block ;
let yyp1 = self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp1.minor,) {
 (YYMinorType::YY135(yy0),YYMinorType::YY30(yy1),) => {
 yyres = Block { stmts: yy0.into_boxed_slice(), ret: Some(yy1) }; 
},    _ => unreachable!() };
 YYMinorType::YY76(yyres)
}
            ,
            4 /* push_scope_ ::= */
            => 
{
match () {
 () => {
 self.push_scope(); 
} };
 YYMinorType::YY0
}
            ,
            5 /* pop_scope_ ::= */
            => 
{
match () {
 () => {
 self.pop_scope(); 
} };
 YYMinorType::YY0
}
            ,
            6 /* stats_ ::= */
            => 
{
let yyres :  Vec<Stmt> ;
match () {
 () => {
 yyres = vec![]; 
} };
 YYMinorType::YY135(yyres)
}
            ,
            7 /* stats_ ::= stats_ stat_ */
            => 
{
let yyres :  Vec<Stmt> ;
let yyp1 = self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp1.minor,) {
 (YYMinorType::YY135(yy0),YYMinorType::YY22(yy1),) => {
 yyres = if let Some(s) = yy1 { let mut v = yy0; v.push(s); v } else { yy0 } 
},    _ => unreachable!() };
 YYMinorType::YY135(yyres)
}
            ,
            8 /* stat_ ::= SEMI */
            => 
{
let yyres :  Option<Stmt> ;
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = None; 
} };
 YYMinorType::YY22(yyres)
}
            ,
            9 /* stat_ ::= stat */
            => 
{
let yyres :  Option<Stmt> ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY35(yy0),) => {
 yyres = Some(yy0); 
},    _ => unreachable!() };
 YYMinorType::YY22(yyres)
}
            ,
            10 /* stat ::= varlist EQ explist */
            => 
{
let yyres :  Stmt ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY122(yy0),YYMinorType::YY30(yy2),) => {
 yyres = StmtSet(yy0, yy2); 
},    _ => unreachable!() };
 YYMinorType::YY35(yyres)
}
            ,
            11 /* stat ::= functioncall */
            => 
{
let yyres :  Stmt ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY95(yy0),) => {
 yyres = StmtCall(P(yy0)); 
},    _ => unreachable!() };
 YYMinorType::YY35(yyres)
}
            ,
            12 /* stat ::= label */
            => 
{
let yyres :  Stmt ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY152(yy0),) => {
 yyres = StmtLabel(yy0); 
},    _ => unreachable!() };
 YYMinorType::YY35(yyres)
}
            ,
            13 /* stat ::= BREAK */
            => 
{
let yyres :  Stmt ;
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = StmtBreak; 
} };
 YYMinorType::YY35(yyres)
}
            ,
            14 /* stat ::= GOTO name */
            => 
{
let yyres :  Stmt ;
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY152(yy1),) => {
 yyres = StmtGoto(yy1); 
},    _ => unreachable!() };
 YYMinorType::YY35(yyres)
}
            ,
            15 /* stat ::= DO push_scope_ block END */
            => 
{
let yyres :  Stmt ;
self.yystack.pop().unwrap();
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp2.minor,) {
 (YYMinorType::YY76(yy2),) => {
 yyres = StmtDo(P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY35(yyres)
}
            ,
            16 /* stat ::= WHILE push_scope_ exp DO block END */
            => 
{
let yyres :  Stmt ;
self.yystack.pop().unwrap();
let yyp4 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp2.minor,yyp4.minor,) {
 (YYMinorType::YY130(yy2),YYMinorType::YY76(yy4),) => {
 yyres = StmtWhile(P(yy2), P(yy4)); 
},    _ => unreachable!() };
 YYMinorType::YY35(yyres)
}
            ,
            17 /* stat ::= REPEAT push_scope_ block UNTIL exp */
            => 
{
let yyres :  Stmt ;
let yyp4 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp2.minor,yyp4.minor,) {
 (YYMinorType::YY76(yy2),YYMinorType::YY130(yy4),) => {
 yyres = StmtRepeat(P(yy2), P(yy4)); 
},    _ => unreachable!() };
 YYMinorType::YY35(yyres)
}
            ,
            18 /* stat ::= IF exp THEN push_scope_ block elsifs_ else_ END */
            => 
{
let yyres :  Stmt ;
self.yystack.pop().unwrap();
let yyp6 = self.yystack.pop().unwrap();
let yyp5 = self.yystack.pop().unwrap();
let yyp4 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,yyp4.minor,yyp5.minor,yyp6.minor,) {
 (YYMinorType::YY130(yy1),YYMinorType::YY76(yy4),YYMinorType::YY12(yy5),YYMinorType::YY55(yy6),) => {
 yyres = StmtIf(P(yy1), P(yy4), yy5.into_boxed_slice(), yy6); 
},    _ => unreachable!() };
 YYMinorType::YY35(yyres)
}
            ,
            19 /* stat ::= FOR push_scope_ declname_ EQ exp COMMA exp DO block END */
            => 
{
let yyres :  Stmt ;
self.yystack.pop().unwrap();
let yyp8 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp6 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp4 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp2.minor,yyp4.minor,yyp6.minor,yyp8.minor,) {
 (YYMinorType::YY152(yy2),YYMinorType::YY130(yy4),YYMinorType::YY130(yy6),YYMinorType::YY76(yy8),) => {
 yyres = StmtForNum(yy2, P(yy4), P(yy6), None, P(yy8)); 
},    _ => unreachable!() };
 YYMinorType::YY35(yyres)
}
            ,
            20 /* stat ::= FOR push_scope_ declname_ EQ exp COMMA exp COMMA exp DO block END */
            => 
{
let yyres :  Stmt ;
self.yystack.pop().unwrap();
let yyp10 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp8 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp6 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp4 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp2.minor,yyp4.minor,yyp6.minor,yyp8.minor,yyp10.minor,) {
 (YYMinorType::YY152(yy2),YYMinorType::YY130(yy4),YYMinorType::YY130(yy6),YYMinorType::YY130(yy8),YYMinorType::YY76(yy10),) => {
 yyres = StmtForNum(yy2, P(yy4), P(yy6), Some(P(yy8)), P(yy10)); 
},    _ => unreachable!() };
 YYMinorType::YY35(yyres)
}
            ,
            21 /* stat ::= FOR push_scope_ namelist IN explist DO block END */
            => 
{
let yyres :  Stmt ;
self.yystack.pop().unwrap();
let yyp6 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp4 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp2.minor,yyp4.minor,yyp6.minor,) {
 (YYMinorType::YY104(yy2),YYMinorType::YY30(yy4),YYMinorType::YY76(yy6),) => {
 yyres = StmtForIn(yy2, yy4, P(yy6)); 
},    _ => unreachable!() };
 YYMinorType::YY35(yyres)
}
            ,
            22 /* stat ::= FUNCTION funcname push_scope_ funcbody */
            => 
{
let yyres :  Stmt ;
let yyp3 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,yyp3.minor,) {
 (YYMinorType::YY99(yy1),YYMinorType::YY132(yy3),) => {
 self.insert_global_name(&yy1.0[0]); yyres = StmtFunction(yy1.0, yy1.1, yy3.0, yy3.1, yy3.2); 
},    _ => unreachable!() };
 YYMinorType::YY35(yyres)
}
            ,
            23 /* stat ::= LOCAL FUNCTION declname_ push_scope_ funcbody */
            => 
{
let yyres :  Stmt ;
let yyp4 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp2.minor,yyp4.minor,) {
 (YYMinorType::YY152(yy2),YYMinorType::YY132(yy4),) => {
 yyres = StmtLocalFunction(yy2, yy4.0, yy4.1, yy4.2); 
},    _ => unreachable!() };
 YYMinorType::YY35(yyres)
}
            ,
            24 /* stat ::= LOCAL namelist */
            => 
{
let yyres :  Stmt ;
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY104(yy1),) => {
 yyres = StmtLocal(yy1, None); 
},    _ => unreachable!() };
 YYMinorType::YY35(yyres)
}
            ,
            25 /* stat ::= LOCAL namelist EQ explist */
            => 
{
let yyres :  Stmt ;
let yyp3 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,yyp3.minor,) {
 (YYMinorType::YY104(yy1),YYMinorType::YY30(yy3),) => {
 yyres = StmtLocal(yy1, Some(yy3)); 
},    _ => unreachable!() };
 YYMinorType::YY35(yyres)
}
            ,
            26 /* stat ::= error */
            => 
{
let yyres :  Stmt ;
let yyp0 = self.yystack.pop().unwrap();
match () {
 () => {
 yyres = StmtInvalid 
} };
 YYMinorType::YY35(yyres)
}
            ,
            27 /* elsifs_ ::= */
            => 
{
let yyres :  Vec<(Expr, Block)> ;
match () {
 () => {
 yyres = vec![]; 
} };
 YYMinorType::YY12(yyres)
}
            ,
            28 /* elsifs_ ::= elsifs_ ELSEIF exp THEN push_scope_ block */
            => 
{
let yyres :  Vec<(Expr, Block)> ;
let yyp5 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,yyp5.minor,) {
 (YYMinorType::YY12(yy0),YYMinorType::YY130(yy2),YYMinorType::YY76(yy5),) => {
 yyres = { let mut v = yy0; v.push((yy2, yy5)); v }; 
},    _ => unreachable!() };
 YYMinorType::YY12(yyres)
}
            ,
            29 /* else_ ::= */
            => 
{
let yyres :  Option<P<Block>> ;
match () {
 () => {
 yyres = None; 
} };
 YYMinorType::YY55(yyres)
}
            ,
            30 /* else_ ::= ELSE push_scope_ block */
            => 
{
let yyres :  Option<P<Block>> ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp2.minor,) {
 (YYMinorType::YY76(yy2),) => {
 yyres = Some(P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY55(yyres)
}
            ,
            31 /* retstat ::= RETURN */
            => 
{
let yyres :  V<Expr> ;
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = P([]); 
} };
 YYMinorType::YY30(yyres)
}
            ,
            32 /* retstat ::= RETURN SEMI */
            => 
{
let yyres :  V<Expr> ;
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = P([]); 
} };
 YYMinorType::YY30(yyres)
}
            ,
            33 /* retstat ::= RETURN explist */
            => 
{
let yyres :  V<Expr> ;
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY30(yy1),) => {
 yyres = yy1; 
},    _ => unreachable!() };
 YYMinorType::YY30(yyres)
}
            ,
            34 /* retstat ::= RETURN explist SEMI */
            => 
{
let yyres :  V<Expr> ;
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY30(yy1),) => {
 yyres = yy1; 
},    _ => unreachable!() };
 YYMinorType::YY30(yyres)
}
            ,
            35 /* label ::= COLONCOLON name COLONCOLON */
            => 
{
let yyres :  Name ;
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY152(yy1),) => {
 yyres = yy1; 
},    _ => unreachable!() };
 YYMinorType::YY152(yyres)
}
            ,
            36 /* funcname ::= dotnames_ */
            => 
{
let yyres :  (V<Name>, Option<Name>) ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY148(yy0),) => {
 yyres = (yy0.into_boxed_slice(), None); 
},    _ => unreachable!() };
 YYMinorType::YY99(yyres)
}
            ,
            37 /* funcname ::= dotnames_ COLON name */
            => 
{
let yyres :  (V<Name>, Option<Name>) ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY148(yy0),YYMinorType::YY152(yy2),) => {
 yyres = (yy0.into_boxed_slice(), Some(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY99(yyres)
}
            ,
            38 /* dotnames_ ::= name */
          | 47 /* namelist_ ::= name */
            => 
{
let yyres :  Vec<Name> ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY152(yy0),) => {
 yyres = vec![yy0]; 
},    _ => unreachable!() };
 YYMinorType::YY148(yyres)
}
            ,
            39 /* dotnames_ ::= dotnames_ DOT name */
            => 
{
let yyres :  Vec<Name> ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY148(yy0),YYMinorType::YY152(yy2),) => {
 yyres = { let mut v = yy0; v.push(yy2); v }; 
},    _ => unreachable!() };
 YYMinorType::YY148(yyres)
}
            ,
            40 /* varlist ::= varlist_ */
            => 
{
let yyres :  V<Var> ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY14(yy0),) => {
 yyres = yy0.into_boxed_slice(); 
},    _ => unreachable!() };
 YYMinorType::YY122(yyres)
}
            ,
            41 /* varlist_ ::= var */
            => 
{
let yyres :  Vec<Var> ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY38(yy0),) => {
 yyres = vec![yy0]; 
},    _ => unreachable!() };
 YYMinorType::YY14(yyres)
}
            ,
            42 /* varlist_ ::= varlist_ COMMA var */
            => 
{
let yyres :  Vec<Var> ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY14(yy0),YYMinorType::YY38(yy2),) => {
 yyres = { let mut v = yy0; v.push(yy2); v } 
},    _ => unreachable!() };
 YYMinorType::YY14(yyres)
}
            ,
            43 /* var ::= name */
            => 
{
let yyres :  Var ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY152(yy0),) => {
 self.is_in_scope(&yy0); yyres = VarName(yy0); 
},    _ => unreachable!() };
 YYMinorType::YY38(yyres)
}
            ,
            44 /* var ::= prefixexp OPENBRACKET exp CLOSEBRACKET */
            => 
{
let yyres :  Var ;
self.yystack.pop().unwrap();
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY130(yy0),YYMinorType::YY130(yy2),) => {
 yyres = VarProperty(P(yy0), P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY38(yyres)
}
            ,
            45 /* var ::= prefixexp DOT name */
            => 
{
let yyres :  Var ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY130(yy0),YYMinorType::YY152(yy2),) => {
 yyres = VarMember(P(yy0), yy2); 
},    _ => unreachable!() };
 YYMinorType::YY38(yyres)
}
            ,
            46 /* namelist ::= namelist_ */
            => 
{
let yyres :  V<Name> ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY148(yy0),) => {
 for n in yy0.iter() { self.insert_local_name(n); }; yyres = yy0.into_boxed_slice(); 
},    _ => unreachable!() };
 YYMinorType::YY104(yyres)
}
            ,
            48 /* namelist_ ::= namelist_ COMMA name */
            => 
{
let yyres :  Vec<Name> ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY148(yy0),YYMinorType::YY152(yy2),) => {
 yyres = { let mut v = yy0; v.push(yy2); v } 
},    _ => unreachable!() };
 YYMinorType::YY148(yyres)
}
            ,
            49 /* explist ::= explist_ */
            => 
{
let yyres :  V<Expr> ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY74(yy0),) => {
 yyres = yy0.into_boxed_slice(); 
},    _ => unreachable!() };
 YYMinorType::YY30(yyres)
}
            ,
            50 /* explist_ ::= exp */
            => 
{
let yyres :  Vec<Expr> ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY130(yy0),) => {
 yyres = vec![yy0]; 
},    _ => unreachable!() };
 YYMinorType::YY74(yyres)
}
            ,
            51 /* explist_ ::= explist_ COMMA exp */
            => 
{
let yyres :  Vec<Expr> ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY74(yy0),YYMinorType::YY130(yy2),) => {
 yyres = { let mut v = yy0; v.push(yy2); v } 
},    _ => unreachable!() };
 YYMinorType::YY74(yyres)
}
            ,
            52 /* exp ::= NIL */
            => 
{
let yyres :  Expr ;
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = ExprNil; 
} };
 YYMinorType::YY130(yyres)
}
            ,
            53 /* exp ::= FALSE */
            => 
{
let yyres :  Expr ;
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = ExprFalse; 
} };
 YYMinorType::YY130(yyres)
}
            ,
            54 /* exp ::= TRUE */
            => 
{
let yyres :  Expr ;
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = ExprTrue; 
} };
 YYMinorType::YY130(yyres)
}
            ,
            55 /* exp ::= DOTDOTDOT */
            => 
{
let yyres :  Expr ;
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = ExprEllipsis; 
} };
 YYMinorType::YY130(yyres)
}
            ,
            56 /* exp ::= NUMBER */
            => 
{
let yyres :  Expr ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY53(yy0),) => {
 yyres = ExprNumber(yy0); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            57 /* exp ::= STRING */
            => 
{
let yyres :  Expr ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY66(yy0),) => {
 yyres = ExprString(yy0); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            58 /* exp ::= functiondef */
            => 
{
let yyres :  Expr ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY132(yy0),) => {
 yyres = ExprFunction(yy0.0, yy0.1, yy0.2); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            59 /* exp ::= prefixexp */
            => 
{
let yyres :  Expr ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY130(yy0),) => {
 yyres = yy0; 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            60 /* exp ::= tableconstructor */
            => 
{
let yyres :  Expr ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY157(yy0),) => {
 yyres = ExprTable(yy0); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            61 /* exp ::= NOT exp */
            => 
{
let yyres :  Expr ;
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY130(yy1),) => {
 yyres = ExprUnary(UnNot, P(yy1)); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            62 /* exp ::= POUND exp */
            => 
{
let yyres :  Expr ;
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY130(yy1),) => {
 yyres = ExprUnary(UnLen, P(yy1)); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            63 /* exp ::= MINUS exp */
            => 
{
let yyres :  Expr ;
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY130(yy1),) => {
 yyres = ExprUnary(UnNeg, P(yy1)); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            64 /* exp ::= exp OR exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY130(yy0),YYMinorType::YY130(yy2),) => {
 yyres = ExprBinary(P(yy0), BinOr, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            65 /* exp ::= exp AND exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY130(yy0),YYMinorType::YY130(yy2),) => {
 yyres = ExprBinary(P(yy0), BinAnd, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            66 /* exp ::= exp LT exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY130(yy0),YYMinorType::YY130(yy2),) => {
 yyres = ExprBinary(P(yy0), BinLt, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            67 /* exp ::= exp LE exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY130(yy0),YYMinorType::YY130(yy2),) => {
 yyres = ExprBinary(P(yy0), BinLe, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            68 /* exp ::= exp GT exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY130(yy0),YYMinorType::YY130(yy2),) => {
 yyres = ExprBinary(P(yy0), BinGt, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            69 /* exp ::= exp GE exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY130(yy0),YYMinorType::YY130(yy2),) => {
 yyres = ExprBinary(P(yy0), BinGe, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            70 /* exp ::= exp EQEQ exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY130(yy0),YYMinorType::YY130(yy2),) => {
 yyres = ExprBinary(P(yy0), BinEq, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            71 /* exp ::= exp NE exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY130(yy0),YYMinorType::YY130(yy2),) => {
 yyres = ExprBinary(P(yy0), BinNe, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            72 /* exp ::= exp DOTDOT exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY130(yy0),YYMinorType::YY130(yy2),) => {
 yyres = ExprBinary(P(yy0), BinCon, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            73 /* exp ::= exp PLUS exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY130(yy0),YYMinorType::YY130(yy2),) => {
 yyres = ExprBinary(P(yy0), BinAdd, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            74 /* exp ::= exp MINUS exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY130(yy0),YYMinorType::YY130(yy2),) => {
 yyres = ExprBinary(P(yy0), BinSub, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            75 /* exp ::= exp STAR exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY130(yy0),YYMinorType::YY130(yy2),) => {
 yyres = ExprBinary(P(yy0), BinMul, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            76 /* exp ::= exp SLASH exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY130(yy0),YYMinorType::YY130(yy2),) => {
 yyres = ExprBinary(P(yy0), BinDiv, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            77 /* exp ::= exp PERCENT exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY130(yy0),YYMinorType::YY130(yy2),) => {
 yyres = ExprBinary(P(yy0), BinMod, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            78 /* exp ::= exp CARET exp */
            => 
{
let yyres :  Expr ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY130(yy0),YYMinorType::YY130(yy2),) => {
 yyres = ExprBinary(P(yy0), BinPow, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            79 /* exp ::= error */
            => 
{
let yyres :  Expr ;
let yyp0 = self.yystack.pop().unwrap();
match () {
 () => {
 yyres = ExprInvalid 
} };
 YYMinorType::YY130(yyres)
}
            ,
            80 /* prefixexp ::= var */
            => 
{
let yyres :  Expr ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY38(yy0),) => {
 yyres = ExprVar(P(yy0)); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            81 /* prefixexp ::= functioncall */
            => 
{
let yyres :  Expr ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY95(yy0),) => {
 yyres = ExprCall(P(yy0)); 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            82 /* prefixexp ::= OPENPAREN2 exp CLOSEPAREN */
            => 
{
let yyres :  Expr ;
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY130(yy1),) => {
 yyres = yy1; 
},    _ => unreachable!() };
 YYMinorType::YY130(yyres)
}
            ,
            83 /* functioncall ::= prefixexp args */
            => 
{
let yyres :  Call ;
let yyp1 = self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp1.minor,) {
 (YYMinorType::YY130(yy0),YYMinorType::YY86(yy1),) => {
 yyres = CallFunction(P(yy0), P(yy1)); 
},    _ => unreachable!() };
 YYMinorType::YY95(yyres)
}
            ,
            84 /* functioncall ::= prefixexp COLON name args */
            => 
{
let yyres :  Call ;
let yyp3 = self.yystack.pop().unwrap();
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,yyp3.minor,) {
 (YYMinorType::YY130(yy0),YYMinorType::YY152(yy2),YYMinorType::YY86(yy3),) => {
 yyres = CallMethod(P(yy0), yy2, P(yy3)); 
},    _ => unreachable!() };
 YYMinorType::YY95(yyres)
}
            ,
            85 /* args ::= OPENPAREN CLOSEPAREN */
            => 
{
let yyres :  Args ;
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = ArgsParen(P([])); 
} };
 YYMinorType::YY86(yyres)
}
            ,
            86 /* args ::= OPENPAREN explist CLOSEPAREN */
            => 
{
let yyres :  Args ;
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY30(yy1),) => {
 yyres = ArgsParen(yy1); 
},    _ => unreachable!() };
 YYMinorType::YY86(yyres)
}
            ,
            87 /* args ::= tableconstructor */
            => 
{
let yyres :  Args ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY157(yy0),) => {
 yyres = ArgsTable(yy0); 
},    _ => unreachable!() };
 YYMinorType::YY86(yyres)
}
            ,
            88 /* args ::= STRING */
            => 
{
let yyres :  Args ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY66(yy0),) => {
 yyres = ArgsString(yy0); 
},    _ => unreachable!() };
 YYMinorType::YY86(yyres)
}
            ,
            89 /* functiondef ::= FUNCTION funcbody */
            => 
{
let yyres :  (V<Name>, bool, P<Block>) ;
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY132(yy1),) => {
 yyres = yy1; 
},    _ => unreachable!() };
 YYMinorType::YY132(yyres)
}
            ,
            90 /* funcbody ::= OPENPAREN CLOSEPAREN block END */
            => 
{
let yyres :  (V<Name>, bool, P<Block>) ;
self.yystack.pop().unwrap();
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp2.minor,) {
 (YYMinorType::YY76(yy2),) => {
 yyres = (P([]), false, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY132(yyres)
}
            ,
            91 /* funcbody ::= OPENPAREN parlist CLOSEPAREN block END */
            => 
{
let yyres :  (V<Name>, bool, P<Block>) ;
self.yystack.pop().unwrap();
let yyp3 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,yyp3.minor,) {
 (YYMinorType::YY71(yy1),YYMinorType::YY76(yy3),) => {
 yyres = (yy1.0, yy1.1, P(yy3)); 
},    _ => unreachable!() };
 YYMinorType::YY132(yyres)
}
            ,
            92 /* parlist ::= namelist */
            => 
{
let yyres :  (V<Name>, bool) ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY104(yy0),) => {
 yyres = (yy0, false); 
},    _ => unreachable!() };
 YYMinorType::YY71(yyres)
}
            ,
            93 /* parlist ::= DOTDOTDOT */
            => 
{
let yyres :  (V<Name>, bool) ;
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = (P([]), true); 
} };
 YYMinorType::YY71(yyres)
}
            ,
            94 /* parlist ::= namelist COMMA2 DOTDOTDOT */
            => 
{
let yyres :  (V<Name>, bool) ;
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY104(yy0),) => {
 yyres = (yy0, true); 
},    _ => unreachable!() };
 YYMinorType::YY71(yyres)
}
            ,
            95 /* tableconstructor ::= OPENBRACE CLOSEBRACE */
            => 
{
let yyres :  V<Field> ;
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match () {
 () => {
 yyres = P([]); 
} };
 YYMinorType::YY157(yyres)
}
            ,
            96 /* tableconstructor ::= OPENBRACE fieldlist CLOSEBRACE */
            => 
{
let yyres :  V<Field> ;
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY157(yy1),) => {
 yyres = yy1; 
},    _ => unreachable!() };
 YYMinorType::YY157(yyres)
}
            ,
            97 /* fieldlist ::= fieldlist_ */
            => 
{
let yyres :  V<Field> ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY41(yy0),) => {
 yyres = yy0.into_boxed_slice(); 
},    _ => unreachable!() };
 YYMinorType::YY157(yyres)
}
            ,
            98 /* fieldlist ::= fieldlist_ fieldsep */
            => 
{
let yyres :  V<Field> ;
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY41(yy0),) => {
 yyres = yy0.into_boxed_slice(); 
},    _ => unreachable!() };
 YYMinorType::YY157(yyres)
}
            ,
            99 /* fieldlist_ ::= field */
            => 
{
let yyres :  Vec<Field> ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY147(yy0),) => {
 yyres = vec![yy0]; 
},    _ => unreachable!() };
 YYMinorType::YY41(yyres)
}
            ,
            100 /* fieldlist_ ::= fieldlist_ fieldsep field */
            => 
{
let yyres :  Vec<Field> ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY41(yy0),YYMinorType::YY147(yy2),) => {
 yyres = { let mut v = yy0; v.push(yy2); v }; 
},    _ => unreachable!() };
 YYMinorType::YY41(yyres)
}
            ,
            101 /* field ::= exp */
            => 
{
let yyres :  Field ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY130(yy0),) => {
 yyres = FieldAuto(P(yy0)); 
},    _ => unreachable!() };
 YYMinorType::YY147(yyres)
}
            ,
            102 /* field ::= name EQ exp */
            => 
{
let yyres :  Field ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY152(yy0),YYMinorType::YY130(yy2),) => {
 yyres = FieldNamed(yy0, P(yy2)); 
},    _ => unreachable!() };
 YYMinorType::YY147(yyres)
}
            ,
            103 /* field ::= OPENBRACKET exp CLOSEBRACKET EQ exp */
            => 
{
let yyres :  Field ;
let yyp4 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,yyp4.minor,) {
 (YYMinorType::YY130(yy1),YYMinorType::YY130(yy4),) => {
 yyres = FieldExpr(P(yy1), P(yy4)); 
},    _ => unreachable!() };
 YYMinorType::YY147(yyres)
}
            ,
            104 /* fieldsep ::= COMMA|SEMI */
            => 
{
self.yystack.pop().unwrap();
match () {
 () => {


} };
 YYMinorType::YY0
}
            ,
            105 /* name ::= NAME */
            => 
{
let yyres :  Name ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY66(yy0),) => {
 yyres = Name::new(yy0); 
},    _ => unreachable!() };
 YYMinorType::YY152(yyres)
}
            ,
            106 /* declname_ ::= name */
            => 
{
let yyres :  Name ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY152(yy0),) => {
 self.insert_local_name(&yy0); yyres = yy0; 
},    _ => unreachable!() };
 YYMinorType::YY152(yyres)
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
 self.recoverable = false; 
    }

    fn yy_syntax_error(&mut self, token: &Token) {
 self.last_error_count = self.last_error_count + 1; self.last_error = Some(token.clone()); 
    }

    fn yy_accept(&mut self) {
        self.yystack.clear();
 self.accepted = true; 
    }
}

