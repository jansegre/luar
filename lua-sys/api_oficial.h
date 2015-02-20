int lua_absindex (lua_State *L, int idx);
typedef void * (*lua_Alloc) (void *ud,
                             void *ptr,
                             size_t osize,
                             size_t nsize);
void lua_arith (lua_State *L, int op);
lua_CFunction lua_atpanic (lua_State *L, lua_CFunction panicf);
void lua_call (lua_State *L, int nargs, int nresults);
void lua_callk (lua_State *L, int nargs, int nresults, int ctx,
                lua_CFunction k);
typedef int (*lua_CFunction) (lua_State *L);
int lua_checkstack (lua_State *L, int extra);
void lua_close (lua_State *L);
int lua_compare (lua_State *L, int index1, int index2, int op);
void lua_concat (lua_State *L, int n);
void lua_copy (lua_State *L, int fromidx, int toidx);
void lua_createtable (lua_State *L, int narr, int nrec);
int lua_dump (lua_State *L, lua_Writer writer, void *data);
int lua_error (lua_State *L);
int lua_gc (lua_State *L, int what, int data);
lua_Alloc lua_getallocf (lua_State *L, void **ud);
int lua_getctx (lua_State *L, int *ctx);
void lua_getfield (lua_State *L, int index, const char *k);
void lua_getglobal (lua_State *L, const char *name);
int lua_getmetatable (lua_State *L, int index);
void lua_gettable (lua_State *L, int index);
int lua_gettop (lua_State *L);
void lua_getuservalue (lua_State *L, int index);
void lua_insert (lua_State *L, int index);
typedef ptrdiff_t lua_Integer;
int lua_isboolean (lua_State *L, int index);
int lua_iscfunction (lua_State *L, int index);
int lua_isfunction (lua_State *L, int index);
int lua_islightuserdata (lua_State *L, int index);
int lua_isnil (lua_State *L, int index);
int lua_isnone (lua_State *L, int index);
int lua_isnoneornil (lua_State *L, int index);
int lua_isnumber (lua_State *L, int index);
int lua_isstring (lua_State *L, int index);
int lua_istable (lua_State *L, int index);
int lua_isthread (lua_State *L, int index);
int lua_isuserdata (lua_State *L, int index);
void lua_len (lua_State *L, int index);
int lua_load (lua_State *L,
              lua_Reader reader,
              void *data,
              const char *source,
              const char *mode);
lua_State *lua_newstate (lua_Alloc f, void *ud);
void lua_newtable (lua_State *L);
lua_State *lua_newthread (lua_State *L);
void *lua_newuserdata (lua_State *L, size_t size);
int lua_next (lua_State *L, int index);
typedef double lua_Number;
int lua_pcall (lua_State *L, int nargs, int nresults, int msgh);
int lua_pcallk (lua_State *L,
                int nargs,
                int nresults,
                int errfunc,
                int ctx,
                lua_CFunction k);
void lua_pop (lua_State *L, int n);
void lua_pushboolean (lua_State *L, int b);
void lua_pushcclosure (lua_State *L, lua_CFunction fn, int n);
void lua_pushcfunction (lua_State *L, lua_CFunction f);
const char *lua_pushfstring (lua_State *L, const char *fmt, ...);
void lua_pushglobaltable (lua_State *L);
void lua_pushinteger (lua_State *L, lua_Integer n);
void lua_pushlightuserdata (lua_State *L, void *p);
const char *lua_pushliteral (lua_State *L, const char *s);
const char *lua_pushlstring (lua_State *L, const char *s, size_t len);
void lua_pushnil (lua_State *L);
void lua_pushnumber (lua_State *L, lua_Number n);
const char *lua_pushstring (lua_State *L, const char *s);
int lua_pushthread (lua_State *L);
void lua_pushunsigned (lua_State *L, lua_Unsigned n);
void lua_pushvalue (lua_State *L, int index);
const char *lua_pushvfstring (lua_State *L,
                              const char *fmt,
                              va_list argp);
int lua_rawequal (lua_State *L, int index1, int index2);
void lua_rawget (lua_State *L, int index);
void lua_rawgeti (lua_State *L, int index, int n);
void lua_rawgetp (lua_State *L, int index, const void *p);
size_t lua_rawlen (lua_State *L, int index);
void lua_rawset (lua_State *L, int index);
void lua_rawseti (lua_State *L, int index, int n);
void lua_rawsetp (lua_State *L, int index, const void *p);
typedef const char * (*lua_Reader) (lua_State *L,
                                    void *data,
                                    size_t *size);
