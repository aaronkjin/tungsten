mod ast;

fn main() {
    let input = "7 + 3 * (10 / (12 / (3 + 1) - 1))";

    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    let mut ast: Ast = Ast::new();
    let mut parser = Parser::new();

    // Macro print
    println!("{:?}", tokens);
}
