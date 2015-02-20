    { if ((nres) == LUA_MULTRET && L->ci->top < L->top) L->ci->top = L->top; }
#define LUA_ERRFILE     (LUA_ERRERR+1)
typedef struct luaL_Reg {
  const char *name;
  lua_CFunction func;
} luaL_Reg;
void (luaL_checkversion_) (lua_State *L, lua_Number ver);
#define luaL_checkversion(L)	luaL_checkversion_(L, LUA_VERSION_NUM)
int (luaL_getmetafield) (lua_State *L, int obj, const char *e);
int (luaL_callmeta) (lua_State *L, int obj, const char *e);
const char *(luaL_tolstring) (lua_State *L, int idx, size_t *len);
int (luaL_argerror) (lua_State *L, int numarg, const char *extramsg);
const char *(luaL_checklstring) (lua_State *L, int numArg,
                                                          size_t *l);
const char *(luaL_optlstring) (lua_State *L, int numArg,
                                          const char *def, size_t *l);
lua_Number (luaL_checknumber) (lua_State *L, int numArg);
lua_Number (luaL_optnumber) (lua_State *L, int nArg, lua_Number def);
lua_Integer (luaL_checkinteger) (lua_State *L, int numArg);
lua_Integer (luaL_optinteger) (lua_State *L, int nArg,
                                          lua_Integer def);
lua_Unsigned (luaL_checkunsigned) (lua_State *L, int numArg);
lua_Unsigned (luaL_optunsigned) (lua_State *L, int numArg,
                                            lua_Unsigned def);
void (luaL_checkstack) (lua_State *L, int sz, const char *msg);
void (luaL_checktype) (lua_State *L, int narg, int t);
void (luaL_checkany) (lua_State *L, int narg);
int   (luaL_newmetatable) (lua_State *L, const char *tname);
void  (luaL_setmetatable) (lua_State *L, const char *tname);
void *(luaL_testudata) (lua_State *L, int ud, const char *tname);
void *(luaL_checkudata) (lua_State *L, int ud, const char *tname);
void (luaL_where) (lua_State *L, int lvl);
int (luaL_error) (lua_State *L, const char *fmt, ...);
int (luaL_checkoption) (lua_State *L, int narg, const char *def,
                                   const char *const lst[]);
int (luaL_fileresult) (lua_State *L, int stat, const char *fname);
int (luaL_execresult) (lua_State *L, int stat);
#define LUA_NOREF       (-2)
#define LUA_REFNIL      (-1)
int (luaL_ref) (lua_State *L, int t);
void (luaL_unref) (lua_State *L, int t, int ref);
int (luaL_loadfilex) (lua_State *L, const char *filename,
                                               const char *mode);
#define luaL_loadfile(L,f)	luaL_loadfilex(L,f,NULL)
int (luaL_loadbufferx) (lua_State *L, const char *buff, size_t sz,
                                   const char *name, const char *mode);
int (luaL_loadstring) (lua_State *L, const char *s);
lua_State *(luaL_newstate) (void);
int (luaL_len) (lua_State *L, int idx);
const char *(luaL_gsub) (lua_State *L, const char *s, const char *p,
                                                  const char *r);
void (luaL_setfuncs) (lua_State *L, const luaL_Reg *l, int nup);
int (luaL_getsubtable) (lua_State *L, int idx, const char *fname);
void (luaL_traceback) (lua_State *L, lua_State *L1,
                                  const char *msg, int level);
void (luaL_requiref) (lua_State *L, const char *modname,
                                 lua_CFunction openf, int glb);
#define luaL_newlibtable(L,l)	lua_createtable(L, 0, sizeof(l)/sizeof((l)[0]) - 1)
#define luaL_newlib(L,l)	(luaL_newlibtable(L,l), luaL_setfuncs(L,l,0))
#define luaL_argcheck(L, cond,numarg,extramsg)	((void)((cond) || luaL_argerror(L, (numarg), (extramsg))))
#define luaL_checkstring(L,n)	(luaL_checklstring(L, (n), NULL))
#define luaL_optstring(L,n,d)	(luaL_optlstring(L, (n), (d), NULL))
#define luaL_checkint(L,n)	((int)luaL_checkinteger(L, (n)))
#define luaL_optint(L,n,d)	((int)luaL_optinteger(L, (n), (d)))
#define luaL_checklong(L,n)	((long)luaL_checkinteger(L, (n)))
#define luaL_optlong(L,n,d)	((long)luaL_optinteger(L, (n), (d)))
#define luaL_typename(L,i)	lua_typename(L, lua_type(L,(i)))
#define luaL_dofile(L, fn) (luaL_loadfile(L, fn) || lulua_pushnila_pcall(L, 0, LUA_MULTRET, 0))
#define luaL_dostring(L, s) (luaL_loadstring(L, s) || lua_pcall(L, 0, LUA_MULTRET, 0))
#define luaL_getmetatable(L,n)	(lua_getfield(L, LUA_REGISTRYINDEX, (n)))
#define luaL_opt(L,f,n,d)	(lua_isnoneornil(L,(n)) ? (d) : f(L,(n)))
#define luaL_loadbuffer(L,s,sz,n)	luaL_loadbufferx(L,s,sz,n,NULL)
typedef struct luaL_Buffer {
  char *b;  /* buffer address */
  lua_State *L;
  char initb[LUAL_BUFFERSIZE];  /* initial buffer */
} luaL_Buffer;
#define luaL_addchar(B,c) ((void)((B)->n < (B)->size || luaL_prepbuffsize((B), 1)), ((B)->b[(B)->n++] = (c)))
#define luaL_addsize(B,s)	((B)->n += (s))
void (luaL_buffinit) (lua_State *L, luaL_Buffer *B);
char *(luaL_prepbuffsize) (luaL_Buffer *B, size_t sz);
void (luaL_addlstring) (luaL_Buffer *B, const char *s, size_t l);
void (luaL_addstring) (luaL_Buffer *B, const char *s);
void (luaL_addvalue) (luaL_Buffer *B);
void (luaL_pushresult) (luaL_Buffer *B);
void (luaL_pushresultsize) (luaL_Buffer *B, size_t sz);
char *(luaL_buffinitsize) (lua_State *L, luaL_Buffer *B, size_t sz);
#define luaL_prepbuffer(B)	luaL_prepbuffsize(B, LUAL_BUFFERSIZE)
#define LUA_FILEHANDLE          "FILE*"
typedef struct luaL_Stream {
  FILE *f;  /* stream (NULL for incompletely created streams) */
  lua_CFunction closef;  /* to close stream (NULL for closed streams) */
} luaL_Stream;
#if defined(LUA_COMPAT_MODULE)
void (luaL_pushmodule) (lua_State *L, const char *modname,
                                   int sizehint);
void (luaL_openlib) (lua_State *L, const char *libname,
                                const luaL_Reg *l, int nup);
