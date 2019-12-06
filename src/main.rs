enum Expr {
    Var (String),
    Lam (String, Box<Expr>),
    App (Box<Expr>, Box<Expr>),
}

fn reduce(expr: Expr) -> Expr {
    match expr {
        Expr::Var(x) => Expr::Var(x),
        Expr::Lam(x, e) => Expr::Lam(x, e),
        Expr::App(e0, e1) => Expr::App(e0, e1),
    }
}

fn main() {
    println!("Hello, world!");
}
