//! Little CLI to drive our core parsing/language logid.
use elysium::{ast, hir, parse, Root, Stmt};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();

    loop {
        write!(stdout, "==> ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        let parse = parse(&input);
        println!("{}", parse.debug_tree());

        let syntax = parse.syntax();

        for error in ast::validation::validate(&syntax) {
            println!("{error}",);
        }

        let root = Root::cast(syntax).unwrap();

        dbg!(root
            .stmts()
            .filter_map(|stmt| if let Stmt::VariableDef(var_def) = stmt {
                Some(var_def.value())
            } else {
                None
            })
            .collect::<Vec<_>>());

        dbg!(hir::lower(root));

        input.clear();
    }
}