#define luaL_register(L,n,l)	(luaL_openlib(L,(n),(l),0))
#define luaK_codeAsBx(fs,o,A,sBx)	luaK_codeABx(fs,o,A,(sBx)+MAXARG_sBx)
#define luaK_setmultret(fs,e)	luaK_setreturns(fs, e, LUA_MULTRET)
#define luaK_jumpto(fs,t)	luaK_patchlist(fs, luaK_jump(fs), t)
int luaK_codeABx (FuncState *fs, OpCode o, int A, unsigned int Bx);
int luaK_codeABC (FuncState *fs, OpCode o, int A, int B, int C);
int luaK_codek (FuncState *fs, int reg, int k);
void luaK_fixline (FuncState *fs, int line);
void luaK_nil (FuncState *fs, int from, int n);
void luaK_reserveregs (FuncState *fs, int n);
void luaK_checkstack (FuncState *fs, int n);
int luaK_stringK (FuncState *fs, TString *s);
int luaK_numberK (FuncState *fs, lua_Number r);
void luaK_dischargevars (FuncState *fs, expdesc *e);
int luaK_exp2anyreg (FuncState *fs, expdesc *e);
void luaK_exp2anyregup (FuncState *fs, expdesc *e);
void luaK_exp2nextreg (FuncState *fs, expdesc *e);
void luaK_exp2val (FuncState *fs, expdesc *e);
int luaK_exp2RK (FuncState *fs, expdesc *e);
void luaK_self (FuncState *fs, expdesc *e, expdesc *key);
void luaK_indexed (FuncState *fs, expdesc *t, expdesc *k);
void luaK_goiftrue (FuncState *fs, expdesc *e);
void luaK_goiffalse (FuncState *fs, expdesc *e);
void luaK_storevar (FuncState *fs, expdesc *var, expdesc *e);
void luaK_setreturns (FuncState *fs, expdesc *e, int nresults);
void luaK_setoneret (FuncState *fs, expdesc *e);
int luaK_jump (FuncState *fs);
void luaK_ret (FuncState *fs, int first, int nret);
void luaK_patchlist (FuncState *fs, int list, int target);
void luaK_patchtohere (FuncState *fs, int list);
void luaK_patchclose (FuncState *fs, int list, int level);
void luaK_concat (FuncState *fs, int *l1, int l2);
int luaK_getlabel (FuncState *fs);
void luaK_prefix (FuncState *fs, UnOpr op, expdesc *v, int line);
void luaK_infix (FuncState *fs, BinOpr op, expdesc *v);
void luaK_posfix (FuncState *fs, BinOpr op, expdesc *v1,
                            expdesc *v2, int line);
void luaK_setlist (FuncState *fs, int base, int nelems, int tostore);
#if !defined(LUA_USE_CTYPE)
#define LUA_USE_CTYPE	0
#else
#define LUA_USE_CTYPE	1
#endif
#if !LUA_USE_CTYPE	/* { */
#define testprop(c,p)	(luai_ctype_[(c)+1] & (p))
const lu_byte luai_ctype_[UCHAR_MAX + 2];
l_noret luaG_typeerror (lua_State *L, const TValue *o,
                                                const char *opname);
l_noret luaG_concaterror (lua_State *L, StkId p1, StkId p2);
l_noret luaG_aritherror (lua_State *L, const TValue *p1,
                                                 const TValue *p2);
l_noret luaG_ordererror (lua_State *L, const TValue *p1,
                                                 const TValue *p2);
l_noret luaG_runerror (lua_State *L, const char *fmt, ...);
l_noret luaG_errormsg (lua_State *L);
#define luaD_checkstack(L,n)	if (L->stack_last - L->top <= (n)) luaD_growstack(L, n); else condmovestack(L);
#define incr_top(L) {L->top++; luaD_checkstack(L,0);}
typedef void (*Pfunc) (lua_State *L, void *ud);
int luaD_protectedparser (lua_State *L, ZIO *z, const char *name,
                                                  const char *mode);
void luaD_hook (lua_State *L, int event, int line);
int luaD_precall (lua_State *L, StkId func, int nresults);
void luaD_call (lua_State *L, StkId func, int nResults,
                                        int allowyield);
int luaD_pcall (lua_State *L, Pfunc func, void *u,
                                        ptrdiff_t oldtop, ptrdiff_t ef);
int luaD_poscall (lua_State *L, StkId firstResult);
void luaD_reallocstack (lua_State *L, int newsize);
void luaD_growstack (lua_State *L, int n);
void luaD_shrinkstack (lua_State *L);
l_noret luaD_throw (lua_State *L, int errcode);
int luaD_rawrunprotected (lua_State *L, Pfunc f, void *ud);
Proto *luaF_newproto (lua_State *L);
Closure *luaF_newCclosure (lua_State *L, int nelems);
Closure *luaF_newLclosure (lua_State *L, int nelems);
UpVal *luaF_newupval (lua_State *L);
UpVal *luaF_findupval (lua_State *L, StkId level);
void luaF_close (lua_State *L, StkId level);
void luaF_freeproto (lua_State *L, Proto *f);
void luaF_freeupval (lua_State *L, UpVal *uv);
const char *luaF_getlocalname (const Proto *func, int local_number,
                                         int pc);
#define luaC_white(g)	cast(lu_byte, (g)->currentwhite & WHITEBITS)
#define luaC_condGC(L,c) {if (G(L)->GCdebt > 0) {c;}; condchangemem(L);}
#define luaC_checkGC(L)		luaC_condGC(L, luaC_step(L);)
#define luaC_barrier(L,p,v) { if (valiswhite(v) && isblack(obj2gco(p)))  luaC_barrier_(L,obj2gco(p),gcvalue(v)); }
#define luaC_barrierback(L,p,v) { if (valiswhite(v) && isblack(obj2gco(p)))  luaC_barrierback_(L,p); }
#define luaC_objbarrier(L,p,o)  { if (iswhite(obj2gco(o)) && isblack(obj2gco(p))) luaC_barrier_(L,obj2gco(p),obj2gco(o)); }
#define luaC_objbarrierback(L,p,o)  { if (iswhite(obj2gco(o)) && isblack(obj2gco(p))) luaC_barrierback_(L,p); }
#define luaC_barrierproto(L,p,c) { if (isblack(obj2gco(p))) luaC_barrierproto_(L,p,c); }
void luaC_freeallobjects (lua_State *L);
void luaC_step (lua_State *L);
void luaC_forcestep (lua_State *L);
void luaC_runtilstate (lua_State *L, int statesmask);
void luaC_fullgc (lua_State *L, int isemergency);
GCObject *luaC_newobj (lua_State *L, int tt, size_t sz,
                                 GCObject **list, int offset);
void luaC_barrier_ (lua_State *L, GCObject *o, GCObject *v);
void luaC_barrierback_ (lua_State *L, GCObject *o);
void luaC_barrierproto_ (lua_State *L, Proto *p, Closure *c);
void luaC_checkfinalizer (lua_State *L, GCObject *o, Table *mt);
void luaC_checkupvalcolor (global_State *g, UpVal *uv);
void luaC_changemode (lua_State *L, int mode);
  lua_Number r;
  TString *ts;
  struct lua_State *L;
  ZIO *z;  /* input stream */
void luaX_init (lua_State *L);
void luaX_setinput (lua_State *L, LexState *ls, ZIO *z,
                              TString *source, int firstchar);
