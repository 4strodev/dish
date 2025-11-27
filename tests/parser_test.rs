use dish::ast::parser::Parser;

#[test]
fn test_infix_expr() {
    let tests = vec![
        ("2 + 2", "(+ 2 2)"),
        ("2 - 2", "(- 2 2)"),
        ("2 * 2", "(* 2 2)"),
        ("2 / 2", "(/ 2 2)"),
    ];

    for (input, result) in tests {
        let mut parser = Parser::from(input);
        assert_eq!(format!("{}", parser.expr()), result);
    }
}

#[test]
fn test_optional_parenthesis() {
    let tests = vec![
        ("(2 + (2))", "(+ 2 2)"),
        ("(2 - (2))", "(- 2 2)"),
        ("(2 * (2))", "(* 2 2)"),
        ("(2 / (2))", "(/ 2 2)"),
    ];

    for (input, result) in tests {
        let mut parser = Parser::from(input);
        assert_eq!(format!("{}", parser.expr()), result);
    }
}

#[test]
fn test_inner_expressions() {
    let tests = vec![
        ("(2 + (2 * 2))", "(+ 2 (* 2 2))"),
    ];

    for (input, result) in tests {
        let mut parser = Parser::from(input);
        assert_eq!(format!("{}", parser.expr()), result);
    }
}