void lua_register (lua_State *L, const char *name, lua_CFunction f);
void lua_remove (lua_State *L, int index);
void lua_replace (lua_State *L, int index);
int lua_resume (lua_State *L, lua_State *from, int nargs);
void lua_setallocf (lua_State *L, lua_Alloc f, void *ud);
void lua_setfield (lua_State *L, int index, const char *k);
void lua_setglobal (lua_State *L, const char *name);
void lua_setmetatable (lua_State *L, int index);
void lua_settable (lua_State *L, int index);
void lua_settop (lua_State *L, int index);
void lua_setuservalue (lua_State *L, int index);
typedef struct lua_State lua_State;
int lua_status (lua_State *L);
int lua_toboolean (lua_State *L, int index);
lua_CFunction lua_tocfunction (lua_State *L, int index);
lua_Integer lua_tointeger (lua_State *L, int index);
lua_Integer lua_tointegerx (lua_State *L, int index, int *isnum);
const char *lua_tolstring (lua_State *L, int index, size_t *len);
lua_Number lua_tonumber (lua_State *L, int index);
lua_Number lua_tonumberx (lua_State *L, int index, int *isnum);
const void *lua_topointer (lua_State *L, int index);
const char *lua_tostring (lua_State *L, int index);
lua_State *lua_tothread (lua_State *L, int index);
lua_Unsigned lua_tounsigned (lua_State *L, int index);
lua_Unsigned lua_tounsignedx (lua_State *L, int index, int *isnum);
void *lua_touserdata (lua_State *L, int index);
int lua_type (lua_State *L, int index);
const char *lua_typename (lua_State *L, int tp);
typedef unsigned long lua_Unsigned;
int lua_upvalueindex (int i);
const lua_Number *lua_version (lua_State *L);
typedef int (*lua_Writer) (lua_State *L,
                           const void* p,
                           size_t sz,
                           void* ud);
void lua_xmove (lua_State *from, lua_State *to, int n);
int lua_yield (lua_State *L, int nresults);
int lua_yieldk (lua_State *L, int nresults, int ctx, lua_CFunction k);
typedef struct lua_Debug {
  int event;
  const char *name;           /* (n) */
  const char *namewhat;       /* (n) */
  const char *what;           /* (S) */
  const char *source;         /* (S) */
  int currentline;            /* (l) */
  int linedefined;            /* (S) */
  int lastlinedefined;        /* (S) */
  unsigned char nups;         /* (u) number of upvalues */
  unsigned char nparams;      /* (u) number of parameters */
  char isvararg;              /* (u) */
  char istailcall;            /* (t) */
  char short_src[LUA_IDSIZE]; /* (S) */
  /* private part */
  other fields
} lua_Debug;
lua_Hook lua_gethook (lua_State *L);
int lua_gethookcount (lua_State *L);
int lua_gethookmask (lua_State *L);
int lua_getinfo (lua_State *L, const char *what, lua_Debug *ar);
const char *lua_getlocal (lua_State *L, lua_Debug *ar, int n);
int lua_getstack (lua_State *L, int level, lua_Debug *ar);
const char *lua_getupvalue (lua_State *L, int funcindex, int n);
typedef void (*lua_Hook) (lua_State *L, lua_Debug *ar);
int lua_sethook (lua_State *L, lua_Hook f, int mask, int count);
const char *lua_setlocal (lua_State *L, lua_Debug *ar, int n);
const char *lua_setupvalue (lua_State *L, int funcindex, int n);
void *lua_upvalueid (lua_State *L, int funcindex, int n);
void lua_upvaluejoin (lua_State *L, int funcindex1, int n1,
                                    int funcindex2, int n2);
void luaL_addchar (luaL_Buffer *B, char c);
void luaL_addlstring (luaL_Buffer *B, const char *s, size_t l);
void luaL_addsize (luaL_Buffer *B, size_t n);
void luaL_addstring (luaL_Buffer *B, const char *s);
void luaL_addvalue (luaL_Buffer *B);
void luaL_argcheck (lua_State *L,
                    int cond,
                    int arg,
                    const char *extramsg);