TString *luaX_newstring (LexState *ls, const char *str, size_t l);
void luaX_next (LexState *ls);
int luaX_lookahead (LexState *ls);
l_noret luaX_syntaxerror (LexState *ls, const char *s);
const char *luaX_token2str (LexState *ls, int token);
typedef unsigned LUA_INT32 lu_int32;
typedef LUAI_UMEM lu_mem;
typedef LUAI_MEM l_mem;
#if !defined(LUAI_USER_ALIGNMENT_T)
#define LUAI_USER_ALIGNMENT_T	union { double u; void *s; long l; }
#endif
typedef LUAI_USER_ALIGNMENT_T L_Umaxalign;
typedef LUAI_UACNUMBER l_uacNumber;
#if defined(lua_assert)
#define check_exp(c,e)		(lua_assert(c), (e))
#define lua_longassert(c)	{ if (!(c)) lua_assert(0); }
#else
#define lua_assert(c)		((void)0)
#define check_exp(c,e)		(e)
#define lua_longassert(c)	((void)0)
#endif
#if !defined(luai_apicheck)
#if defined(LUA_USE_APICHECK)
#include <assert.h>
#define luai_apicheck(L,e)	assert(e)
#else
#define luai_apicheck(L,e)	lua_assert(e)
#endif
#define api_check(l,e,msg)	luai_apicheck(l,(e) && msg)
#define cast_num(i)	cast(lua_Number, (i))
#define cast_int(i)	cast(int, (i))
#if !defined(LUAI_MAXCCALLS)
#define LUAI_MAXCCALLS		200
#endif
#if !defined(LUA_MINBUFFER)
#define LUA_MINBUFFER	32
#endif
#if !defined(lua_lock)
#define lua_lock(L)     ((void) 0)
#define lua_unlock(L)   ((void) 0)
#endif
#if !defined(luai_threadyield)
#define luai_threadyield(L)     {lua_unlock(L); lua_lock(L);}
#endif
#if !defined(luai_userstateopen)
#define luai_userstateopen(L)		((void)L)
#endif
#if !defined(luai_userstateclose)
#define luai_userstateclose(L)		((void)L)
#endif
#if !defined(luai_userstatethread)
#define luai_userstatethread(L,L1)	((void)L)
#endif
#if !defined(luai_userstatefree)
#define luai_userstatefree(L,L1)	((void)L)
#endif
#if !defined(luai_userstateresume)
#define luai_userstateresume(L,n)       ((void)L)
#endif
#if !defined(luai_userstateyield)
#define luai_userstateyield(L,n)        ((void)L)
#endif
#if defined(MS_ASMTRICK) || defined(LUA_MSASMTRICK)	/* { */
#define lua_number2int(i,n)  __asm {__asm fld n   __asm fistp i}
#define lua_number2integer(i,n)		lua_number2int(i, n)
#define lua_number2unsigned(i,n)  {__int64 l; __asm {__asm fld n   __asm fistp l} i = (unsigned int)l;}
#elif defined(LUA_IEEE754TRICK)		/* }{ */
union luai_Cast { double l_d; LUA_INT32 l_p[2]; };
#if !defined(LUA_IEEEENDIAN)	/* { */
#define LUAI_EXTRAIEEE	static const union luai_Cast ieeeendian = {-(33.0 + 6755399441055744.0)};
#define LUA_IEEEENDIANLOC	(ieeeendian.l_p[1] == 33)
#else
#define LUA_IEEEENDIANLOC	LUA_IEEEENDIAN
#define LUAI_EXTRAIEEE		/* empty */
#endif				/* } */
#define lua_number2int32(i,n,t) { LUAI_EXTRAIEEE volatile union luai_Cast u; u.l_d = (n) + 6755399441055744.0; (i) = (t)u.l_p[LUA_IEEEENDIANLOC]; }
#define luai_hashnum(i,n)  { volatile union luai_Cast u; u.l_d = (n) + 1.0;  /* avoid -0 */ (i) = u.l_p[0]; (i) += u.l_p[1]; }  /* add double bits for his hash */
#define lua_number2int(i,n)		lua_number2int32(i, n, int)
#define lua_number2unsigned(i,n)	lua_number2int32(i, n, lua_Unsigned)
#if defined(LUA_IEEELL)
#define lua_number2integer(i,n)		lua_number2int32(i, n, lua_Integer)
#endif
#if !defined(lua_number2int)
#define lua_number2int(i,n)	((i)=(int)(n))
#endif
#if !defined(lua_number2integer)
#define lua_number2integer(i,n)	((i)=(lua_Integer)(n))
#endif
#if !defined(lua_number2unsigned)	/* { */
#if defined(LUA_NUMBER_DOUBLE) || defined(LUA_NUMBER_FLOAT)
#include <math.h>
#define SUPUNSIGNED	((lua_Number)(~(lua_Unsigned)0) + 1)
#define lua_number2unsigned(i,n)  ((i)=(lua_Unsigned)((n) - floor((n)/SUPUNSIGNED)*SUPUNSIGNED))
#else
#define lua_number2unsigned(i,n)	((i)=(lua_Unsigned)(n))
#endif
#if !defined(lua_unsigned2number)
#define lua_unsigned2number(u)  (((u) <= (lua_Unsigned)INT_MAX) ? (lua_Number)(int)(u) : (lua_Number)(u))
#endif
#if defined(ltable_c) && !defined(luai_hashnum)
#define luai_hashnum(i,n) { int e;  n = l_mathop(frexp)(n, &e) * (lua_Number)(INT_MAX - DBL_MAX_EXP);  lua_number2int(i, n); i += e; }
#define condmovestack(L)	luaD_reallocstack((L), (L)->stacksize)
#endif
	((void)(!(G(L)->gcrunning) || (luaC_fullgc(L, 0), 1)))
#endif
#define luaM_reallocv(L,b,on,n,e) (cast(void, (cast(size_t, (n)+1) > MAX_SIZET/(e)) ? (luaM_toobig(L), 0) : 0), luaM_realloc_(L, (b), (on)*(e), (n)*(e)))
#define luaM_freemem(L, b, s)	luaM_realloc_(L, (b), (s), 0)
#define luaM_free(L, b)		luaM_realloc_(L, (b), sizeof(*(b)), 0)
#define luaM_freearray(L, b, n)   luaM_reallocv(L, (b), n, 0, sizeof((b)[0]))
#define luaM_malloc(L,s)	luaM_realloc_(L, NULL, 0, (s))
#define luaM_new(L,t)		cast(t *, luaM_malloc(L, sizeof(t)))
#define luaM_newvector(L,n,t) cast(t *, luaM_reallocv(L, NULL, 0, n, sizeof(t)))
#define luaM_newobject(L,tag,s)	luaM_realloc_(L, NULL, tag, (s))
#define luaM_growvector(L,v,nelems,size,t,limit,e) if ((nelems)+1 > (size)) ((v)=cast(t *, luaM_growaux_(L,v,&(size),sizeof(t),limit,e)))
#define luaM_reallocvector(L, v,oldn,n,t) ((v)=cast(t *, luaM_reallocv(L, v, oldn, n, sizeof(t))))
l_noret luaM_toobig (lua_State *L);
void *luaM_realloc_ (lua_State *L, void *block, size_t oldsize,
                                                          size_t size);
