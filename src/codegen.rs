use ast::*;
use std::io::Write;
use std::io::Error as IoError;
use std::collections::HashMap;
use std::collections::hash_map::Entry::*;
use std::fmt;
use self::Value::*;

#[derive(Debug)]
pub enum Error {
    FnNameError,
    VarNameError,
    ArgNameError,
    SetMismatchError,
    MultipleConstError,
    MultipleLabelError,
    IoError(IoError),
    NotSupported,
}

impl From<IoError> for Error {
    fn from(other: IoError) -> Self {
        Error::IoError(other)
    }
}

#[derive(Debug)]
pub struct Codegen<'w, W: Write + 'w> {
    fn_count: usize,
    fn_map: HashMap<(V<Name>, Option<Name>), usize>,
    const_count: usize,
    const_map: HashMap<Value, usize>,
    out: &'w mut W,
    label_count: usize,
    label_map: HashMap<Name, usize>,
}

impl<'w, W: Write + 'w> Codegen<'w, W> {
    pub fn new(out: &'w mut W) -> Codegen<'w, W> {
        let mut fn_map = HashMap::new();
        fn_map.insert((vec![].into_boxed_slice(), None), 0);
        Codegen {
            fn_count: 1,
            fn_map: fn_map,
            const_count: 0,
            const_map: HashMap::new(),
            out: out,
            label_count: 0,
            label_map: HashMap::new(),
        }
    }

    fn get_const_map(&mut self) -> (&mut HashMap<Value, usize>, &mut usize) {
        (&mut self.const_map, &mut self.const_count)
    }

    fn get_label_map(&mut self) -> (&mut HashMap<Name, usize>, &mut usize) {
        (&mut self.label_map, &mut self.label_count)
    }

    fn declare_fn(&mut self, vname: V<Name>, oname: Option<Name>) -> Result<(), Error> {
        match self.fn_map.entry((vname, oname)) {
            Occupied(..) => {
                // TODO: improve error report
                Err(Error::FnNameError)
            }
            Vacant(entry) => {
                entry.insert(self.fn_count);
                self.fn_count += 1;
                Ok(())
            }
        }
    }

    fn fill_fn_map(&mut self, block: &Block) -> Result<(), Error> {
        for stmt in block.stmts.iter() {
            match *stmt {
                StmtFunction(ref vname, ref oname, _, _, ref block) => {
                    try!(self.declare_fn(vname.clone(), oname.clone()));
                    try!(self.fill_fn_map(block));
                }
                StmtLocalFunction(ref name, _, _, ref block) => {
                    try!(self.declare_fn(vec![name.clone()].into_boxed_slice(), None));
                    try!(self.fill_fn_map(block));
                }
                // TODO: detect functions declared with StmtSet maybe?
                StmtIf(_, ref block, ref velseif, ref velse) => {
                    try!(self.fill_fn_map(block));
                    for &(_, ref block) in velseif.iter() {
                        try!(self.fill_fn_map(block));
                    }
                    if let &Some(ref block) = velse {
                        try!(self.fill_fn_map(block));
                    }
                }
                StmtDo(ref block) | StmtWhile(_, ref block) | StmtRepeat(ref block, _) | StmtForNum(_, _, _, _, ref block) | StmtForIn(_, _, ref block) => try!(self.fill_fn_map(block)),
                _ => {}
            }
        }

        Ok(())
    }

    pub fn gen_from(&mut self, chunk: Chunk) -> Result<(), Error> {
        try!(self.fill_fn_map(&chunk.block));

        let block = chunk.block;
        try!(block.fun_gen_to(self, 0, vec![].into_boxed_slice()));

        let mut rev_const_map = HashMap::new();
        for (cval, cid) in &self.const_map {
            if let Some(_) = rev_const_map.insert(cid, cval) {
                return Err(Error::MultipleConstError);
            }
        }
        try!(writeln!(self.out, "CONST_MAP_BEGIN"));
        for (cid, cval) in rev_const_map {
            try!(writeln!(self.out, "{} -> {}", cid, cval));
        }
        try!(writeln!(self.out, "CONST_MAP_END"));

        Ok(())
    }

}

