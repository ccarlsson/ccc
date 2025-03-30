use crate::ast::{ASTNode, Expression, Statement};

pub fn generate_code(ast: ASTNode) -> String {
    let mut assembly = String::new();
    let mut data_section = String::from("section .data\n");
    let mut text_section = String::from("section .text\n");
    text_section.push_str("global _start\n_start:\n");

    let mut string_literals = Vec::new(); // To store string literals

    match ast {
        ASTNode::Program { variable_section, statement_section } => {
            // Add variable declarations to the .data section
            for var in variable_section {
                for name in &var.names {
                    data_section.push_str(&format!("{} dq 0\n", name));
                }
            }

            // Add the buffer to the .data section
            data_section.push_str("buffer db 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0\n");

            // Generate assembly for statements
            for stmt in statement_section {
                text_section.push_str(&generate_statement(&stmt, &mut string_literals));
            }
        }
        _ => unimplemented!(),
    }

    // Add string literals to the .data section
    for (i, literal) in string_literals.iter().enumerate() {
        data_section.push_str(&format!("string_{} db '{}', 0\n", i, literal));
    }

    // Add the number-to-string conversion routine
    text_section.push_str(&number_to_string_routine());

    // Append the sections to the final assembly
    assembly.push_str(&data_section);
    assembly.push_str(&text_section);
    assembly.push_str("mov rax, 60\nxor rdi, rdi\nsyscall\n"); // Exit syscall
    assembly
}

fn generate_statement(stmt: &Statement, string_literals: &mut Vec<String>) -> String {
    let mut assembly = String::new();
    match stmt {
        Statement::Assignment(var, expr) => {
            assembly.push_str(&generate_expression(expr, string_literals));
            assembly.push_str(&format!("mov [{}], rax\n", var)); // Store result in variable
        }
        Statement::Print(items) => {
            for item in items {
                match item {
                    Expression::StringLiteral(text) => {
                        // Add the string literal to the list and reference it
                        let index = string_literals.len();
                        string_literals.push(text.clone());
                        assembly.push_str(&format!(
                            "mov rax, 1\nmov rdi, 1\nmov rsi, string_{}\nmov rdx, {}\nsyscall\n",
                            index,
                            text.len() + 1 // Include null terminator
                        ));
                    }
                    _ => {
                        assembly.push_str(&generate_expression(item, string_literals));
                        assembly.push_str("mov rdi, buffer\n"); // Address of the buffer
                        assembly.push_str("call int_to_string\n"); // Convert number to string
                        assembly.push_str(
                            "mov rax, 1\nmov rdi, 1\nmov rsi, buffer\nmov rdx, rbx\nsyscall\n",
                        ); // Print the converted number
                    }
                }
            }
        }
    }
    assembly
}

fn generate_expression(expr: &Expression, string_literals: &mut Vec<String>) -> String {
    match expr {
        Expression::IntegerLiteral(value) => format!("mov rax, {}\n", value),
        Expression::Variable(name) => format!("mov rax, [{}]\n", name),
        Expression::BinaryOperation { left, operator, right } => {
            let mut assembly = String::new();
            assembly.push_str(&generate_expression(left, string_literals));
            assembly.push_str("push rax\n"); // Save left operand
            assembly.push_str(&generate_expression(right, string_literals));
            assembly.push_str("pop rbx\n"); // Restore left operand
            match operator {
                '+' => assembly.push_str("add rax, rbx\n"),
                '-' => assembly.push_str("sub rax, rbx\n"),
                '*' => assembly.push_str("imul rax, rbx\n"),
                '/' => assembly.push_str("xor rdx, rdx\nidiv rbx\n"),
                _ => unimplemented!(),
            }
            assembly
        }
        Expression::StringLiteral(text) => {
            // Add the string literal to the list and return its label
            let index = string_literals.len();
            string_literals.push(text.clone());
            format!("mov rax, string_{}\n", index)
        }
    }
}

fn number_to_string_routine() -> String {
    String::from(
        r#"
int_to_string:
    xor rcx, rcx        ; Clear RCX (digit counter)
    mov rbx, rdi        ; Save the buffer address in RBX
    add rbx, 10         ; Point to the end of the buffer
    mov byte [rbx], 0   ; Null-terminate the string

convert_loop:
    xor rdx, rdx        ; Clear RDX (remainder)
    mov rcx, 10         ; Load the divisor (10) into RCX
    div rcx             ; Divide RAX by RCX, quotient in RAX, remainder in RDX
    add dl, '0'         ; Convert remainder to ASCII
    dec rbx             ; Move buffer pointer backward
    mov [rbx], dl       ; Store ASCII character in buffer
    inc rcx             ; Increment digit counter
    test rax, rax       ; Check if quotient is 0
    jnz convert_loop    ; Repeat if RAX is not 0

    mov rdi, rbx        ; Update RDI to point to the start of the string
    mov rbx, 10         ; Return the string length in RBX
    ret
"#,
    )
}