void *luaM_growaux_ (lua_State *L, void *block, int *size,
                               size_t size_elem, int limit,
#define LUA_TPROTO	LUA_NUMTAGS
#define LUA_TUPVAL	(LUA_NUMTAGS+1)
#define LUA_TDEADKEY	(LUA_NUMTAGS+2)
#define LUA_TOTALTAGS	(LUA_TUPVAL+2)
#define LUA_TLCL	(LUA_TFUNCTION | (0 << 4))  /* Lua closure */
#define LUA_TLCF	(LUA_TFUNCTION | (1 << 4))  /* light C function */
#define LUA_TCCL	(LUA_TFUNCTION | (2 << 4))  /* C closure */
#define LUA_TSHRSTR	(LUA_TSTRING | (0 << 4))  /* short strings */
#define LUA_TLNGSTR	(LUA_TSTRING | (1 << 4))  /* long strings */
#define numfield	lua_Number n;    /* numbers */
typedef struct lua_TValue TValue;
#define NILCONSTANT	{NULL}, LUA_TNIL
#define ttisnumber(o)		checktag((o), LUA_TNUMBER)
#define ttisnil(o)		checktag((o), LUA_TNIL)
#define ttisboolean(o)		checktag((o), LUA_TBOOLEAN)
#define ttislightuserdata(o)	checktag((o), LUA_TLIGHTUSERDATA)
#define ttisstring(o)		checktype((o), LUA_TSTRING)
#define ttisshrstring(o)	checktag((o), ctb(LUA_TSHRSTR))
#define ttislngstring(o)	checktag((o), ctb(LUA_TLNGSTR))
#define ttistable(o)		checktag((o), ctb(LUA_TTABLE))
#define ttisfunction(o)		checktype(o, LUA_TFUNCTION)
#define ttisclosure(o)		((rttype(o) & 0x1F) == LUA_TFUNCTION)
#define ttisCclosure(o)		checktag((o), ctb(LUA_TCCL))
#define ttisLclosure(o)		checktag((o), ctb(LUA_TLCL))
#define ttislcf(o)		checktag((o), LUA_TLCF)
#define ttisuserdata(o)		checktag((o), ctb(LUA_TUSERDATA))
#define ttisthread(o)		checktag((o), ctb(LUA_TTHREAD))
#define ttisdeadkey(o)		checktag((o), LUA_TDEADKEY)
	lua_longassert(!iscollectable(obj) || \
			(righttt(obj) && !isdead(g,gcvalue(obj))))
  { TValue *io=(obj); num_(io)=(x); settt_(io, LUA_TNUMBER); }
#define setnilvalue(obj) settt_(obj, LUA_TNIL)
  { TValue *io=(obj); val_(io).f=(x); settt_(io, LUA_TLCF); }
  { TValue *io=(obj); val_(io).p=(x); settt_(io, LUA_TLIGHTUSERDATA); }
  { TValue *io=(obj); val_(io).b=(x); settt_(io, LUA_TBOOLEAN); }
    val_(io).gc=cast(GCObject *, (x)); settt_(io, ctb(LUA_TUSERDATA)); \
    checkliveness(G(L),io); }
    val_(io).gc=cast(GCObject *, (x)); settt_(io, ctb(LUA_TTHREAD)); \
    checkliveness(G(L),io); }
    val_(io).gc=cast(GCObject *, (x)); settt_(io, ctb(LUA_TLCL)); \
    checkliveness(G(L),io); }
    val_(io).gc=cast(GCObject *, (x)); settt_(io, ctb(LUA_TCCL)); \
    checkliveness(G(L),io); }
    val_(io).gc=cast(GCObject *, (x)); settt_(io, ctb(LUA_TTABLE)); \
    checkliveness(G(L),io); }
#define setdeadvalue(obj)	settt_(obj, LUA_TDEADKEY)
#define luai_checknum(L,o,c)	{ /* empty */ }
#if defined(LUA_NANTRICK)
#if !defined(LUA_IEEEENDIAN)
#error option 'LUA_NANTRICK' needs 'LUA_IEEEENDIAN'
#endif
#if (LUA_IEEEENDIAN == 0)	/* { */
#define NILCONSTANT	{{{NULL}, tag2tt(LUA_TNIL)}}
#define NILCONSTANT	{{tag2tt(LUA_TNIL), {NULL}}}
#define rttype(o)	(ttisnumber(o) ? LUA_TNUMBER : tt_(o) & 0xff)
	{ TValue *io_=(obj); num_(io_)=(x); lua_assert(ttisnumber(io_)); }
#undef luai_checknum
#define luai_checknum(L,o,c)	{ if (!ttisnumber(o)) c; }
  lua_CFunction f; /* light C functions */
  numfield         /* numbers */
struct lua_TValue {
  TValuefields;
  lua_CFunction f;
  TValue upvalue[1];  /* list of upvalues */
#define luaO_nilobject		(&luaO_nilobject_)
const TValue luaO_nilobject_;
int luaO_int2fb (unsigned int x);
int luaO_fb2int (int x);
int luaO_ceillog2 (unsigned int x);
lua_Number luaO_arith (int op, lua_Number v1, lua_Number v2);
int luaO_str2d (const char *s, size_t len, lua_Number *result);
int luaO_hexavalue (int c);
const char *luaO_pushvfstring (lua_State *L, const char *fmt,
                                                       va_list argp);
const char *luaO_pushfstring (lua_State *L, const char *fmt, ...);
void luaO_chunkid (char *out, const char *source, size_t len);
#if SIZE_Bx < LUAI_BITSINT-1
#define MAXARG_Bx        ((1<<SIZE_Bx)-1)
#if SIZE_Ax < LUAI_BITSINT-1
#define MAXARG_Ax	((1<<SIZE_Ax)-1)
const lu_byte luaP_opmodes[NUM_OPCODES];
#define getOpMode(m)	(cast(enum OpMode, luaP_opmodes[m] & 3))
#define getBMode(m)	(cast(enum OpArgMask, (luaP_opmodes[m] >> 4) & 3))
#define getCMode(m)	(cast(enum OpArgMask, (luaP_opmodes[m] >> 2) & 3))
#define testAMode(m)	(luaP_opmodes[m] & (1 << 6))
#define testTMode(m)	(luaP_opmodes[m] & (1 << 7))
const char *const luaP_opnames[NUM_OPCODES+1];  /* opcode names */
    lua_Number nval;  /* for VKNUM */
  } u;
Closure *luaY_parser (lua_State *L, ZIO *z, Mbuffer *buff,
                                Dyndata *dyd, const char *name, int firstchar);
struct lua_longjmp;  /* defined in ldo.c */
#define BASIC_STACK_SIZE        (2*LUA_MINSTACK)
      lua_CFunction k;  /* continuation in case of yields */
      ptrdiff_t old_errfunc;
                                   luaV_execute of previous call */
#define CIST_YIELDED	(1<<3)	/* call reentered after suspension */
  lua_Alloc frealloc;  /* function to reallocate memory */
  void *ud;         /* auxiliary data to `frealloc' */
  lua_CFunction panic;  /* to be called in unprotected errors */
  struct lua_State *mainthread;
  const lua_Number *version;  /* pointer to version number */
  TString *memerrmsg;  /* memory-error message */
  struct Table *mt[LUA_NUMTAGS];  /* metatables for basic types */
} global_State;
struct lua_State {
  CommonHeader;
  lua_Hook hook;
  GCObject *openupval;  /* list of open upvalues in this stack */
  struct lua_longjmp *errorJmp;  /* current error recover point */
  ptrdiff_t errfunc;  /* current error handling function (stack index) */
  struct lua_State th;  /* thread */
};
	check_exp(novariant((o)->gch.tt) == LUA_TSTRING, &((o)->ts))
