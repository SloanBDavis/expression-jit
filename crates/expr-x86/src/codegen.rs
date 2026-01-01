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
