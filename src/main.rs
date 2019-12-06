enum Expr {
    Var (String),
    Lam (String, Box<Expr>),
    App (Box<Expr>, Box<Expr>),
}
use Expr::{Var, Lam, App};

fn reduce(expr: Expr) -> Box<Expr> {
    match expr {
        Var(x) => Box::new(Var(x)),
        Lam(x, e) => Box::new(Lam(x, e)),
        App(e0, e1) => match e0 {
            Var(x) => Box::new(App(Box::new(Var(x)), reduce(e1)))
        }
    }
}

fn main() {
    println!("Hello, world!");
}
