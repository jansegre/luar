#include <lua.h>
#include <lualib.h>
#include <lauxlib.h>

// TODO fix char escaping
#define i(VAR) printf("pub const %s: c_int = %i;\n", #VAR, VAR);
#define s(VAR) printf("pub const %s: &'static str = \"%s\";\n", #VAR, VAR);

int main() {
  #include "defs.h"
  return 0;
}