int luaL_argerror (lua_State *L, int arg, const char *extramsg);
typedef struct luaL_Buffer luaL_Buffer;
void luaL_buffinit (lua_State *L, luaL_Buffer *B);
char *luaL_buffinitsize (lua_State *L, luaL_Buffer *B, size_t sz);
int luaL_callmeta (lua_State *L, int obj, const char *e);
void luaL_checkany (lua_State *L, int arg);
int luaL_checkint (lua_State *L, int arg);
lua_Integer luaL_checkinteger (lua_State *L, int arg);
long luaL_checklong (lua_State *L, int arg);
const char *luaL_checklstring (lua_State *L, int arg, size_t *l);
lua_Number luaL_checknumber (lua_State *L, int arg);
int luaL_checkoption (lua_State *L,
                      int arg,
                      const char *def,
                      const char *const lst[]);
void luaL_checkstack (lua_State *L, int sz, const char *msg);
const char *luaL_checkstring (lua_State *L, int arg);
void luaL_checktype (lua_State *L, int arg, int t);
void *luaL_checkudata (lua_State *L, int arg, const char *tname);
lua_Unsigned luaL_checkunsigned (lua_State *L, int arg);
void luaL_checkversion (lua_State *L);
int luaL_dofile (lua_State *L, const char *filename);
int luaL_dostring (lua_State *L, const char *str);
int luaL_error (lua_State *L, const char *fmt, ...);
int luaL_execresult (lua_State *L, int stat);
int luaL_fileresult (lua_State *L, int stat, const char *fname);
int luaL_getmetafield (lua_State *L, int obj, const char *e);
void luaL_getmetatable (lua_State *L, const char *tname);
int luaL_getsubtable (lua_State *L, int idx, const char *fname);
const char *luaL_gsub (lua_State *L,
                       const char *s,
                       const char *p,
                       const char *r);
int luaL_len (lua_State *L, int index);
int luaL_loadbuffer (lua_State *L,
                     const char *buff,
                     size_t sz,
                     const char *name);
int luaL_loadbufferx (lua_State *L,
                      const char *buff,
                      size_t sz,
                      const char *name,
                      const char *mode);
int luaL_loadfile (lua_State *L, const char *filename);
int luaL_loadfilex (lua_State *L, const char *filename,
                                            const char *mode);
int luaL_loadstring (lua_State *L, const char *s);
void luaL_newlib (lua_State *L, const luaL_Reg *l);
void luaL_newlibtable (lua_State *L, const luaL_Reg l[]);
int luaL_newmetatable (lua_State *L, const char *tname);
lua_State *luaL_newstate (void);
void luaL_openlibs (lua_State *L);
int luaL_optint (lua_State *L, int arg, int d);
lua_Integer luaL_optinteger (lua_State *L,
                             int arg,
                             lua_Integer d);
long luaL_optlong (lua_State *L, int arg, long d);
const char *luaL_optlstring (lua_State *L,
                             int arg,
                             const char *d,
                             size_t *l);
lua_Number luaL_optnumber (lua_State *L, int arg, lua_Number d);
const char *luaL_optstring (lua_State *L,
                            int arg,
                            const char *d);
lua_Unsigned luaL_optunsigned (lua_State *L,
                               int arg,
                               lua_Unsigned u);
char *luaL_prepbuffer (luaL_Buffer *B);
char *luaL_prepbuffsize (luaL_Buffer *B, size_t sz);
void luaL_pushresult (luaL_Buffer *B);
void luaL_pushresultsize (luaL_Buffer *B, size_t sz);
int luaL_ref (lua_State *L, int t);
typedef struct luaL_Reg {
  const char *name;
  lua_CFunction func;
} luaL_Reg;
void luaL_requiref (lua_State *L, const char *modname,
                    lua_CFunction openf, int glb);
void luaL_setfuncs (lua_State *L, const luaL_Reg *l, int nup);
void luaL_setmetatable (lua_State *L, const char *tname);
void *luaL_testudata (lua_State *L, int arg, const char *tname);
const char *luaL_tolstring (lua_State *L, int idx, size_t *len);
void luaL_traceback (lua_State *L, lua_State *L1, const char *msg,
                     int level);
const char *luaL_typename (lua_State *L, int index);
void luaL_unref (lua_State *L, int t, int ref);
void luaL_where (lua_State *L, int lvl);