impl Block {
    fn partial_local_var_map(&self, map: &mut HashMap<Name, usize>, count: &mut usize) -> Result<(), Error> {
        for stmt in self.stmts.iter() {
            match *stmt {
                StmtLocal(ref vname, _) => {
                    for name in vname.iter() {
                        match map.entry(name.clone()) {
                            Vacant(entry) => {
                                entry.insert(*count);
                                *count += 1;
                            }
                            Occupied(..) => {
                                return Err(Error::VarNameError)
                            }
                        }
                    }
                }
                StmtDo(ref block) | StmtWhile(_, ref block) | StmtRepeat(ref block, _) | StmtForNum(_, _, _, _, ref block) | StmtForIn(_, _, ref block) => {
                    try!(block.partial_local_var_map(map, count));
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn local_var_map(&self, args: V<Name>) -> Result<HashMap<Name, usize>, Error> {
        let mut map = HashMap::new();
        let mut count = 0;
        for name in args.iter() {
            match map.entry(name.clone()) {
                Vacant(entry) => {
                    entry.insert(count);
                    count += 1;
                }
                Occupied(..) => {
                    return Err(Error::ArgNameError)
                }
            }
        }
        try!(self.partial_local_var_map(&mut map, &mut count));
        Ok(map)
    }

    fn gen_to<'w, W: Write>(&self, cg: &mut Codegen<'w, W>, var_map: &HashMap<Name, usize>) -> Result<(), Error> {
        for stmt in self.stmts.iter() {
            try!(stmt.gen_to(cg, var_map));
        }
        Ok(())
    }

    fn fun_gen_to<'w, W: Write>(&self, cg: &mut Codegen<'w, W>, func_id: usize, args: V<Name>) -> Result<(), Error> {

        let n_args = args.len();
        let var_map = &try!(self.local_var_map(args));
        let n_vars = var_map.len() - n_args;
        try!(writeln!(cg.out, "BEGIN_FUNC {} {} {}", func_id, n_args, n_vars));

        try!(self.gen_to(cg, var_map));

        if let Some(ref vexpr) = self.ret {
            for expr in vexpr.iter() {
                try!(expr.gen_to(cg, var_map));
            }
            try!(writeln!(cg.out, "    RET"));
        }
        try!(writeln!(cg.out, "END_FUNC"));

        for stmt in self.stmts.iter() {
            match *stmt {
                StmtFunction(ref vname, ref oname, ref args, ref ellipsis, ref pblock) if !*ellipsis => {
                    let fname = (vname.clone(), oname.clone());
                    let func_id = *try!(cg.fn_map.get(&fname).ok_or(Error::FnNameError));
                    try!(pblock.fun_gen_to(cg, func_id, args.clone()));
                }
                StmtLocalFunction(ref name, ref args, ref ellipsis, ref pblock) if !*ellipsis => {
                    let fname = (vec![name.clone()].into_boxed_slice(), None);
                    let func_id = *try!(cg.fn_map.get(&fname).ok_or(Error::FnNameError));
                    try!(pblock.fun_gen_to(cg, func_id, args.clone()));
                }
                StmtFunction(_, _, _, ref ellipsis, _) | StmtLocalFunction(_, _, ref ellipsis, _)  if !*ellipsis => {
                    return Err(Error::NotSupported);
                }
                _ => {}
            }
        }
        Ok(())
    }
}

impl Stmt {
    fn gen_to<'w, W: Write>(&self, cg: &mut Codegen<'w, W>, var_map: &HashMap<Name, usize>) -> Result<(), Error> {
        match *self {
            StmtSet(ref vvar, ref vexpr) => {
                if vvar.len() != vexpr.len() {
                    return Err(Error::SetMismatchError);
                }

                let mut ids = vec![];
                for var in vvar.iter() {
                    match *var {
                        VarName(ref name) => {
                            let id = *var_map.get(name).unwrap();
                            ids.push(id);
                        }
                        VarProperty(..) | VarMember(..) => {
                            return Err(Error::NotSupported);
                        }
                    }
                }

                for expr in vexpr.iter() {
                    try!(expr.gen_to(cg, var_map));
                }

                ids.reverse();
                for id in &ids {
                    try!(writeln!(cg.out, "    STORE_VAR {}", id));
                }
            }
            StmtLocal(ref vname, ref ovexpr) => {
                if let &Some(ref vexpr) = ovexpr {
                    if vname.len() != vexpr.len() {
                        return Err(Error::SetMismatchError);
                    }

                    let mut ids = vec![];
                    for name in vname.iter() {
                        let id = *var_map.get(name).unwrap();
                        ids.push(id);
                    }

                    for expr in vexpr.iter() {
                        try!(expr.gen_to(cg, var_map));
                    }

                    ids.reverse();
                    for id in &ids {
                        try!(writeln!(cg.out, "    STORE_VAR {}", id));
                    }
                }
            }
            StmtLabel(ref name) => {
                let id = {
                    let (map, count) = cg.get_label_map();
                    *map.entry(name.clone()).or_insert_with(|| {
                        let i = *count;
                        *count = i + 1;
                        i
                    })
                };
                try!(writeln!(cg.out, "  L{}:", id));
            }
            StmtGoto(ref name) => {
                let (id, fwd) = {
                    let (map, count) = cg.get_label_map();
                    match map.entry(name.clone()) {
                        Vacant(entry) => {
                            let i = *count;
                            entry.insert(i);
                            *count = i + 1;
                            (i, true)
                        }
                        Occupied(entry) => {
                            let i = *entry.get();
                            (i, false)
                        }
                    }
                };
                if fwd {
                    try!(writeln!(cg.out, "    JMP_FW L{}", id));
                } else {
                    try!(writeln!(cg.out, "    JMP_BW L{}", id));
                }
            }
            StmtFunction(..) | StmtLocalFunction(..) => { /* skip for a second run */ }
            StmtWhile(ref pexpr, ref pblock) => {
                    let (id1, id2) = {
                        let i = cg.label_count;
                        cg.label_count = i + 2;
                        (i, i + 1)
                    };
                    try!(writeln!(cg.out, "  L{}:", id1));
                    try!(pexpr.gen_to(cg, var_map));
                    try!(writeln!(cg.out, "    TJMP_FW L{}", id2));
                    try!(pblock.gen_to(cg, var_map));
                    try!(writeln!(cg.out, "    JMP_BW L{}", id1));
                    try!(writeln!(cg.out, "  L{}:", id2));
            }
            StmtRepeat(ref pblock, ref pexpr) => {
                    let id = {
                        let i = cg.label_count;
                        cg.label_count = i + 1;
                        i
                    };
                    try!(writeln!(cg.out, "  L{}:", id));
                    try!(pblock.gen_to(cg, var_map));
                    try!(pexpr.gen_to(cg, var_map));
                    try!(writeln!(cg.out, "    TJMP_BW L{}", id));
            }
            StmtIf(ref pexpr, ref pblock, ref velseif, ref opblock) => {
                    let (idout, idn) = {
                        let i = cg.label_count;
                        cg.label_count = i + 2;
                        (i, i + 1)
                    };
                    try!(pexpr.gen_to(cg, var_map));
                    try!(writeln!(cg.out, "    TJMP_FW L{}", idn));
                    try!(pblock.gen_to(cg, var_map));
                    try!(writeln!(cg.out, "    JMP_FW L{}", idout));
                    try!(writeln!(cg.out, "  L{}:", idn));
                    let mut idn;
                    for &(ref expr, ref block) in velseif.iter() {
                        idn = {
                            let i = cg.label_count;
                            cg.label_count = i + 1;
                            i
                        };
                        try!(expr.gen_to(cg, var_map));
                        try!(writeln!(cg.out, "    TJMP_FW L{}", idn));
                        try!(block.gen_to(cg, var_map));
                        try!(writeln!(cg.out, "    JMP_FW L{}", idout));
                        try!(writeln!(cg.out, "  L{}:", idn));
                    }
                    if let &Some(ref block) = opblock {
                        try!(block.gen_to(cg, var_map));
                    }
                    try!(writeln!(cg.out, "  L{}:", idout));
            }
            StmtCall(ref pcall) => {
                try!(pcall.gen_to(cg, var_map));
                try!(writeln!(cg.out, "    POP"));
            }
            _ => {
                return Err(Error::NotSupported);
            }
        }
        Ok(())
    }
}

impl Expr {
    fn gen_to<'w, W: Write>(&self, cg: &mut Codegen<'w, W>, var_map: &HashMap<Name, usize>) -> Result<(), Error> {
        match *self {
            ExprNil => try!(ValueNil.gen_to(cg)),
            ExprFalse => try!(ValueFalse.gen_to(cg)),
            ExprTrue => try!(ValueTrue.gen_to(cg)),
            ExprNumber(ref n) => try!(ValueNumber(*n as u64).gen_to(cg)),
            ExprString(ref s) => try!(ValueString(s.clone()).gen_to(cg)),
            ExprBinary(ref e1, ref op, ref e2) => {
                try!(e1.gen_to(cg, var_map));
                try!(e2.gen_to(cg, var_map));
                try!(op.gen_to(cg));
            }
            ExprUnary(ref op, ref e1) => {
                try!(e1.gen_to(cg, var_map));
                try!(op.gen_to(cg));
            }
            ExprCall(ref call) => {
                try!(call.gen_to(cg, var_map));
            }
            ExprVar(ref var) => {
                match **var {
                    VarName(ref name) => {
                        let id = *var_map.get(name).unwrap();
                        try!(writeln!(cg.out, "    LOAD_VAR {}", id));
                    }
                    _ => {
                        return Err(Error::NotSupported);
                    }
                }
            }
            _ => {
                return Err(Error::NotSupported);
            }
        }
        Ok(())
    }
}

