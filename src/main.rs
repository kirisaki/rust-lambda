use std::collections::HashSet;

#[derive(Clone, Debug)]
enum Expr {
    Var (String),
    Lam (String, Box<Expr>),
    App (Box<Expr>, Box<Expr>),
}
use Expr::{Var, Lam, App};

impl Expr {
    fn reduce(&self) -> Expr {
        match self {
            Var(_) => self.clone(),
            Lam(_, _) => self.clone(),
            App(e0, e1) =>
                match e0.id() {
                    Var(_) => app(e0.id(), e1.reduce()),
                    Lam(x, e) => e1.subst(&x, e.id()),
                    App(e2, e3) => app(app(e2.reduce(), e3.reduce()).reduce(), e1.id())
                }
        }
    }

    fn subst(&self, x:&String, y:Expr) -> Expr {
        match y.clone() {
            Var(n) => if x.to_string() == n.to_string() { self.clone() } else { y },
            Lam(n, z) => {
                if x.to_string() == n.to_string() {
                    lam(&n, z.id())
                } else {
                    if self.free().contains(x) {
                        lam(&n, self.subst(x, z.id()))
                    } else {
                        lam(&n, self.subst(x, z.id()))
                    }
                }
            },
            App(e0, e1) => app(self.subst(x, e0.id()), self.subst(x, e1.id()))
        }
    }

    fn id(&self) -> Expr {
        match self {
            Var(x) => var(x),
            Lam(n, e) => lam(n, e.id()),
            App(e0, e1) => app(e0.id(), e1.id(),),
        }
    }

    fn free(&self) -> HashSet<String> {
        match self {
            Var(x) => {
                let mut ns = HashSet::new();
                ns.insert(x.to_string());
                ns
            },
            Lam(n, e) => {
                let mut ns = e.free();
                ns.remove(&n.to_string());
                ns
            }
            App(e0, e1) => {
                let e2 = e0.free();
                let e3 = e1.free();
                e2.union(&e3);
                e2
            }
        }
    }

}

fn var(s: &str) -> Expr {
    Var(s.to_string())
}

fn lam(s: &str, e: Expr) -> Expr {
    Lam(s.to_string(), Box::new(e))
}

fn app (e0: Expr, e1: Expr) -> Expr {
    App(Box::new(e0), Box::new(e1))
}

fn main() {
    let expr = app(app(lam("x", var("x")), var("y")), var("z"));
    println!("{:?}", expr.reduce());
}