#define gco2ts(o)	(&rawgco2ts(o)->tsv)
#define rawgco2u(o)	check_exp((o)->gch.tt == LUA_TUSERDATA, &((o)->u))
#define gco2u(o)	(&rawgco2u(o)->uv)
#define gco2lcl(o)	check_exp((o)->gch.tt == LUA_TLCL, &((o)->cl.l))
#define gco2ccl(o)	check_exp((o)->gch.tt == LUA_TCCL, &((o)->cl.c))
#define gco2cl(o)  check_exp(novariant((o)->gch.tt) == LUA_TFUNCTION, &((o)->cl))
#define gco2t(o)	check_exp((o)->gch.tt == LUA_TTABLE, &((o)->h))
#define gco2p(o)	check_exp((o)->gch.tt == LUA_TPROTO, &((o)->p))
#define gco2uv(o)	check_exp((o)->gch.tt == LUA_TUPVAL, &((o)->uv))
#define gco2th(o)	check_exp((o)->gch.tt == LUA_TTHREAD, &((o)->th))
void luaE_setdebt (global_State *g, l_mem debt);
void luaE_freethread (lua_State *L, lua_State *L1);
CallInfo *luaE_extendCI (lua_State *L);
void luaE_freeCI (lua_State *L);
#define luaS_newliteral(L, s)	(luaS_newlstr(L, "" s, (sizeof(s)/sizeof(char))-1))
#define luaS_fix(s)	l_setbit((s)->tsv.marked, FIXEDBIT)
#define isreserved(s)	((s)->tsv.tt == LUA_TSHRSTR && (s)->tsv.extra > 0)
#define eqshrstr(a,b)	check_exp((a)->tsv.tt == LUA_TSHRSTR, (a) == (b))
unsigned int luaS_hash (const char *str, size_t l, unsigned int seed);
int luaS_eqlngstr (TString *a, TString *b);
int luaS_eqstr (TString *a, TString *b);
void luaS_resize (lua_State *L, int newsize);
Udata *luaS_newudata (lua_State *L, size_t s, Table *e);
TString *luaS_newlstr (lua_State *L, const char *str, size_t l);
TString *luaS_new (lua_State *L, const char *str);
const TValue *luaH_getint (Table *t, int key);
void luaH_setint (lua_State *L, Table *t, int key, TValue *value);
const TValue *luaH_getstr (Table *t, TString *key);
const TValue *luaH_get (Table *t, const TValue *key);
TValue *luaH_newkey (lua_State *L, Table *t, const TValue *key);
TValue *luaH_set (lua_State *L, Table *t, const TValue *key);
Table *luaH_new (lua_State *L);
void luaH_resize (lua_State *L, Table *t, int nasize, int nhsize);
void luaH_resizearray (lua_State *L, Table *t, int nasize);
void luaH_free (lua_State *L, Table *t);
int luaH_next (lua_State *L, Table *t, StkId key);
int luaH_getn (Table *t);
#if defined(LUA_DEBUG)
Node *luaH_mainposition (const Table *t, const TValue *key);
int luaH_isdummy (Node *n);
#endif
  ((et)->flags & (1u<<(e))) ? NULL : luaT_gettm(et, e, (g)->tmname[e]))
#define ttypename(x)	luaT_typenames_[(x) + 1]
#define objtypename(x)	ttypename(ttypenv(x))
const char *const luaT_typenames_[LUA_TOTALTAGS];
const TValue *luaT_gettm (Table *events, TMS event, TString *ename);
const TValue *luaT_gettmbyobj (lua_State *L, const TValue *o,
                                                       TMS event);
void luaT_init (lua_State *L);
#ifndef lua_h
#define lua_h
#define LUA_VERSION_MAJOR	"5"
#define LUA_VERSION_MINOR	"2"
#define LUA_VERSION_NUM		502
#define LUA_VERSION_RELEASE	"3"
#define LUA_VERSION	"Lua " LUA_VERSION_MAJOR "." LUA_VERSION_MINOR
#define LUA_RELEASE	LUA_VERSION "." LUA_VERSION_RELEASE
#define LUA_COPYRIGHT	LUA_RELEASE "  Copyright (C) 1994-2013 Lua.org, PUC-Rio"
#define LUA_AUTHORS	"R. Ierusalimschy, L. H. de Figueiredo, W. Celes"
#define LUA_SIGNATURE	"\033Lua"
#define LUA_MULTRET	(-1)
#define LUA_REGISTRYINDEX	LUAI_FIRSTPSEUDOIDX
#define lua_upvalueindex(i)	(LUA_REGISTRYINDEX - (i))
#define LUA_OK		0
#define LUA_YIELD	1
#define LUA_ERRRUN	2
#define LUA_ERRSYNTAX	3
#define LUA_ERRMEM	4
#define LUA_ERRGCMM	5
#define LUA_ERRERR	6
typedef struct lua_State lua_State;
typedef int (*lua_CFunction) (lua_State *L);
typedef const char * (*lua_Reader) (lua_State *L, void *ud, size_t *sz);
typedef int (*lua_Writer) (lua_State *L, const void* p, size_t sz, void* ud);
typedef void * (*lua_Alloc) (void *ud, void *ptr, size_t osize, size_t nsize);
#define LUA_TNONE		(-1)
#define LUA_TNIL		0
#define LUA_TBOOLEAN		1
#define LUA_TLIGHTUSERDATA	2
#define LUA_TNUMBER		3
#define LUA_TSTRING		4
#define LUA_TTABLE		5
#define LUA_TFUNCTION		6
#define LUA_TUSERDATA		7
#define LUA_TTHREAD		8
#define LUA_NUMTAGS		9
#define LUA_MINSTACK	20
#define LUA_RIDX_MAINTHREAD	1
#define LUA_RIDX_GLOBALS	2
#define LUA_RIDX_LAST		LUA_RIDX_GLOBALS
typedef LUA_NUMBER lua_Number;
typedef LUA_INTEGER lua_Integer;
typedef LUA_UNSIGNED lua_Unsigned;
#if defined(LUA_USER_H)
#include LUA_USER_H
#endif
extern const char lua_ident[];
lua_State *(lua_newstate) (lua_Alloc f, void *ud);
void       (lua_close) (lua_State *L);
lua_State *(lua_newthread) (lua_State *L);
lua_CFunction (lua_atpanic) (lua_State *L, lua_CFunction panicf);
const lua_Number *(lua_version) (lua_State *L);
int   (lua_absindex) (lua_State *L, int idx);
int   (lua_gettop) (lua_State *L);
void  (lua_settop) (lua_State *L, int idx);
void  (lua_pushvalue) (lua_State *L, int idx);
void  (lua_remove) (lua_State *L, int idx);
void  (lua_insert) (lua_State *L, int idx);
void  (lua_replace) (lua_State *L, int idx);
void  (lua_copy) (lua_State *L, int fromidx, int toidx);
int   (lua_checkstack) (lua_State *L, int sz);
void  (lua_xmove) (lua_State *from, lua_State *to, int n);
int             (lua_isnumber) (lua_State *L, int idx);
int             (lua_isstring) (lua_State *L, int idx);
int             (lua_iscfunction) (lua_State *L, int idx);
int             (lua_isuserdata) (lua_State *L, int idx);
int             (lua_type) (lua_State *L, int idx);
const char     *(lua_typename) (lua_State *L, int tp);
lua_Number      (lua_tonumberx) (lua_State *L, int idx, int *isnum);
lua_Integer     (lua_tointegerx) (lua_State *L, int idx, int *isnum);
lua_Unsigned    (lua_tounsignedx) (lua_State *L, int idx, int *isnum);
int             (lua_toboolean) (lua_State *L, int idx);
const char     *(lua_tolstring) (lua_State *L, int idx, size_t *len);
size_t          (lua_rawlen) (lua_State *L, int idx);
lua_CFunction   (lua_tocfunction) (lua_State *L, int idx);
void	       *(lua_touserdata) (lua_State *L, int idx);
lua_State      *(lua_tothread) (lua_State *L, int idx);
const void     *(lua_topointer) (lua_State *L, int idx);
#define LUA_OPADD	0	/* ORDER TM */
#define LUA_OPSUB	1
#define LUA_OPMUL	2
#define LUA_OPDIV	3
#define LUA_OPMOD	4
#define LUA_OPPOW	5
#define LUA_OPUNM	6
void  (lua_arith) (lua_State *L, int op);
#define LUA_OPEQ	0
#define LUA_OPLT	1
#define LUA_OPLE	2
int   (lua_rawequal) (lua_State *L, int idx1, int idx2);
int   (lua_compare) (lua_State *L, int idx1, int idx2, int op);
void        (lua_pushnil) (lua_State *L);
void        (lua_pushnumber) (lua_State *L, lua_Number n);
void        (lua_pushinteger) (lua_State *L, lua_Integer n);
void        (lua_pushunsigned) (lua_State *L, lua_Unsigned n);
const char *(lua_pushlstring) (lua_State *L, const char *s, size_t l);
const char *(lua_pushstring) (lua_State *L, const char *s);
const char *(lua_pushvfstring) (lua_State *L, const char *fmt,
                                                      va_list argp);
