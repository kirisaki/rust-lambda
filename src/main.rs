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
            Lam(x, e) => Lam(x.to_string(), Box::new(e.reduce())),
            App(e0, e1) => {
                let e0 = e0.reduce();
                let e1 = e1.reduce();
                match e0 {
                    Var(_) => App(Box::new(e0), Box::new(e1)),
                    Lam(_, _) => App(Box::new(e0), Box::new(e1)),
                    App(_, _) => App(Box::new(e0), Box::new(e1)),
                }
            }
        }
    }

    fn subst(&self, x:String, y:Expr) -> Expr {
        match self {
            Var(n) => if x == n.to_string() { self.clone() } else { y },
            Lam(n, z) => {
                if x == n.to_string() {
                    y.clone()
                } else {
                    y
                }
            },
            App(e0, e1) => App(Box::new(e0.subst(x.to_string(), y.clone())), Box::new(e1.subst(x.to_string(), y.clone()))),
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



fn main() {
    println!("Hello, world!");
}