impl Call {
    fn gen_to<'w, W: Write>(&self, cg: &mut Codegen<'w, W>, var_map: &HashMap<Name, usize>) -> Result<(), Error> {
        let (fname, args) = match *self {
            CallFunction(ref expr, ref args) => {
                match **expr {
                    ExprVar(ref var) => {
                        match **var {
                            VarName(ref name) => ((vec![name.clone()].into_boxed_slice(), None), args),
                            _ => {
                                return Err(Error::NotSupported);
                            }
                        }
                    }
                    _ => {
                        return Err(Error::NotSupported);
                    }
                }
            }
            CallMethod(ref expr, ref mname, ref args) => {
                match **expr {
                    ExprVar(ref var) => {
                        match **var {
                            VarName(ref name) => ((vec![name.clone()].into_boxed_slice(), Some(mname.clone())), args),
                            _ => {
                                return Err(Error::NotSupported);
                            }
                        }
                    }
                    _ => {
                        return Err(Error::NotSupported);
                    }
                }
            }
        };
        let fid = *try!(cg.fn_map.get(&fname).ok_or(Error::FnNameError));
        match **args {
            ArgsParen(ref vexpr) => {
                for expr in vexpr.iter() {
                    try!(expr.gen_to(cg, var_map));
                }
            }
            ArgsString(ref s) => {
                try!(ValueString(s.clone()).gen_to(cg));
            }
            _ => {
                return Err(Error::NotSupported);
            }
        }
        try!(writeln!(cg.out, "    CALL {}", fid));
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Value {
    ValueNil,
    ValueFalse,
    ValueTrue,
    ValueNumber(u64),
    ValueString(String),
}

impl Value {
    fn gen_to<'w, W: Write>(self, cg: &mut Codegen<'w, W>) -> Result<(), Error> {
        let id = {
            let (map, count) = cg.get_const_map();
            *map.entry(self).or_insert_with(|| {
                let i = *count;
                *count = i + 1;
                i
            })
        };
        try!(writeln!(cg.out, "    LOAD_CONST {}", id));
        Ok(())
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ValueNil => write!(f, "null"),
            ValueFalse => write!(f, "false"),
            ValueTrue => write!(f, "true"),
            ValueNumber(n) => write!(f, "{}", n),
            ValueString(ref s) => write!(f, "\"{}\"", s),
        }
    }
}