const char *(lua_pushfstring) (lua_State *L, const char *fmt, ...);
void  (lua_pushcclosure) (lua_State *L, lua_CFunction fn, int n);
void  (lua_pushboolean) (lua_State *L, int b);
void  (lua_pushlightuserdata) (lua_State *L, void *p);
int   (lua_pushthread) (lua_State *L);
void  (lua_getglobal) (lua_State *L, const char *var);
void  (lua_gettable) (lua_State *L, int idx);
void  (lua_getfield) (lua_State *L, int idx, const char *k);
void  (lua_rawget) (lua_State *L, int idx);
void  (lua_rawgeti) (lua_State *L, int idx, int n);
void  (lua_rawgetp) (lua_State *L, int idx, const void *p);
void  (lua_createtable) (lua_State *L, int narr, int nrec);
void *(lua_newuserdata) (lua_State *L, size_t sz);
int   (lua_getmetatable) (lua_State *L, int objindex);
void  (lua_getuservalue) (lua_State *L, int idx);
void  (lua_setglobal) (lua_State *L, const char *var);
void  (lua_settable) (lua_State *L, int idx);
void  (lua_setfield) (lua_State *L, int idx, const char *k);
void  (lua_rawset) (lua_State *L, int idx);
void  (lua_rawseti) (lua_State *L, int idx, int n);
void  (lua_rawsetp) (lua_State *L, int idx, const void *p);
int   (lua_setmetatable) (lua_State *L, int objindex);
void  (lua_setuservalue) (lua_State *L, int idx);
void  (lua_callk) (lua_State *L, int nargs, int nresults, int ctx,
                           lua_CFunction k);
#define lua_call(L,n,r)		lua_callk(L, (n), (r), 0, NULL)
int   (lua_getctx) (lua_State *L, int *ctx);
int   (lua_pcallk) (lua_State *L, int nargs, int nresults, int errfunc,
                            int ctx, lua_CFunction k);
