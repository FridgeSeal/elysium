//! Little CLI to drive our core parsing/language logid.
use elysium::{parse, Root, Stmt, hir};
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

        let root = Root::cast(parse.syntax()).unwrap();

        dbg!(root
            .stmts()
            .filter_map(|stmt| if let Stmt::VariableDef(var_def) = stmt {
                Some(var_def.value())
            } else {
                None
            })
            .collect::<Vec<_>>());


        dbg!(hir::lower(root).collect::<Vec<_>>());
        
        input.clear();
    }
}
