%fallback OPENPAREN2 OPENPAREN.
%fallback COMMA2 COMMA.
%type NAME { String }
%type NUMBER { f64 }
%type STRING { String }

chunk ::= block(B). { self.chunk = Some(Chunk { block: B }) }

%type block { Block }
block(B) ::= block_(A) pop_scope_. { B = A; }
%type block_ { Block }
block_(B) ::= stats_(S). { B = Block { stmts: S.into_boxed_slice(), ret: None }; }
block_(B) ::= stats_(S) retstat(R). { B = Block { stmts: S.into_boxed_slice(), ret: Some(R) }; }
push_scope_ ::= . { self.push_scope(); }
pop_scope_ ::= . { self.pop_scope(); }

%type stats_ { Vec<Stmt> }
stats_(L) ::= . { L = vec![]; }
stats_(L) ::= stats_(V) stat_(S). { L = if let Some(s) = S { let mut v = V; v.push(s); v } else { V } }

%type stat_ { Option<Stmt> }
stat_(S) ::= SEMI. { S = None; }
stat_(S) ::= stat(Z). { S = Some(Z); }

%type stat { Stmt }
stat(S) ::= varlist(V) EQ explist(E). { S = StmtSet(V, E); }
stat(S) ::= functioncall(C). { S = StmtCall(P(C)); }
stat(S) ::= label(N). { S = StmtLabel(N); }
stat(S) ::= BREAK. { S = StmtBreak; }
stat(S) ::= GOTO name(N). { S = StmtGoto(N); }
stat(S) ::= DO push_scope_ block(B) END. { S = StmtDo(P(B)); }
stat(S) ::= WHILE push_scope_ exp(E) DO block(B) END. { S = StmtWhile(P(E), P(B)); }
stat(S) ::= REPEAT push_scope_ block(B) UNTIL exp(E). { S = StmtRepeat(P(B), P(E)); }
stat(S) ::= IF exp(E) THEN push_scope_ block(B) elsifs_(I) else_(J) END. { S = StmtIf(P(E), P(B), I.into_boxed_slice(), J); }
stat(S) ::= FOR push_scope_ declname_(N) EQ exp(I) COMMA exp(J) DO block(B) END. { S = StmtForNum(N, P(I), P(J), None, P(B)); }
stat(S) ::= FOR push_scope_ declname_(N) EQ exp(I) COMMA exp(J) COMMA exp(K) DO block(B) END. { S = StmtForNum(N, P(I), P(J), Some(P(K)), P(B)); }
stat(S) ::= FOR push_scope_ namelist(N) IN explist(E) DO block(B) END. { S = StmtForIn(N, E, P(B)); }
stat(S) ::= FUNCTION push_scope_ funcname(N) funcbody(B). { self.insert_global_name(&N.0[0]); S = StmtFunction(N.0, N.1, B.0, B.1, B.2); }
stat(S) ::= LOCAL FUNCTION push_scope_ declname_(N) funcbody(B). { S = StmtLocalFunction(N, B.0, B.1, B.2); }
stat(S) ::= LOCAL namelist(N). { S = StmtLocal(N, None); }
stat(S) ::= LOCAL namelist(N) EQ explist(E). { S = StmtLocal(N, Some(E)); }
stat(S) ::= error. { S = StmtInvalid }

%type elsifs_ { Vec<(Expr, Block)> }
elsifs_(L) ::= . { L = vec![]; }
elsifs_(L) ::= elsifs_(V) ELSEIF exp(E) THEN block(B). { L = { let mut v = V; v.push((E, B)); v }; }
%type else_ { Option<P<Block>> }
else_(E) ::= . { E = None; }
else_(E) ::= ELSE block(B). { E = Some(P(B)); }

%type retstat { V<Expr> }
retstat(R) ::= RETURN. { R = P([]); }
retstat(R) ::= RETURN SEMI. { R = P([]); }
retstat(R) ::= RETURN explist(E). { R = E; }
retstat(R) ::= RETURN explist(E) SEMI. { R = E; }

%type label { Name }
label(L) ::= COLONCOLON name(N) COLONCOLON. { L = N; }