#define lua_pcall(L,n,r,f)	lua_pcallk(L, (n), (r), (f), 0, NULL)
int   (lua_load) (lua_State *L, lua_Reader reader, void *dt,
                                        const char *chunkname,
int (lua_dump) (lua_State *L, lua_Writer writer, void *data);
int  (lua_yieldk) (lua_State *L, int nresults, int ctx,
                           lua_CFunction k);
#define lua_yield(L,n)		lua_yieldk(L, (n), 0, NULL)
int  (lua_resume) (lua_State *L, lua_State *from, int narg);
int  (lua_status) (lua_State *L);
#define LUA_GCSTOP		0
#define LUA_GCRESTART		1
#define LUA_GCCOLLECT		2
#define LUA_GCCOUNT		3
#define LUA_GCCOUNTB		4
#define LUA_GCSTEP		5
#define LUA_GCSETPAUSE		6
#define LUA_GCSETSTEPMUL	7
#define LUA_GCSETMAJORINC	8
#define LUA_GCISRUNNING		9
#define LUA_GCGEN		10
#define LUA_GCINC		11
int (lua_gc) (lua_State *L, int what, int data);
int   (lua_error) (lua_State *L);
int   (lua_next) (lua_State *L, int idx);
void  (lua_concat) (lua_State *L, int n);
void  (lua_len)    (lua_State *L, int idx);
lua_Alloc (lua_getallocf) (lua_State *L, void **ud);
void      (lua_setallocf) (lua_State *L, lua_Alloc f, void *ud);
#define lua_tonumber(L,i)	lua_tonumberx(L,i,NULL)
#define lua_tointeger(L,i)	lua_tointegerx(L,i,NULL)
#define lua_tounsigned(L,i)	lua_tounsignedx(L,i,NULL)
#define lua_pop(L,n)		lua_settop(L, -(n)-1)
#define lua_newtable(L)		lua_createtable(L, 0, 0)
#define lua_register(L,n,f) (lua_pushcfunction(L, (f)), lua_setglobal(L, (n)))
#define lua_pushcfunction(L,f)	lua_pushcclosure(L, (f), 0)
#define lua_isfunction(L,n)	(lua_type(L, (n)) == LUA_TFUNCTION)
#define lua_istable(L,n)	(lua_type(L, (n)) == LUA_TTABLE)
#define lua_islightuserdata(L,n)	(lua_type(L, (n)) == LUA_TLIGHTUSERDATA)
#define lua_isnil(L,n)		(lua_type(L, (n)) == LUA_TNIL)
#define lua_isboolean(L,n)	(lua_type(L, (n)) == LUA_TBOOLEAN)
#define lua_isthread(L,n)	(lua_type(L, (n)) == LUA_TTHREAD)
#define lua_isnone(L,n)		(lua_type(L, (n)) == LUA_TNONE)
#define lua_isnoneornil(L, n)	(lua_type(L, (n)) <= 0)
#define lua_pushliteral(L, s)	lua_pushlstring(L, "" s, (sizeof(s)/sizeof(char))-1)
#define lua_pushglobaltable(L)  lua_rawgeti(L, LUA_REGISTRYINDEX, LUA_RIDX_GLOBALS)
#define lua_tostring(L,i)	lua_tolstring(L, (i), NULL)
#define LUA_HOOKCALL	0
#define LUA_HOOKRET	1
#define LUA_HOOKLINE	2
#define LUA_HOOKCOUNT	3
#define LUA_HOOKTAILCALL 4
#define LUA_MASKCALL	(1 << LUA_HOOKCALL)
#define LUA_MASKRET	(1 << LUA_HOOKRET)
#define LUA_MASKLINE	(1 << LUA_HOOKLINE)
#define LUA_MASKCOUNT	(1 << LUA_HOOKCOUNT)
typedef struct lua_Debug lua_Debug;  /* activation record */
typedef void (*lua_Hook) (lua_State *L, lua_Debug *ar);
int (lua_getstack) (lua_State *L, int level, lua_Debug *ar);
int (lua_getinfo) (lua_State *L, const char *what, lua_Debug *ar);
const char *(lua_getlocal) (lua_State *L, const lua_Debug *ar, int n);
const char *(lua_setlocal) (lua_State *L, const lua_Debug *ar, int n);
const char *(lua_getupvalue) (lua_State *L, int funcindex, int n);
const char *(lua_setupvalue) (lua_State *L, int funcindex, int n);
void *(lua_upvalueid) (lua_State *L, int fidx, int n);
void  (lua_upvaluejoin) (lua_State *L, int fidx1, int n1,
                                               int fidx2, int n2);
int (lua_sethook) (lua_State *L, lua_Hook func, int mask, int count);
lua_Hook (lua_gethook) (lua_State *L);
int (lua_gethookmask) (lua_State *L);
int (lua_gethookcount) (lua_State *L);
struct lua_Debug {
  int event;
  char short_src[LUA_IDSIZE]; /* (S) */
  /* private part */
#if !defined(LUA_ANSI) && defined(__STRICT_ANSI__)
#define LUA_ANSI
#endif
#if !defined(LUA_ANSI) && defined(_WIN32) && !defined(_WIN32_WCE)
#define LUA_WIN		/* enable goodies for regular Windows platforms */
#endif
#if defined(LUA_WIN)
#define LUA_DL_DLL
#define LUA_USE_AFORMAT		/* assume 'printf' handles 'aA' specifiers */
#endif
#if defined(LUA_USE_LINUX)
#define LUA_USE_POSIX
#define LUA_USE_DLOPEN		/* needs an extra library: -ldl */
#define LUA_USE_READLINE	/* needs some extra libraries */
#define LUA_USE_STRTODHEX	/* assume 'strtod' handles hex formats */
#define LUA_USE_AFORMAT		/* assume 'printf' handles 'aA' specifiers */
#define LUA_USE_LONGLONG	/* assume support for long long */
#endif
#if defined(LUA_USE_MACOSX)
#define LUA_USE_POSIX
#define LUA_USE_DLOPEN		/* does not need -ldl */
#define LUA_USE_READLINE	/* needs an extra library: -lreadline */
#define LUA_USE_STRTODHEX	/* assume 'strtod' handles hex formats */
#define LUA_USE_AFORMAT		/* assume 'printf' handles 'aA' specifiers */
#define LUA_USE_LONGLONG	/* assume support for long long */
#endif
@* Interfaces Extension (XSI).
#if defined(LUA_USE_POSIX)
#define LUA_USE_MKSTEMP
#define LUA_USE_ISATTY
#define LUA_USE_POPEN
#define LUA_USE_ULONGJMP
#define LUA_USE_GMTIME_R
#endif
@* Lua libraries.
@* C libraries.
#define LUA_LDIR	"!\\lua\\"
#define LUA_CDIR	"!\\"
#define LUA_PATH_DEFAULT  LUA_LDIR"?.lua;"  LUA_LDIR"?\\init.lua;" LUA_CDIR"?.lua;"  LUA_CDIR"?\\init.lua;" ".\\?.lua"
#define LUA_CPATH_DEFAULT LUA_CDIR"?.dll;" LUA_CDIR"loadall.dll;" ".\\?.dll"
#define LUA_VDIR	LUA_VERSION_MAJOR "." LUA_VERSION_MINOR "/"
#define LUA_ROOT	"/usr/local/"
#define LUA_LDIR	LUA_ROOT "share/lua/" LUA_VDIR
#define LUA_CDIR	LUA_ROOT "lib/lua/" LUA_VDIR
#define LUA_PATH_DEFAULT  LUA_LDIR"?.lua;"  LUA_LDIR"?/init.lua;" LUA_CDIR"?.lua;"  LUA_CDIR"?/init.lua;" "./?.lua"
#define LUA_CPATH_DEFAULT LUA_CDIR"?.so;" LUA_CDIR"loadall.so;" "./?.so"
#endif			/* } */
#define LUA_DIRSEP	"\\"
#else
#define LUA_DIRSEP	"/"
#endif
#define LUA_ENV		"_ENV"
#if defined(LUA_BUILD_AS_DLL)	/* { */
#if defined(LUA_CORE) || defined(LUA_LIB)	/* { */
#define LUA_API __declspec(dllexport)
#else						/* }{ */
#define LUA_API __declspec(dllimport)
#endif						/* } */
#define LUA_API		extern
#define LUALIB_API	LUA_API
#define LUAMOD_API	LUALIB_API
@* exported to outside modules.
@* that are not to be exported to outside modules (LUAI_DDEF for
@* definitions and LUAI_DDEC for declarations).
#define LUAI_FUNC	__attribute__((visibility("hidden"))) extern
#define LUAI_DDEC	LUAI_FUNC
#define LUAI_DDEF	/* empty */
#define LUAI_FUNC	extern
#define LUAI_DDEC	extern
#define LUAI_DDEF	/* empty */
#endif				/* } */
#define LUA_QL(x)	"'" x "'"
#define LUA_QS		LUA_QL("%s")
@* of a function in debug information.
#define LUA_IDSIZE	60
#if defined(LUA_LIB) || defined(lua_c)
#include <stdio.h>
#define luai_writestring(s,l)	fwrite((s), sizeof(char), (l), stdout)
#define luai_writeline()	(luai_writestring("\n", 1), fflush(stdout))
#endif
#define luai_writestringerror(s,p) (fprintf(stderr, (s), (p)), fflush(stderr))
#define LUAI_MAXSHORTLEN        40
#if defined(LUA_COMPAT_ALL)	/* { */
#define LUA_COMPAT_UNPACK
#define LUA_COMPAT_LOADERS
#define lua_cpcall(L,f,u)  (lua_pushcfunction(L, (f)), lua_pushlightuserdata(L,(u)), lua_pcall(L,1,0,0))
#define LUA_COMPAT_LOG10
#define LUA_COMPAT_LOADSTRING
#define LUA_COMPAT_MAXN
#define lua_strlen(L,i)		lua_rawlen(L, (i))
#define lua_objlen(L,i)		lua_rawlen(L, (i))
#define lua_equal(L,idx1,idx2)		lua_compare(L,(idx1),(idx2),LUA_OPEQ)
#define lua_lessthan(L,idx1,idx2)	lua_compare(L,(idx1),(idx2),LUA_OPLT)
#define LUA_COMPAT_MODULE
#define LUAI_BITSINT	16
#elif INT_MAX > 2147483640L	/* }{ */
#define LUAI_BITSINT	32
#else				/* }{ */
#error "you must define LUA_BITSINT with number of bits in an integer"
#endif				/* } */
@* memory used by Lua.
@* used by Lua.
#if LUAI_BITSINT >= 32		/* { */
#define LUA_INT32	int
#define LUAI_UMEM	size_t
#define LUAI_MEM	ptrdiff_t
#else				/* }{ */
#define LUA_INT32	long
#define LUAI_UMEM	unsigned long
#define LUAI_MEM	long
#endif				/* } */
#if LUAI_BITSINT >= 32
#define LUAI_MAXSTACK		1000000
#else
#define LUAI_MAXSTACK		15000
#endif
#define LUAI_FIRSTPSEUDOIDX	(-LUAI_MAXSTACK - 1000)
#define LUAL_BUFFERSIZE		BUFSIZ
#define LUA_NUMBER_DOUBLE
#define LUA_NUMBER	double
@* over a number.
#define LUAI_UACNUMBER	double
#define LUA_NUMBER_SCAN		"%lf"
#define LUA_NUMBER_FMT		"%.14g"
#define lua_number2str(s,n)	sprintf((s), LUA_NUMBER_FMT, (n))
#define LUAI_MAXNUMBER2STR	32 /* 16 digits, sign, point, and \0 */
#define lua_str2number(s,p)	strtod((s), (p))
#if defined(LUA_USE_STRTODHEX)
#define lua_strx2number(s,p)	strtod((s), (p))
#endif
#define luai_nummod(L,a,b)	((a) - l_mathop(floor)((a)/(b))*(b))
#define luai_numpow(L,a,b)	(l_mathop(pow)(a,b))
#endif
#if defined(LUA_CORE)
#define luai_numadd(L,a,b)	((a)+(b))
#define luai_numsub(L,a,b)	((a)-(b))
#define luai_nummul(L,a,b)	((a)*(b))
#define luai_numdiv(L,a,b)	((a)/(b))
#define luai_numunm(L,a)	(-(a))
#define luai_numeq(a,b)		((a)==(b))
#define luai_numlt(L,a,b)	((a)<(b))
#define luai_numle(L,a,b)	((a)<=(b))
#define luai_numisnan(L,a)	(!luai_numeq((a), (a)))
#endif
#define LUA_INTEGER	ptrdiff_t
#define LUA_UNSIGNED	unsigned LUA_INT32
#if defined(LUA_NUMBER_DOUBLE) && !defined(LUA_ANSI)	/* { */
#if defined(LUA_WIN) && defined(_MSC_VER) && defined(_M_IX86)	/* { */
#define LUA_MSASMTRICK
#define LUA_IEEEENDIAN		0
#define LUA_NANTRICK
#define LUA_IEEE754TRICK
#define LUA_IEEELL
#define LUA_IEEEENDIAN		0
#define LUA_NANTRICK
#define LUA_IEEE754TRICK
#define LUA_IEEEENDIAN		0
#define LUA_IEEE754TRICK
#define LUA_IEEEENDIAN		1
#define LUA_IEEE754TRICK
int (luaopen_base) (lua_State *L);
#define LUA_COLIBNAME	"coroutine"
int (luaopen_coroutine) (lua_State *L);
#define LUA_TABLIBNAME	"table"
int (luaopen_table) (lua_State *L);
#define LUA_IOLIBNAME	"io"
int (luaopen_io) (lua_State *L);
#define LUA_OSLIBNAME	"os"
int (luaopen_os) (lua_State *L);
#define LUA_STRLIBNAME	"string"
int (luaopen_string) (lua_State *L);
#define LUA_BITLIBNAME	"bit32"
int (luaopen_bit32) (lua_State *L);
#define LUA_MATHLIBNAME	"math"
int (luaopen_math) (lua_State *L);
#define LUA_DBLIBNAME	"debug"
int (luaopen_debug) (lua_State *L);
#define LUA_LOADLIBNAME	"package"
int (luaopen_package) (lua_State *L);
void (luaL_openlibs) (lua_State *L);
#if !defined(lua_assert)
#define lua_assert(x)	((void)0)
#endif
Closure* luaU_undump (lua_State* L, ZIO* Z, Mbuffer* buff, const char* name);
void luaU_header (lu_byte* h);
int luaU_dump (lua_State* L, const Proto* f, lua_Writer w, void* data, int strip);
#define LUAC_TAIL		"\x19\x93\r\n\x1a\n"
#define LUAC_HEADERSIZE		(sizeof(LUA_SIGNATURE)-sizeof(char)+2+6+sizeof(LUAC_TAIL)-sizeof(char))
#define tostring(L,o) (ttisstring(o) || (luaV_tostring(L, o)))
#define tonumber(o,n)	(ttisnumber(o) || (((o) = luaV_tonumber(o,n)) != NULL))
#define equalobj(L,o1,o2)  (ttisequal(o1, o2) && luaV_equalobj_(L, o1, o2))
#define luaV_rawequalobj(o1,o2)		equalobj(NULL,o1,o2)
int luaV_equalobj_ (lua_State *L, const TValue *t1, const TValue *t2);
int luaV_lessthan (lua_State *L, const TValue *l, const TValue *r);
int luaV_lessequal (lua_State *L, const TValue *l, const TValue *r);
const TValue *luaV_tonumber (const TValue *obj, TValue *n);
int luaV_tostring (lua_State *L, StkId obj);
void luaV_gettable (lua_State *L, const TValue *t, TValue *key,
                                            StkId val);
void luaV_settable (lua_State *L, const TValue *t, TValue *key,
                                            StkId val);
void luaV_finishOp (lua_State *L);
void luaV_execute (lua_State *L);
void luaV_concat (lua_State *L, int total);
void luaV_arith (lua_State *L, StkId ra, const TValue *rb,
                           const TValue *rc, TMS op);
void luaV_objlen (lua_State *L, StkId ra, const TValue *rb);
#define zgetc(z)  (((z)->n--)>0 ?  cast_uchar(*(z)->p++) : luaZ_fill(z))
#define luaZ_initbuffer(L, buff) ((buff)->buffer = NULL, (buff)->buffsize = 0)
#define luaZ_buffer(buff)	((buff)->buffer)
#define luaZ_sizebuffer(buff)	((buff)->buffsize)
#define luaZ_bufflen(buff)	((buff)->n)
#define luaZ_resetbuffer(buff) ((buff)->n = 0)
#define luaZ_resizebuffer(L, buff, size) (luaM_reallocvector(L, (buff)->buffer, (buff)->buffsize, size, char), (buff)->buffsize = size)
#define luaZ_freebuffer(L, buff)	luaZ_resizebuffer(L, buff, 0)
char *luaZ_openspace (lua_State *L, Mbuffer *buff, size_t n);
void luaZ_init (lua_State *L, ZIO *z, lua_Reader reader,
                                        void *data);
size_t luaZ_read (ZIO* z, void* b, size_t n);	/* read next n bytes */
  lua_Reader reader;		/* reader function */
  void* data;			/* additional data */
  lua_State *L;			/* Lua state (for reader) */
};
int luaZ_fill (ZIO *z);
