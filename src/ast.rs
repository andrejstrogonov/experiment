#[derive(Debug, Clone)]
pub enum Expr {
    Call { name: String, args: Vec<Expr> },
    Ident(String),
    Int(i64),
    Float(f64),
    Str(String),
    BinaryOp { op: char, lhs: Box<Expr>, rhs: Box<Expr> },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    If { cond: Expr, body: Vec<Stmt> },
    Empty,
}

fn parse_number(s: &str) -> Option<Expr> {
    if let Ok(i) = s.parse::<i64>() { return Some(Expr::Int(i)); }
    if let Ok(f) = s.parse::<f64>() { return Some(Expr::Float(f)); }
    None
}

fn parse_atom(s: &str) -> Expr {
    let s = s.trim();
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        return Expr::Str(s[1..s.len()-1].to_string());
    }
    if let Some(n) = parse_number(s) { return n; }
    // identifier or simple binary like a*b
    // check for binary ops
    for op in ['*','/','+','-'] {
        if let Some(idx) = s.find(op) {
            let lhs = parse_atom(&s[..idx]);
            let rhs = parse_atom(&s[idx+1..]);
            return Expr::BinaryOp { op, lhs: Box::new(lhs), rhs: Box::new(rhs) };
        }
    }
    Expr::Ident(s.to_string())
}

pub fn parse_statements(body: &str) -> Vec<Stmt> {
    let mut res = Vec::new();
    let mut i = 0usize;
    let b = body;
    while i < b.len() {
        // skip whitespace
        while i < b.len() && b.as_bytes()[i].is_ascii_whitespace() { i += 1; }
        if i >= b.len() { break; }
        if b[i..].starts_with("if ") {
            // parse condition between '(' and ')'
            if let Some(op) = b[i..].find('(') {
                if let Some(cp) = b[i+op..].find(')') {
                    let cond = &b[i+op+1..i+op+cp];
                    // find body '{...}'
                    if let Some(bopen) = b[i+op+cp..].find('{') {
                        let mut lev = 1usize;
                        let mut j = i+op+cp+bopen+1;
                        let start = j;
                        while j < b.len() && lev > 0 {
                            match b.as_bytes()[j] {
                                b'{' => lev += 1,
                                b'}' => lev -= 1,
                                _ => {}
                            }
                            j += 1;
                        }
                        let inner = &b[start..j-1];
                        let inner_stmts = parse_statements(inner);
                        res.push(Stmt::If { cond: parse_atom(cond), body: inner_stmts });
                        i = j;
                        continue;
                    }
                }
            }
        }
        // otherwise parse until ';' or end
        if let Some(sc) = b[i..].find(';') {
            let stmt = &b[i..i+sc];
            let s = stmt.trim();
            if s.is_empty() { res.push(Stmt::Empty); }
            else if let Some(p) = s.find('(') {
                // call
                let name = s[..p].trim().to_string();
                let mut args = Vec::new();
                if let Some(pclose_rel) = s[p..].find(')') {
                    let args_str = &s[p+1..p+pclose_rel];
                    for a in args_str.split(',') { let ta = a.trim(); if !ta.is_empty() { args.push(parse_atom(ta)); } }
                }
                res.push(Stmt::Expr(Expr::Call { name, args }));
            } else {
                // expression
                res.push(Stmt::Expr(parse_atom(s)));
            }
            i += sc + 1;
        } else {
            // remaining
            let s = b[i..].trim();
            if !s.is_empty() {
                if let Some(p) = s.find('(') {
                    let name = s[..p].trim().to_string();
                    let mut args = Vec::new();
                    if let Some(pclose_rel) = s[p..].find(')') {
                        let args_str = &s[p+1..p+pclose_rel];
                        for a in args_str.split(',') { let ta = a.trim(); if !ta.is_empty() { args.push(parse_atom(ta)); } }
                    }
                    res.push(Stmt::Expr(Expr::Call { name, args }));
                } else {
                    res.push(Stmt::Expr(parse_atom(s)));
                }
            }
            break;
        }
    }
    res
}

pub fn stmt_to_string(stmt: &Stmt) -> String {
    match stmt {
        Stmt::Empty => String::new(),
        Stmt::Expr(e) => expr_to_string(e) + ";",
        Stmt::If { cond, body } => {
            let mut s = format!("if ({}) {{ ", expr_to_string(cond));
            for st in body { s.push_str(&expr_stmt_block_to_string(st)); }
            s.push_str(" }");
            s
        }
    }
}