impl BinOp {
    fn gen_to<'w, W: Write>(&self, cg: &mut Codegen<'w, W>) -> Result<(), Error> {
        match *self {
            BinAdd => try!(writeln!(cg.out, "    ADD")),
            BinSub => try!(writeln!(cg.out, "    SUB")),
            BinMul => try!(writeln!(cg.out, "    MUL")),
            BinDiv => try!(writeln!(cg.out, "    DIV")),
            BinLt  => try!(writeln!(cg.out, "    LT")),
            BinLe  => try!(writeln!(cg.out, "    LE")),
            BinGt  => try!(writeln!(cg.out, "    GT")),
            BinGe  => try!(writeln!(cg.out, "    GE")),
            BinEq  => try!(writeln!(cg.out, "    EQ")),
            BinNe  => try!(writeln!(cg.out, "    NEQ")),
            BinAnd => try!(writeln!(cg.out, "    AND")),
            BinOr  => try!(writeln!(cg.out, "    OR")),
            _ => {
                return Err(Error::NotSupported);
            }
        }
        Ok(())
    }
}

impl UnOp {
    fn gen_to<'w, W: Write>(&self, cg: &mut Codegen<'w, W>) -> Result<(), Error> {
        match *self {
            UnNeg => try!(writeln!(cg.out, "    NEG")),
            UnNot => try!(writeln!(cg.out, "    NOT")),
            _ => {
                return Err(Error::NotSupported);
            }
        }
        Ok(())
    }
}
