#![allow(dead_code)]
#![allow(unused_variables)]
/* TMPL: %include */
%%
use std::collections::HashSet;
/* TMPL: makeheader cruft */

%%

/* TMPL: types */

%%

//const YY_NO_ACTION: i32 = YYNSTATE+YYNRULE+2;
//const YY_ACCEPT_ACTION: i32 = YYNSTATE+YYNRULE+1;
//const YY_ERROR_ACTION: i32 = YYNSTATE+YYNRULE+1;

/* TMPL: action tables */

%%

/* TMPL: fallback tokens */

%%

/* TMPL: symbol names */

%%

/* TMPL: rules */

%%

/* TMPL: destructors */

%%

/* TMPL: stack-overflow */

%%

/* TMPL: stack-overflow */

%%

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
%%
}

impl Parser {

    #[inline]
    pub fn new(
%%
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

%%

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
%%
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
%%
    }

    fn yy_syntax_error(&mut self, token: &Token) {
%%
    }

    fn yy_accept(&mut self) {
        self.yystack.clear();
%%
    }
}