%type funcname { (V<Name>, Option<Name>) }
funcname(F) ::= dotnames_(D). { F = (D.into_boxed_slice(), None); }
funcname(F) ::= dotnames_(D) COLON name(N). { F = (D.into_boxed_slice(), Some(N)); }

%type dotnames_ { Vec<Name> }
dotnames_(L) ::= name(N). { L = vec![N]; }
dotnames_(L) ::= dotnames_(V) DOT name(N). { L = { let mut v = V; v.push(N); v }; }

%type varlist { V<Var> }
varlist(A) ::= varlist_(B). { A = B.into_boxed_slice(); }
%type varlist_ { Vec<Var> }
varlist_(L) ::= var(A). { L = vec![A]; }
varlist_(L) ::= varlist_(V) COMMA var(A). { L = { let mut v = V; v.push(A); v } }

%type var { Var }
var(V) ::= name(N). { self.is_in_scope(&N); V = VarName(N); }
var(V) ::= prefixexp(E) OPENBRACKET exp(F) CLOSEBRACKET. { V = VarProperty(P(E), P(F)); }
var(V) ::= prefixexp(E) DOT name(N). { V = VarMember(P(E), N); }

%type namelist { V<Name> }
namelist(A) ::= namelist_(B). { for n in B.iter() { self.insert_local_name(n); }; A = B.into_boxed_slice(); }
%type namelist_ { Vec<Name> }
namelist_(L) ::= name(N). { L = vec![N]; }
namelist_(L) ::= namelist_(V) COMMA name(N). { L = { let mut v = V; v.push(N); v } }

%type explist { V<Expr> }
explist(A) ::= explist_(B). { A = B.into_boxed_slice(); }
%type explist_ { Vec<Expr> }
explist_(L) ::= exp(E). { L = vec![E]; }
explist_(L) ::= explist_(V) COMMA exp(E). { L = { let mut v = V; v.push(E); v } }

%left OR.
%left AND.
%left LT GT LE GE NE EQEQ.
%right DOTDOT.
%left PLUS MINUS.
%left STAR SLASH PERCENT.
%right NOT POUND.
%right CARET.

%type exp { Expr }
exp(E) ::= NIL. { E = ExprNil; }
exp(E) ::= FALSE. { E = ExprFalse; }
exp(E) ::= TRUE. { E = ExprTrue; }
exp(E) ::= DOTDOTDOT. { E = ExprEllipsis; }
exp(E) ::= NUMBER(N). { E = ExprNumber(N); }
exp(E) ::= STRING(S). { E = ExprString(S); }
exp(E) ::= functiondef(F). { E = ExprFunction(F.0, F.1, F.2); }
exp(E) ::= prefixexp(P). { E = P; }
exp(E) ::= tableconstructor(T). { E = ExprTable(T); }
exp(E) ::= NOT exp(A). { E = ExprUnary(UnNot, P(A)); }
exp(E) ::= POUND exp(A). { E = ExprUnary(UnLen, P(A)); }
exp(E) ::= MINUS exp(A). [NOT] { E = ExprUnary(UnNeg, P(A)); }
exp(E) ::= exp(A) OR exp(B). { E = ExprBinary(P(A), BinOr, P(B)); }
exp(E) ::= exp(A) AND exp(B). { E = ExprBinary(P(A), BinAnd, P(B)); }
exp(E) ::= exp(A) LT exp(B). { E = ExprBinary(P(A), BinLt, P(B)); }
exp(E) ::= exp(A) LE exp(B). { E = ExprBinary(P(A), BinLe, P(B)); }
exp(E) ::= exp(A) GT exp(B). { E = ExprBinary(P(A), BinGt, P(B)); }
exp(E) ::= exp(A) GE exp(B). { E = ExprBinary(P(A), BinGe, P(B)); }
exp(E) ::= exp(A) EQEQ exp(B). { E = ExprBinary(P(A), BinEq, P(B)); }
exp(E) ::= exp(A) NE exp(B). { E = ExprBinary(P(A), BinNe, P(B)); }
exp(E) ::= exp(A) DOTDOT exp(B). { E = ExprBinary(P(A), BinCon, P(B)); }
exp(E) ::= exp(A) PLUS exp(B). { E = ExprBinary(P(A), BinAdd, P(B)); }
exp(E) ::= exp(A) MINUS exp(B). { E = ExprBinary(P(A), BinSub, P(B)); }
exp(E) ::= exp(A) STAR exp(B). { E = ExprBinary(P(A), BinMul, P(B)); }
exp(E) ::= exp(A) SLASH exp(B). { E = ExprBinary(P(A), BinDiv, P(B)); }
exp(E) ::= exp(A) PERCENT exp(B). { E = ExprBinary(P(A), BinMod, P(B)); }
exp(E) ::= exp(A) CARET exp(B). { E = ExprBinary(P(A), BinPow, P(B)); }
exp(E) ::= error. { E = ExprInvalid }

