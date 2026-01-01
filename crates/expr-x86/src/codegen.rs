use crate::encoding;
use crate::runtime::CompiledCode;
use expr_core::{BinOp, Expr};

pub fn compile(expr: &Expr) -> Result<CompiledCode, String> {
    let mut code = Vec::new();

    emit_expr(&mut code, expr);

    code.extend(encoding::pop_rax());
    code.extend(encoding::ret());

    CompiledCode::new(&code)
}

fn emit_expr(code: &mut Vec<u8>, expr: &Expr) {
    match expr {
        Expr::Integer(n) => {
            code.extend(encoding::mov_rax_imm64(*n));
            code.extend(encoding::push_rax());
        }
        Expr::BinaryOp { op, left, right } => {
            emit_expr(code, left);
            emit_expr(code, right);

            code.extend(encoding::pop_rbx());
            code.extend(encoding::pop_rax());

            match op {
                BinOp::Add => code.extend(encoding::add_rax_rbx()),
                BinOp::Sub => code.extend(encoding::sub_rax_rbx()),
                BinOp::Mul => code.extend(encoding::imul_rax_rbx()),
                BinOp::Div => {
                    code.extend(encoding::cqo());
                    code.extend(encoding::idiv_rbx());
                }
            }

            // Push res
            code.extend(encoding::push_rax());
        }
    }
}

#[cfg(all(test, target_arch = "x86_64"))]
mod tests {
    use super::*;
    use expr_parse::Parser;

    fn eval(input: &str) -> i64 {
        let mut parser = Parser::new(input).unwrap();
        let ast = parser.parse().unwrap();
        let code = compile(&ast).unwrap();
        unsafe { code.execute() }
    }

    #[test]
    fn single_integer() {
        assert_eq!(eval("42"), 42);
        assert_eq!(eval("0"), 0);
        assert_eq!(eval("1"), 1);
    }

    #[test]
    fn addition() {
        assert_eq!(eval("1 + 2"), 3);
        assert_eq!(eval("0 + 0"), 0);
        assert_eq!(eval("100 + 200"), 300);
    }

    #[test]
    fn subtraction() {
        assert_eq!(eval("5 - 3"), 2);
        assert_eq!(eval("3 - 5"), -2);
        assert_eq!(eval("0 - 0"), 0);
    }

    #[test]
    fn multiplication() {
        assert_eq!(eval("4 * 5"), 20);
        assert_eq!(eval("0 * 100"), 0);
        assert_eq!(eval("1 * 1"), 1);
    }

    #[test]
    fn division() {
        assert_eq!(eval("10 / 2"), 5);
        assert_eq!(eval("10 / 3"), 3);
        assert_eq!(eval("0 / 5"), 0);
        assert_eq!(eval("1 / 1"), 1);
    }

    #[test]
    fn division_truncation() {
        assert_eq!(eval("7 / 2"), 3);
        assert_eq!(eval("7 / 3"), 2);
    }

    #[test]
    fn precedence() {
        assert_eq!(eval("2 + 3 * 4"), 14);
        assert_eq!(eval("2 * 3 + 4"), 10);
        assert_eq!(eval("10 - 2 * 3"), 4);
        assert_eq!(eval("10 / 2 + 3"), 8);
    }

    #[test]
    fn parentheses() {
        assert_eq!(eval("(2 + 3) * 4"), 20);
        assert_eq!(eval("2 * (3 + 4)"), 14);
        assert_eq!(eval("(10 - 2) * 3"), 24);
        assert_eq!(eval("((1 + 2))"), 3);
    }

    #[test]
    fn associativity() {
        assert_eq!(eval("10 - 3 - 2"), 5);
        assert_eq!(eval("20 / 4 / 2"), 2);
        assert_eq!(eval("2 + 3 + 4"), 9);
        assert_eq!(eval("2 * 3 * 4"), 24);
    }

    #[test]
    fn complex_expressions() {
        assert_eq!(eval("(2 + 3) * (4 - 1)"), 15);
        assert_eq!(eval("1 + 2 * 3 + 4"), 11);
        assert_eq!(eval("(1 + 2) * (3 + 4) * (5 + 6)"), 231);
    }

    #[test]
    fn negative_results() {
        assert_eq!(eval("3 - 10"), -7);
        assert_eq!(eval("0 - 100"), -100);
        assert_eq!(eval("(1 - 5) * 2"), -8);
    }

    #[test]
    fn large_numbers() {
        assert_eq!(eval("1000000 * 1000"), 1000000000);
        assert_eq!(eval("1000000000 + 1000000000"), 2000000000);
    }
}