fn expr_stmt_block_to_string(stmt: &Stmt) -> String {
    match stmt {
        Stmt::Expr(e) => expr_to_string(e) + "; ",
        Stmt::If { cond, body } => {
            let mut s = format!("if ({}) {{ ", expr_to_string(cond));
            for st in body { s.push_str(&expr_stmt_block_to_string(st)); }
            s.push_str("} ");
            s
        }
        Stmt::Empty => String::new(),
    }
}

pub fn expr_to_string(e: &Expr) -> String {
    match e {
        Expr::Call { name, args } => {
            let a: Vec<String> = args.iter().map(|x| expr_to_string(x)).collect();
            format!("{}({})", name, a.join(", "))
        }
        Expr::Ident(s) => s.clone(),
        Expr::Int(i) => i.to_string(),
        Expr::Float(f) => f.to_string(),
        Expr::Str(s) => format!("\"{}\"", s),
        Expr::BinaryOp { op, lhs, rhs } => format!("{} {} {}", expr_to_string(lhs), op, expr_to_string(rhs)),
    }
}

// simple constant folding: replace binary ops with literals when both sides are literals
pub fn fold_constants(stmts: &mut Vec<Stmt>) {
    for st in stmts.iter_mut() {
        match st {
            Stmt::Expr(e) => { fold_expr(e); }
            Stmt::If { cond, body } => { fold_expr(cond); fold_constants(body); }
            Stmt::Empty => {}
        }
    }
}

fn fold_expr(e: &mut Expr) {
    match e {
        Expr::BinaryOp { op, lhs, rhs } => {
            fold_expr(lhs); fold_expr(rhs);
            match (&**lhs, &**rhs) {
                (Expr::Int(a), Expr::Int(b)) => {
                    let v = match op { '+' => a + b, '-' => a - b, '*' => a * b, '/' => if *b!=0 { a / b } else { *a }, _=> *a };
                    *e = Expr::Int(v);
                }
                (Expr::Float(a), Expr::Float(b)) => {
                    let v = match op { '+' => a + b, '-' => a - b, '*' => a * b, '/' => a / b, _=> *a };
                    *e = Expr::Float(v);
                }
                _ => {}
            }
        }
        Expr::Call { args, .. } => { for a in args { fold_expr(a); } }
        _ => {}
    }
}

// inline helpers: replace Call nodes whose name matches helper name with sequence of helper statements
pub fn inline_helpers(stmts: &mut Vec<Stmt>, helper_bodies: &std::collections::HashMap<String, Vec<Stmt>>) {
    let mut out = Vec::new();
    for st in stmts.iter() {
        match st {
            Stmt::Expr(Expr::Call { name, .. }) => {
                if let Some(body) = helper_bodies.get(name) {
                    // inline body (clone)
                    for b in body.iter() { out.push(b.clone()); }
                    continue;
                } else { out.push(st.clone()); }
            }
            Stmt::If { cond, body } => {
                let mut new_body = body.clone();
                inline_helpers(&mut new_body, helper_bodies);
                out.push(Stmt::If { cond: cond.clone(), body: new_body });
            }
            _ => out.push(st.clone()),
        }
    }
    *stmts = out;
}

// dead code elimination: remove Empty and trivial literal-only expr statements
pub fn dce(stmts: &mut Vec<Stmt>) {
    stmts.retain(|st| match st {
        Stmt::Empty => false,
        Stmt::Expr(e) => match e {
            Expr::Int(_) | Expr::Float(_) | Expr::Str(_) => false,
            _ => true,
        },
        Stmt::If { cond: _, body } => { let mut nb = body.clone(); dce(&mut nb); !nb.is_empty() }
    });
}

// resolve wrapper helpers: if a helper body is a single call to another helper, replace it with the target body
pub fn resolve_wrappers(helper_bodies: &mut std::collections::HashMap<String, Vec<Stmt>>) {
    // iterate until no change
    loop {
        let mut changed = false;
        let keys: Vec<String> = helper_bodies.keys().cloned().collect();
        for k in keys {
            if let Some(body) = helper_bodies.get(&k).cloned() {
                if body.len() == 1 {
                    if let Stmt::Expr(Expr::Call { name, .. }) = &body[0] {
                        if helper_bodies.contains_key(name) {
                            let target = helper_bodies.get(name).cloned().unwrap_or_default();
                            helper_bodies.insert(k.clone(), target);
                            changed = true;
                        }
                    }
                }
            }
        }
        if !changed { break; }
    }
}