%type prefixexp { Expr }
prefixexp(E) ::= var(V). { E = ExprVar(P(V)); }
prefixexp(E) ::= functioncall(C). { E = ExprCall(P(C)); }
prefixexp(E) ::= OPENPAREN2 exp(F) CLOSEPAREN. { E = F; }

%type functioncall { Call }
functioncall(C) ::= prefixexp(E) args(A). { C = CallFunction(P(E), P(A)); }
functioncall(C) ::= prefixexp(E) COLON name(N) args(A). { C = CallMethod(P(E), N, P(A)); }

%type args { Args }
args(A) ::= OPENPAREN CLOSEPAREN. { A = ArgsParen(P([])); }
args(A) ::= OPENPAREN explist(E) CLOSEPAREN. { A = ArgsParen(E); }
args(A) ::= tableconstructor(T). { A = ArgsTable(T); }
args(A) ::= STRING(S). { A = ArgsString(S); }

%type functiondef { (V<Name>, bool, P<Block>) }
functiondef(F) ::= FUNCTION funcbody(G). { F = G; }

%type funcbody { (V<Name>, bool, P<Block>) }
funcbody(F) ::= OPENPAREN CLOSEPAREN block(B) END. { F = (P([]), false, P(B)); }
funcbody(F) ::= OPENPAREN parlist(A) CLOSEPAREN block(B) END. { F = (A.0, A.1, P(B)); }

%type parlist { (V<Name>, bool) }
parlist(A) ::= namelist(N). { A = (N, false); }
parlist(A) ::= DOTDOTDOT. { A = (P([]), true); }
parlist(A) ::= namelist(N) COMMA2 DOTDOTDOT. { A = (N, true); }

%type tableconstructor { V<Field> }
tableconstructor(T) ::= OPENBRACE CLOSEBRACE. { T = P([]); }
tableconstructor(T) ::= OPENBRACE fieldlist(L) CLOSEBRACE. { T = L; }

%type fieldlist { V<Field> }
fieldlist(A) ::= fieldlist_(B). { A = B.into_boxed_slice(); }
fieldlist(A) ::= fieldlist_(B) fieldsep. { A = B.into_boxed_slice(); }
%type fieldlist_ { Vec<Field> }
fieldlist_(L) ::= field(F). { L = vec![F]; }
fieldlist_(L) ::= fieldlist_(V) fieldsep field(F). { L = { let mut v = V; v.push(F); v }; }

%type field { Field }
field(F) ::= exp(E). { F = FieldAuto(P(E)); }
field(F) ::= name(N) EQ exp(E). { F = FieldNamed(N, P(E)); }
field(F) ::= OPENBRACKET exp(A) CLOSEBRACKET EQ exp(B). { F = FieldExpr(P(A), P(B)); }

fieldsep ::= COMMA|SEMI.

%type name { Name }
name(N) ::= NAME(S). { N = Name::new(S); }
%type declname_ { Name }
declname_(A) ::= name(N). { self.insert_local_name(&N); A = N; }

// binop and unop are integrated into exp

%derive_token { Clone }
%include { use ::ast::*; }
%parse_accept { self.accepted = true; }
%parse_failure { self.recoverable = false; }
%syntax_error { self.last_error_count = self.last_error_count + 1; self.last_error = Some(token.clone()); }

// vim: et sw=4 ts=4 sts=4
