use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::{Expr, Program, Stmt};
use crate::object::{Environment, Object};

pub struct Evaluator {
    pub stdout: Vec<String>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator { stdout: Vec::new() }
    }

    pub fn eval_program(&mut self, program: &Program, env: &Rc<RefCell<Environment>>) -> Object {
        let mut result = Object::Null;

        for stmt in &program.statement {
            result = self.eval_statement(stmt, env);

            match result {
                Object::ReturnValue(val) => return *val,
                Object::Error(_) => return result,
                _ => {}
            }
        }

        result
    }

    pub fn eval_statement(&mut self, stmt: &Stmt, env: &Rc<RefCell<Environment>>) -> Object {
        match stmt {
            Stmt::Let {
                name,
                var_type,
                value,
            } => {
                let val = self.eval_expression(value, env);
                if self.is_error(&val) {
                    return val;
                }

                if var_type != "auto" && val.type_name() != "null" && val.type_name() != var_type {
                    return Object::Error(format!(
                        "Type mismatch: cannot assign value of type '{}' to variable '{}' of type '{}'",
                        val.type_name(),
                        name,
                        var_type
                    ));
                }

                match env
                    .borrow_mut()
                    .define(name.clone(), val, var_type.clone(), false)
                {
                    Ok(v) => v,
                    Err(e) => Object::Error(e),
                }
            }
            Stmt::Const {
                name,
                var_type,
                value,
            } => {
                let val = self.eval_expression(value, env);
                if self.is_error(&val) {
                    return val;
                }

                if var_type != "auto" && val.type_name() != "null" && val.type_name() != var_type {
                    return Object::Error(format!(
                        "Type mismatch: cannot assign value of type '{}' to const '{}' of type '{}'",
                        val.type_name(),
                        name,
                        var_type
                    ));
                }

                match env
                    .borrow_mut()
                    .define(name.clone(), val, var_type.clone(), true)
                {
                    Ok(v) => v,
                    Err(e) => Object::Error(e),
                }
            }
            Stmt::Return { value } => {
                let val = self.eval_expression(value, env);
                if self.is_error(&val) {
                    return val;
                }
                Object::ReturnValue(Box::new(val))
            }
            Stmt::Break => Object::Break,
            Stmt::Continue => Object::Continue,
            Stmt::If {
                condition,
                consequence,
                alternative,
            } => {
                let cond_val = self.eval_expression(condition, env);
                if self.is_error(&cond_val) {
                    return cond_val;
                }

                if cond_val.is_truthy() {
                    self.eval_block(consequence, env)
                } else if let Some(alt) = alternative {
                    self.eval_block(alt, env)
                } else {
                    Object::Null
                }
            }
            Stmt::While { condition, body } => {
                let mut result = Object::Null;

                loop {
                    let cond_val = self.eval_expression(condition, env);
                    if self.is_error(&cond_val) {
                        return cond_val;
                    }

                    if !cond_val.is_truthy() {
                        break;
                    }

                    result = self.eval_block(body, env);

                    match result {
                        Object::ReturnValue(_) | Object::Error(_) => return result,
                        Object::Break => {
                            result = Object::Null;
                            break;
                        }
                        Object::Continue => {
                            result = Object::Null;
                            continue;
                        }
                        _ => {}
                    }
                }

                result
            }
            Stmt::For {
                identifier,
                start_value,
                end_value,
                body,
            } => {
                let start_obj = self.eval_expression(start_value, env);
                if self.is_error(&start_obj) {
                    return start_obj;
                }

                let end_obj = self.eval_expression(end_value, env);
                if self.is_error(&end_obj) {
                    return end_obj;
                }

                let start = match start_obj {
                    Object::Integer(i) => i,
                    _ => {
                        return Object::Error(format!(
                            "For loop start value must be an integer, got '{}'",
                            start_obj.type_name()
                        ));
                    }
                };

                let end = match end_obj {
                    Object::Integer(i) => i,
                    _ => {
                        return Object::Error(format!(
                            "For loop end value must be an integer, got '{}'",
                            end_obj.type_name()
                        ));
                    }
                };

                let mut result = Object::Null;

                let for_env = Rc::new(RefCell::new(Environment::new_enclosed(Rc::clone(env))));
                let _ = for_env.borrow_mut().define(
                    identifier.clone(),
                    Object::Integer(start),
                    "int".to_string(),
                    false,
                );

                let range: Box<dyn Iterator<Item = i64>> = if start <= end {
                    Box::new(start..=end)
                } else {
                    Box::new((end..=start).rev())
                };

                for current in range {
                    let _ = for_env
                        .borrow_mut()
                        .set(identifier, Object::Integer(current));
                    result = self.eval_block(body, &for_env);

                    match result {
                        Object::ReturnValue(_) | Object::Error(_) => return result,
                        Object::Break => {
                            result = Object::Null;
                            break;
                        }
                        Object::Continue => {
                            result = Object::Null;
                            continue;
                        }
                        _ => {}
                    }
                }

                result
            }
            Stmt::Function {
                name,
                parameters,
                return_type,
                body,
            } => {
                let func = Object::Function {
                    parameters: parameters.clone(),
                    return_type: return_type.clone(),
                    body: body.clone(),
                    env: Rc::clone(env),
                };

                match env
                    .borrow_mut()
                    .define(name.clone(), func, "fun".to_string(), false)
                {
                    Ok(v) => v,
                    Err(e) => Object::Error(e),
                }
            }
            Stmt::Expression(expr) => self.eval_expression(expr, env),
        }
    }

    pub fn eval_block(&mut self, statements: &[Stmt], env: &Rc<RefCell<Environment>>) -> Object {
        let block_env = Rc::new(RefCell::new(Environment::new_enclosed(Rc::clone(env))));
        let mut result = Object::Null;

        for stmt in statements {
            result = self.eval_statement(stmt, &block_env);

            match result {
                Object::ReturnValue(_) | Object::Break | Object::Continue | Object::Error(_) => {
                    return result;
                }
                _ => {}
            }
        }

        result
    }

    pub fn eval_expression(&mut self, expr: &Expr, env: &Rc<RefCell<Environment>>) -> Object {
        match expr {
            Expr::IntLiteral(val) => Object::Integer(*val),
            Expr::FloatLiteral(val) => Object::Float(*val),
            Expr::StringLiteral(val) => Object::String(val.clone()),
            Expr::BooleanLiteral(val) => Object::Boolean(*val),
            Expr::NoneLiteral => Object::Null,
            Expr::Identifier(name) => match env.borrow().get(name) {
                Some(obj) => obj,
                None => Object::Error(format!("Identifier not found: '{}'", name)),
            },
            Expr::Prefix { operator, right } => {
                let right_obj = self.eval_expression(right, env);
                if self.is_error(&right_obj) {
                    return right_obj;
                }
                self.eval_prefix_expression(operator, &right_obj)
            }
            Expr::Infix {
                left,
                operator,
                right,
            } => {
                if self.is_assignment_operator(operator) {
                    return self.eval_assignment_expression(left, operator, right, env);
                }

                let left_obj = self.eval_expression(left, env);
                if self.is_error(&left_obj) {
                    return left_obj;
                }

                let right_obj = self.eval_expression(right, env);
                if self.is_error(&right_obj) {
                    return right_obj;
                }

                self.eval_infix_expression(operator, &left_obj, &right_obj)
            }
            Expr::Cast { left, cast_type } => {
                let val = self.eval_expression(left, env);
                if self.is_error(&val) {
                    return val;
                }
                self.eval_cast_expression(&val, cast_type)
            }
            Expr::Ternary {
                condition,
                consequence,
                alternative,
            } => {
                let cond_val = self.eval_expression(condition, env);
                if self.is_error(&cond_val) {
                    return cond_val;
                }

                if cond_val.is_truthy() {
                    self.eval_expression(consequence, env)
                } else {
                    self.eval_expression(alternative, env)
                }
            }
            Expr::Call {
                function,
                arguments,
            } => {
                let func_obj = self.eval_expression(function, env);
                if self.is_error(&func_obj) {
                    return func_obj;
                }

                let mut args = Vec::new();
                for arg_expr in arguments {
                    let arg_val = self.eval_expression(arg_expr, env);
                    if self.is_error(&arg_val) {
                        return arg_val;
                    }
                    args.push(arg_val);
                }

                self.apply_function(&func_obj, args)
            }
        }
    }

    fn is_assignment_operator(&self, op: &str) -> bool {
        matches!(op, "=" | "+=" | "-=" | "*=" | "/=" | "%=")
    }

    fn eval_assignment_expression(
        &mut self,
        left: &Expr,
        operator: &str,
        right: &Expr,
        env: &Rc<RefCell<Environment>>,
    ) -> Object {
        let name = match left {
            Expr::Identifier(id) => id,
            _ => return Object::Error("Invalid assignment target".to_string()),
        };

        let right_val = self.eval_expression(right, env);
        if self.is_error(&right_val) {
            return right_val;
        }

        let new_value = if operator == "=" {
            right_val
        } else {
            let current_val = match env.borrow().get(name) {
                Some(val) => val,
                None => {
                    return Object::Error(format!("Variable '{}' not found for assignment", name));
                }
            };

            let op_char = match operator {
                "+=" => "+",
                "-=" => "-",
                "*=" => "*",
                "/=" => "/",
                "%=" => "%",
                _ => {
                    return Object::Error(format!(
                        "Unsupported assignment operator '{}'",
                        operator
                    ));
                }
            };

            let computed = self.eval_infix_expression(op_char, &current_val, &right_val);
            if self.is_error(&computed) {
                return computed;
            }
            computed
        };

        match env.borrow_mut().set(name, new_value.clone()) {
            Ok(_) => new_value,
            Err(e) => Object::Error(e),
        }
    }

    fn eval_prefix_expression(&self, operator: &str, right: &Object) -> Object {
        match operator {
            "!" => Object::Boolean(!right.is_truthy()),
            "-" => match right {
                Object::Integer(i) => Object::Integer(-i),
                Object::Float(f) => Object::Float(-f),
                _ => Object::Error(format!("Unknown operator: -{}", right.type_name())),
            },
            _ => Object::Error(format!(
                "Unknown operator: {}{}",
                operator,
                right.type_name()
            )),
        }
    }

    fn eval_infix_expression(&self, operator: &str, left: &Object, right: &Object) -> Object {
        match (left, right) {
            (Object::Integer(l), Object::Integer(r)) => match operator {
                "+" => Object::Integer(l + r),
                "-" => Object::Integer(l - r),
                "*" => Object::Integer(l * r),
                "/" => {
                    if *r == 0 {
                        Object::Error("Division by zero".to_string())
                    } else {
                        Object::Integer(l / r)
                    }
                }
                "%" => {
                    if *r == 0 {
                        Object::Error("Modulo by zero".to_string())
                    } else {
                        Object::Integer(l % r)
                    }
                }
                "^" => Object::Integer(l.pow(*r as u32)),
                "==" => Object::Boolean(l == r),
                "!=" => Object::Boolean(l != r),
                "<" => Object::Boolean(l < r),
                "<=" => Object::Boolean(l <= r),
                ">" => Object::Boolean(l > r),
                ">=" => Object::Boolean(l >= r),
                _ => Object::Error(format!("Unknown operator: int {} int", operator)),
            },
            (Object::Float(l), Object::Float(r)) => match operator {
                "+" => Object::Float(l + r),
                "-" => Object::Float(l - r),
                "*" => Object::Float(l * r),
                "/" => Object::Float(l / r),
                "%" => Object::Float(l % r),
                "^" => Object::Float(l.powf(*r)),
                "==" => Object::Boolean(l == r),
                "!=" => Object::Boolean(l != r),
                "<" => Object::Boolean(l < r),
                "<=" => Object::Boolean(l <= r),
                ">" => Object::Boolean(l > r),
                ">=" => Object::Boolean(l >= r),
                _ => Object::Error(format!("Unknown operator: float {} float", operator)),
            },
            (Object::Integer(l), Object::Float(r)) => match operator {
                "+" => Object::Float(*l as f64 + r),
                "-" => Object::Float(*l as f64 - r),
                "*" => Object::Float(*l as f64 * r),
                "/" => Object::Float(*l as f64 / r),
                "==" => Object::Boolean(*l as f64 == *r),
                "!=" => Object::Boolean(*l as f64 != *r),
                "<" => Object::Boolean((*l as f64) < *r),
                ">" => Object::Boolean((*l as f64) > *r),
                _ => Object::Error(format!("Unknown operator: int {} float", operator)),
            },
            (Object::Float(l), Object::Integer(r)) => match operator {
                "+" => Object::Float(l + *r as f64),
                "-" => Object::Float(l - *r as f64),
                "*" => Object::Float(l * *r as f64),
                "/" => Object::Float(l / *r as f64),
                "==" => Object::Boolean(*l == *r as f64),
                "!=" => Object::Boolean(*l != *r as f64),
                "<" => Object::Boolean(*l < (*r as f64)),
                ">" => Object::Boolean(*l > (*r as f64)),
                _ => Object::Error(format!("Unknown operator: float {} int", operator)),
            },
            (Object::String(l), Object::String(r)) => match operator {
                "+" => Object::String(format!("{}{}", l, r)),
                "==" => Object::Boolean(l == r),
                "!=" => Object::Boolean(l != r),
                _ => Object::Error(format!("Unknown operator: str {} str", operator)),
            },
            (Object::String(l), other) => match operator {
                "+" => Object::String(format!("{}{}", l, other.inspect())),
                _ => Object::Error(format!(
                    "Unknown operator: str {} {}",
                    operator,
                    other.type_name()
                )),
            },
            (other, Object::String(r)) => match operator {
                "+" => Object::String(format!("{}{}", other.inspect(), r)),
                _ => Object::Error(format!(
                    "Unknown operator: {} {} str",
                    other.type_name(),
                    operator
                )),
            },
            (Object::Boolean(l), Object::Boolean(r)) => match operator {
                "==" => Object::Boolean(l == r),
                "!=" => Object::Boolean(l != r),
                "&&" => Object::Boolean(*l && *r),
                "||" => Object::Boolean(*l || *r),
                _ => Object::Error(format!("Unknown operator: bool {} bool", operator)),
            },
            _ => Object::Error(format!(
                "Type mismatch: {} {} {}",
                left.type_name(),
                operator,
                right.type_name()
            )),
        }
    }

    fn eval_cast_expression(&self, val: &Object, target_type: &str) -> Object {
        match target_type {
            "int" => match val {
                Object::Integer(i) => Object::Integer(*i),
                Object::Float(f) => Object::Integer(*f as i64),
                Object::String(s) => match s.parse::<i64>() {
                    Ok(i) => Object::Integer(i),
                    Err(_) => Object::Error(format!("Cannot cast string '{}' to int", s)),
                },
                Object::Boolean(b) => Object::Integer(if *b { 1 } else { 0 }),
                _ => Object::Error(format!("Cannot cast {} to int", val.type_name())),
            },
            "float" => match val {
                Object::Float(f) => Object::Float(*f),
                Object::Integer(i) => Object::Float(*i as f64),
                Object::String(s) => match s.parse::<f64>() {
                    Ok(f) => Object::Float(f),
                    Err(_) => Object::Error(format!("Cannot cast string '{}' to float", s)),
                },
                _ => Object::Error(format!("Cannot cast {} to float", val.type_name())),
            },
            "str" => Object::String(val.inspect()),
            "bool" => Object::Boolean(val.is_truthy()),
            _ => Object::Error(format!("Unknown type for casting: '{}'", target_type)),
        }
    }

    fn apply_function(&mut self, func: &Object, args: Vec<Object>) -> Object {
        match func {
            Object::BuiltinFunction(name) => {
                if name == "print" {
                    let output = args
                        .iter()
                        .map(|a| a.inspect())
                        .collect::<Vec<String>>()
                        .join(" ");
                    println!("{}", output);
                    self.stdout.push(output);
                    Object::Null
                } else {
                    Object::Error(format!("Unknown builtin function '{}'", name))
                }
            }
            Object::Function {
                parameters,
                body,
                env,
                ..
            } => {
                if parameters.len() != args.len() {
                    return Object::Error(format!(
                        "Argument count mismatch: expected {}, got {}",
                        parameters.len(),
                        args.len()
                    ));
                }

                let call_env = Rc::new(RefCell::new(Environment::new_enclosed(Rc::clone(env))));

                for (param, arg) in parameters.iter().zip(args) {
                    if param.var_type != "auto"
                        && arg.type_name() != "null"
                        && arg.type_name() != param.var_type
                    {
                        return Object::Error(format!(
                            "Type mismatch for parameter '{}': expected '{}', got '{}'",
                            param.name,
                            param.var_type,
                            arg.type_name()
                        ));
                    }
                    let _ = call_env.borrow_mut().define(
                        param.name.clone(),
                        arg,
                        param.var_type.clone(),
                        false,
                    );
                }

                let result = self.eval_block(body, &call_env);
                match result {
                    Object::ReturnValue(val) => *val,
                    _ => result,
                }
            }
            _ => Object::Error(format!("'{}' is not a callable function", func.type_name())),
        }
    }

    fn is_error(&self, obj: &Object) -> bool {
        matches!(obj, Object::Error(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn test_eval(input: &str) -> (Object, Vec<String>) {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert!(
            parser.errors.is_empty(),
            "Parser errors: {:?}",
            parser.errors
        );

        let env = Rc::new(RefCell::new(Environment::new()));
        let mut evaluator = Evaluator::new();
        let result = evaluator.eval_program(&program, &env);
        (result, evaluator.stdout)
    }

    #[test]
    fn test_variables_and_arithmetic() {
        let input = "
            let a: int = 10;
            let b: int = 20;
            let sum: int = a + b * 2;
        ";
        let (res, _) = test_eval(input);
        assert_eq!(res, Object::Integer(50));
    }

    #[test]
    fn test_strings_and_builtins() {
        let input = "
            let greeting: str = \"Hello, \" + \"Nouva!\";
            print(greeting);
        ";
        let (_, stdout) = test_eval(input);
        assert_eq!(stdout, vec!["Hello, Nouva!"]);
    }

    #[test]
    fn test_functions_and_returns() {
        let input = "
            fun multiply(x: int, y: int) -> int {
                return x * y;
            }
            let res: int = multiply(6, 7);
        ";
        let (res, _) = test_eval(input);
        assert_eq!(res, Object::Integer(42));
    }

    #[test]
    fn test_if_else_control_flow() {
        let input = "
            let age: int = 18;
            if age >= 18 {
                print(\"Adult\");
            } else {
                print(\"Minor\");
            }
        ";
        let (_, stdout) = test_eval(input);
        assert_eq!(stdout, vec!["Adult"]);
    }

    #[test]
    fn test_loops_while_and_for() {
        let input = "
            let mut_sum: int = 0;
            for i in 1 to 5 {
                mut_sum += i;
            }
            print(mut_sum);
        ";
        let (_, stdout) = test_eval(input);
        assert_eq!(stdout, vec!["15"]);
    }

    #[test]
    fn test_type_checking_error() {
        let input = "let x: int = \"hello\";";
        let (res, _) = test_eval(input);
        match res {
            Object::Error(msg) => assert!(msg.contains("Type mismatch")),
            other => panic!("Expected type error, got {:?}", other),
        }
    }
}
