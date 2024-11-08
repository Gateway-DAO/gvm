use compute::prelude::*;

#[test]
fn test_macro_arithmetic_compiler() {
    #[encrypted(compile)]
    fn multi_arithmetic(a: u8, b: u8, c: u8, d: u8) -> (Circuit, Vec<bool>) {
        let res = a * b;
        let res = res + c;
        res - d
    }

    let a = 2_u8;
    let b = 5_u8;
    let c = 3_u8;
    let d = 4_u8;

    let (circuit, inputs) = multi_arithmetic(a, b, c, d);
    let result = get_executor().execute(&circuit, &inputs, &[]).unwrap();
    let result: GarbledUint<8> = GarbledUint::new(result);
    let result: u8 = result.into();
    assert_eq!(result, a * b + c - d);
}

#[test]
fn test_macro_arithmetic() {
    #[encrypted(execute)]
    fn multi_arithmetic(a: u8, b: u8, c: u8, d: u8) -> u8 {
        let res = a * b;
        let res = res + c;
        res - d
    }

    let a = 2_u8;
    let b = 5_u8;
    let c = 3_u8;
    let d = 4_u8;

    let result = multi_arithmetic(a, b, c, d);
    assert_eq!(result, a * b + c - d);
}

#[test]
fn test_macro_arithmetic_u128() {
    #[encrypted(execute)]
    fn multi_arithmetic_u128(a: u8, b: u8, c: u8, d: u8) -> u8 {
        let res = a + b;
        let res = res + c;
        res - d
    }

    let a = 2_u128;
    let b = 5_u128;
    let c = 3_u128;
    let d = 4_u128;

    let result = multi_arithmetic_u128(a, b, c, d);
    assert_eq!(result, a + b + c - d);
}

#[test]
fn test_macro_mixed_arithmetic() {
    #[encrypted(execute)]
    fn mixed_arithmetic(a: u8, b: u8, c: u8, d: u8) -> u8 {
        let res = a * b;
        let res = context.add(&res, c);
        let res = res - d;
        context.mul(&res, a)
    }

    let a = 2_u8;
    let b = 5_u8;
    let c = 3_u8;
    let d = 4_u8;

    let result = mixed_arithmetic(a, b, c, d);
    assert_eq!(result, ((a * b + c - d) * a));
}

#[test]
fn test_macro_addition() {
    #[encrypted(execute)]
    fn addition(a: u8, b: u8) -> u8 {
        a + b
    }

    let a = 2_u8;
    let b = 5_u8;

    let result = addition(a, b);
    assert_eq!(result, a + b);
}

#[test]
fn test_macro_subtraction() {
    #[encrypted(execute)]
    fn subtraction(a: u8, b: u8) -> u8 {
        a - b
    }

    let a = 20_u8;
    let b = 5_u8;

    let result = subtraction(a, b);
    assert_eq!(result, a - b);
}

#[test]
fn test_macro_multiplication() {
    #[encrypted(execute)]
    fn multiplication(a: u8, b: u8) -> u8 {
        a * b
    }

    let a = 20_u8;
    let b = 5_u8;

    let result = multiplication(a, b);
    assert_eq!(result, a * b);
}

#[test]
fn test_macro_mux() {
    #[encrypted(execute)]
    fn mux_circuit(a: u8, b: u8) -> u8 {
        let condition = a == b;
        &context.mux(&condition, a, b)
    }

    let a = 5_u8;
    let b = 10_u8;

    let result = mux_circuit(a, b);
    assert_eq!(result, b);
}

#[test]
fn test_macro_if_else() {
    #[encrypted(execute)]
    fn mux_circuit(a: T, b: T) -> T {
        if a == b {
            let c = a * b;
            c + a
        } else {
            a + b
        }
    }

    let a = 10_u16;
    let b = 5_u16;

    let result: u16 = mux_circuit(a, b);
    assert_eq!(result, a + b);
}

#[test]
fn test_macro_if_else2() {
    #[encrypted(execute)]
    fn mux_circuit(a: u8, b: u8) -> u8 {
        let true_branch = a * b;
        let false_branch = a + b;
        let condition = a == b;
        if condition {
            true_branch
        } else {
            false_branch
        }
    }

    let a = 10_u8;
    let b = 5_u8;

    let result = mux_circuit(a, b);
    assert_eq!(result, a + b);

    let a = 5_u8;
    let result = mux_circuit(a, b);
    assert_eq!(result, a * b);
}

#[test]
fn test_macro_if_else3() {
    #[encrypted(execute)]
    fn mux_circuit(a: u8, b: u8) -> u8 {
        if a == b {
            a * b
        } else {
            a + b
        }
    }

    let a = 4_u8;
    let b = 4_u8;

    let result = mux_circuit(a, b);
    assert_eq!(result, a * b);

    let a = 5_u8;
    let result = mux_circuit(a, b);
    assert_eq!(result, a + b);
}

#[test]
fn test_macro_if_else4() {
    #[encrypted(execute)]
    fn mux_circuit(a: u8, b: u8) -> u8 {
        if a == b {
            let c = a * b;
            c + a
        } else {
            let x = a + b;
            x * x
        }
    }

    let a = 5_u8;
    let b = 7_u8;

    let result = mux_circuit(a, b);
    assert_eq!(result, (a + b) * (a + b));
}

#[ignore = "division not yet supported"]
#[test]
fn test_macro_division() {
    #[encrypted(execute)]
    fn division(a: u8, b: u8) -> u8 {
        a / b
    }

    let a = 20_u8;
    let b = 5_u8;

    let result = division(a, b);
    assert_eq!(result, a / b);
}

#[ignore = "modulo not yet supported"]
#[test]
fn test_macro_remainder() {
    #[encrypted(execute)]
    fn remainder(a: u8, b: u8) -> u8 {
        a % b
    }

    let a = 20_u8;
    let b = 5_u8;

    let result = remainder(a, b);
    assert_eq!(result, a % b);
}

#[test]
fn test_macro_nested_arithmetic() {
    #[encrypted(execute)]
    fn nested_arithmetic(a: u8, b: u8, c: u8, d: u8) -> u8 {
        let res = a * b;
        let res = res + c;
        res - d
    }

    let a = 2_u8;
    let b = 5_u8;
    let c = 3_u8;
    let d = 4_u8;

    let result = nested_arithmetic(a, b, c, d);
    assert_eq!(result, a * b + c - d);
}

// test bitwise operations
#[test]
fn test_macro_bitwise_and() {
    #[encrypted(execute)]
    fn bitwise_and(a: u8, b: u8) -> u8 {
        a & b
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = bitwise_and(a, b);
    assert_eq!(result, a & b);
}

#[test]
fn test_macro_bitwise_or() {
    #[encrypted(execute)]
    fn bitwise_or(a: u8, b: u8) -> u8 {
        a | b
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = bitwise_or(a, b);
    assert_eq!(result, a | b);
}

#[test]
fn test_macro_bitwise_xor() {
    #[encrypted(execute)]
    fn bitwise_xor(a: u8, b: u8) -> u8 {
        a ^ b
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = bitwise_xor(a, b);
    assert_eq!(result, a ^ b);
}

#[test]
fn test_macro_bitwise_not() {
    #[encrypted(execute)]
    fn bitwise_not(a: u8) -> u8 {
        !a
    }

    let a = 2_u8;

    let result = bitwise_not(a);
    assert_eq!(result, !a);
}

#[test]
fn test_macro_bitwise_nand() {
    #[encrypted(execute)]
    fn bitwise_nand(a: u8, b: u8) -> u8 {
        let and = a & b;
        !and
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = bitwise_nand(a, b);
    assert_eq!(result, !(a & b));
}

#[test]
fn test_macro_bitwise_nor() {
    #[encrypted(execute)]
    fn bitwise_nor(a: u8, b: u8) -> u8 {
        let or = a | b;
        !or
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = bitwise_nor(a, b);
    assert_eq!(result, !(a | b));
}

#[test]
fn test_macro_bitwise_xnor() {
    #[encrypted(execute)]
    fn bitwise_xnor(a: u8, b: u8) -> u8 {
        let xor = a ^ b;
        !xor
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = bitwise_xnor(a, b);
    assert_eq!(result, !(a ^ b));
}

#[test]
fn test_macro_equal() {
    #[encrypted(execute)]
    fn equal(a: u8, b: u8) -> u8 {
        if a == b {
            a * b
        } else {
            a + b
        }
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = equal(a, b);
    assert_eq!(result, a + b);
}

#[test]
fn test_macro_not_equal() {
    #[encrypted(execute)]
    fn not_equal(a: u8, b: u8) -> u8 {
        if a != b {
            a * b
        } else {
            a + b
        }
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = not_equal(a, b);
    assert_eq!(result, a * b);
}

#[test]
fn test_macro_greater_than() {
    #[encrypted(execute)]
    fn greater_than(a: u8, b: u8) -> u8 {
        if a > b {
            a * b
        } else {
            a + b
        }
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = greater_than(a, b);
    assert_eq!(result, a + b);

    let a = 3_u8;
    let result = greater_than(a, b);
    assert_eq!(result, a + b);

    let a = 4_u8;
    let result = greater_than(a, b);
    assert_eq!(result, a * b);
}

#[test]
fn test_macro_greater_than_or_equal() {
    #[encrypted(execute)]
    fn greater_than_or_equal(a: u8, b: u8) -> u8 {
        if a >= b {
            a * b
        } else {
            a + b
        }
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = greater_than_or_equal(a, b);
    assert_eq!(result, a + b);

    let a = 3_u8;
    let result = greater_than_or_equal(a, b);
    assert_eq!(result, a * b);

    let a = 4_u8;
    let result = greater_than_or_equal(a, b);
    assert_eq!(result, a * b);
}

#[test]
fn test_macro_less_than() {
    #[encrypted(execute)]
    fn less_than(a: u8, b: u8) -> u8 {
        if a < b {
            a * b
        } else {
            a + b
        }
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = less_than(a, b);
    assert_eq!(result, a * b);

    let a = 3_u8;
    let result = less_than(a, b);
    assert_eq!(result, a + b);

    let a = 4_u8;
    let result = less_than(a, b);
    assert_eq!(result, a + b);
}

#[test]
fn test_macro_less_than_or_equal() {
    #[encrypted(execute)]
    fn less_than_or_equal(a: u8, b: u8) -> u8 {
        if a <= b {
            a * b
        } else {
            a + b
        }
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = less_than_or_equal(a, b);
    assert_eq!(result, a * b);

    let a = 3_u8;
    let result = less_than_or_equal(a, b);
    assert_eq!(result, a * b);

    let a = 4_u8;
    let result = less_than_or_equal(a, b);
    assert_eq!(result, a + b);
}

#[test]
fn test_macro_bool_return() {
    #[encrypted(execute)]
    fn equal(a: u8, b: u8) -> bool {
        a == b
    }

    let a = 2_u8;
    let b = 3_u8;

    let result = equal(a, b);
    assert!(!result);
}

// div
#[test]
fn test_macro_div() {
    #[encrypted(execute)]
    fn div(a: u8, b: u8) -> u8 {
        a / b
    }

    let a = 20_u8;
    let b = 5_u8;

    let result = div(a, b);
    assert_eq!(result, a / b);
}

#[test]
fn test_macro_div_with_remainder() {
    #[encrypted(execute)]
    fn div(a: u8, b: u8) -> u8 {
        a / b
    }

    let a = 20_u8;
    let b = 3_u8;

    let result = div(a, b);
    assert_eq!(result, a / b);
}

#[test]
fn test_macro_div_with_remainder2() {
    #[encrypted(execute)]
    fn div(a: u8, b: u8) -> u8 {
        a / b
    }

    let a = 20_u8;
    let b = 7_u8;

    let result = div(a, b);
    assert_eq!(result, a / b);
}

// rem
#[test]
fn test_macro_rem() {
    #[encrypted(execute)]
    fn rem(a: u8, b: u8) -> u8 {
        a % b
    }

    let a = 20_u8;
    let b = 5_u8;

    let result = rem(a, b);
    assert_eq!(result, a % b);
}

#[test]
fn test_macro_rem_with_remainder() {
    #[encrypted(execute)]
    fn rem(a: u8, b: u8) -> u8 {
        a % b
    }

    let a = 20_u8;
    let b = 3_u8;

    let result = rem(a, b);
    assert_eq!(result, a % b);
}

#[test]
fn test_macro_constants() {
    #[encrypted(execute)]
    fn constants(a: u8) -> u8 {
        a + 20
    }

    let a = 10_u8;
    let result = constants(a);
    assert_eq!(result, 30_u8);
}

#[test]
fn test_macro_embedded_constants() {
    #[encrypted(execute)]
    fn embedded_constants(a: u8) -> u8 {
        let B = 20;
        a + B
    }

    let a = 10_u8;
    let result = embedded_constants(a);
    assert_eq!(result, 30_u8);
}

#[test]
fn test_order_of_operations() {
    #[encrypted(execute)]
    fn order_of_operations(a: u16, b: u16, c: u16) -> u16 {
        a + b * c
    }

    let a = 10_u16;
    let b = 20_u16;
    let c = 30_u16;
    let result = order_of_operations(a, b, c);
    assert_eq!(result, 610_u16);
}

#[test]
fn test_order_of_operations2() {
    #[encrypted(execute)]
    fn order_of_operations(a: u16, b: u16, c: u16) -> u16 {
        (a + b) * c
    }

    let a = 10_u16;
    let b = 20_u16;
    let c = 30_u16;
    let result = order_of_operations(a, b, c);
    assert_eq!(result, 900);
}

#[test]
fn test_add_assign() {
    #[encrypted(execute)]
    fn add_assign(a: u8, b: u8) -> u8 {
        let c = a;
        c += b
    }

    let a = 10_u8;
    let b = 20_u8;
    let result = add_assign(a, b);
    assert_eq!(result, 30_u8);
}

#[test]
fn test_sub_assign() {
    #[encrypted(execute)]
    fn sub_assign(a: u8, b: u8) -> u8 {
        let c = a;
        c -= b
    }

    let a = 10_u8;
    let b = 20_u8;
    let result = sub_assign(a, b);
    assert_eq!(result, 246_u8);
}

#[test]
fn test_mul_assign() {
    #[encrypted(execute)]
    fn mul_assign(a: u8, b: u8) -> u8 {
        let c = a;
        c *= b
    }

    let a = 10_u8;
    let b = 20_u8;
    let result = mul_assign(a, b);
    assert_eq!(result, 200_u8);
}

#[test]
fn test_div_assign() {
    #[encrypted(execute)]
    fn div_assign(a: u8, b: u8) -> u8 {
        let c = a;
        c /= b
    }

    let a = 10_u8;
    let b = 20_u8;
    let result = div_assign(a, b);
    assert_eq!(result, 0_u8);
}

#[test]
fn test_rem_assign() {
    #[encrypted(execute)]
    fn rem_assign(a: u8, b: u8) -> u8 {
        let c = a;
        c %= b
    }

    let a = 10_u8;
    let b = 20_u8;
    let result = rem_assign(a, b);
    assert_eq!(result, 10_u8);
}

#[test]
fn test_bitand_assign() {
    #[encrypted(execute)]
    fn bitand_assign(a: u8, b: u8) -> u8 {
        let c = a;
        c &= b
    }

    let a = 10_u8;
    let b = 20_u8;
    let result = bitand_assign(a, b);
    assert_eq!(result, 0_u8);
}

#[test]
fn test_bitor_assign() {
    #[encrypted(execute)]
    fn bitor_assign(a: u8, b: u8) -> u8 {
        let c = a;
        c |= b
    }

    let a = 10_u8;
    let b = 20_u8;
    let result = bitor_assign(a, b);
    assert_eq!(result, 30_u8);
}

#[test]
fn test_bitxor_assign() {
    #[encrypted(execute)]
    fn bitxor_assign(a: u8, b: u8) -> u8 {
        let c = a;
        c ^= b
    }

    let a = 10_u8;
    let b = 20_u8;
    let result = bitxor_assign(a, b);
    assert_eq!(result, 30_u8);
}

#[test]
fn test_if_elif_else() {
    #[encrypted(execute)]
    fn if_statement(a: u8) -> u8 {
        if a > 100 {
            a + 1
        } else if a > 50 {
            a + 2
        } else {
            a
        }
    }

    let a = 60_u8;
    let result = if_statement(a);
    assert_eq!(result, 62_u8);

    let a = 110_u8;
    let result = if_statement(a);
    assert_eq!(result, 111_u8);

    let a = 40_u8;
    let result = if_statement(a);
    assert_eq!(result, 40_u8);
}

#[test]
fn test_nested_if() {
    #[encrypted(execute)]
    fn nested_if(a: u8) -> u8 {
        if a > 100 {
            if a > 200 {
                a + 1
            } else {
                a + 2
            }
        } else {
            a
        }
    }

    let a = 150_u8;
    let result = nested_if(a);
    assert_eq!(result, 152_u8);

    let a = 250_u8;
    let result = nested_if(a);
    assert_eq!(result, 251_u8);

    let a = 50_u8;
    let result = nested_if(a);
    assert_eq!(result, 50_u8);
}

#[test]
fn test_nested_if_else() {
    #[encrypted(execute)]
    fn nested_if_else(a: u8) -> u8 {
        if a > 100 {
            if a > 200 {
                a + 1
            } else {
                a + 2
            }
        } else {
            if a > 50 {
                a + 3
            } else {
                a + 4
            }
        }
    }

    let a = 150_u8;
    let result = nested_if_else(a);
    assert_eq!(result, 152_u8);

    let a = 250_u8;
    let result = nested_if_else(a);
    assert_eq!(result, 251_u8);

    let a = 60_u8;
    let result = nested_if_else(a);
    assert_eq!(result, 63_u8);

    let a = 40_u8;
    let result = nested_if_else(a);
    assert_eq!(result, 44_u8);
}

#[test]
fn test_nested_if_else_if() {
    #[encrypted(execute)]
    fn nested_if_else_if(a: u8) -> u8 {
        if a > 100 {
            if a > 200 {
                a + 1
            } else {
                a + 2
            }
        } else if a > 50 {
            a + 3
        } else {
            a + 4
        }
    }

    let a = 150_u8;
    let result = nested_if_else_if(a);
    assert_eq!(result, 152_u8);

    let a = 250_u8;
    let result = nested_if_else_if(a);
    assert_eq!(result, 251_u8);

    let a = 60_u8;
    let result = nested_if_else_if(a);
    assert_eq!(result, 63_u8);

    let a = 40_u8;
    let result = nested_if_else_if(a);
    assert_eq!(result, 44_u8);
}

#[test]
fn test_if_else() {
    #[encrypted(execute)]
    fn if_else(a: u8) -> u8 {
        if a > 100 {
            a + 1
        } else {
            a + 2
        }
    }

    let a = 150_u8;
    let result = if_else(a);
    assert_eq!(result, 151_u8);

    let a = 50_u8;
    let result = if_else(a);
    assert_eq!(result, 52_u8);
}

#[test]
fn test_macro_bool_literal() {
    #[encrypted(execute)]
    fn boolean_literal(a: bool) -> bool {
        let x = false;
        let y = true;

        if a {
            x
        } else {
            y
        }
    }

    let bool1 = true;
    let result = boolean_literal(bool1);
    assert_eq!(result, !bool1);
}

#[test]
fn test_macro_bool_literal2() {
    #[encrypted(execute)]
    fn boolean_literal2(a: bool) -> bool {
        if a {
            false
        } else {
            true
        }
    }

    let bool1 = false;
    let result = boolean_literal2(bool1);
    assert_eq!(result, !bool1);
}

#[test]
fn macro_test_if_assign() {
    #[encrypted(execute)]
    fn if_test(a: u8) -> u8 {
        let y = 22;

        let mut c = 100;
        if a == 1 {
            c = c + 1;
            c
        } else {
            c = y + 1;
            c
        }
    }

    let a = 1_u8;
    let result = if_test(a);
    assert_eq!(result, 101);

    let a = 2_u8;
    let result = if_test(a);
    assert_eq!(result, 23);
}

#[test]
fn macro_test_assignment2() {
    #[encrypted(execute)]
    fn assignment_test2(a: u8) -> u8 {
        let mut x = 11;
        x = a + 1;
        x
    }

    let a = 42_u8;
    let result = assignment_test2(a);
    assert_eq!(result, 43);
}

#[test]
fn macro_test_assignment() {
    #[encrypted(execute)]
    fn assignment_test(a: u8) -> u8 {
        let mut x = 11;
        x = a;
        x
    }

    let a = 42_u8;
    let result = assignment_test(a);
    assert_eq!(result, 42);
}

#[test]
fn test_macro_match() {
    #[encrypted(execute)]
    fn match_test(a: u8) -> u8 {
        match a {
            1 => 7,
            2 => 8,
            3 => 9,
            _ => 10,
        }
    }

    let a = 1_u8;
    let result = match_test(a);
    assert_eq!(result, 7_u8);

    let a = 2_u8;
    let result = match_test(a);
    assert_eq!(result, 8_u8);

    let a = 3_u8;
    let result = match_test(a);
    assert_eq!(result, 9_u8);

    let a = 4_u8;
    let result = match_test(a);
    assert_eq!(result, 10_u8);
}

#[test]
fn test_macro_match_with_expr() {
    #[encrypted(execute)]
    fn match_test_with_expr(a: u8) -> u8 {
        match a {
            1 => {
                let b = 5;
                b + 2
            }
            2 => 8,
            3 => 9,
            _ => 10,
        }
    }

    let a = 1_u8;
    let result = match_test_with_expr(a);
    assert_eq!(result, 7_u8);

    let a = 2_u8;
    let result = match_test_with_expr(a);
    assert_eq!(result, 8_u8);

    let a = 3_u8;
    let result = match_test_with_expr(a);
    assert_eq!(result, 9_u8);

    let a = 4_u8;
    let result = match_test_with_expr(a);
    assert_eq!(result, 10_u8);
}

#[test]
fn test_macro_match_with_block() {
    #[encrypted(execute)]
    fn match_test_with_block(a: u8) -> u8 {
        match a {
            1 => {
                let b = 5;
                b + 2
            }
            2 => {
                let c = 6;
                c + 2
            }
            3 => {
                let d = 7;
                d + 2
            }
            _ => 10,
        }
    }

    let a = 1_u8;
    let result = match_test_with_block(a);
    assert_eq!(result, 7_u8);

    let a = 2_u8;
    let result = match_test_with_block(a);
    assert_eq!(result, 8_u8);

    let a = 3_u8;
    let result = match_test_with_block(a);
    assert_eq!(result, 9_u8);

    let a = 4_u8;
    let result = match_test_with_block(a);
    assert_eq!(result, 10_u8);
}

#[test]
fn macro_test_if_with_consts() {
    #[encrypted(execute)]
    fn if_test(a: u8) -> u8 {
        if a == 42 {
            a + 1
        } else {
            54
        }
    }

    let a = 42_u8;
    let result = if_test(a);
    assert_eq!(result, 43);

    let a = 43_u8;
    let result = if_test(a);
    assert_eq!(result, 54);
}

#[test]
fn macro_test_if_with_consts2() {
    #[encrypted(execute)]
    fn if_test(a: u8) -> u8 {
        let accum = 0;
        let if_else = if a == 42 {
            let accum2 = accum + 11;
            43 + accum2
        } else if a == 32 {
            let accum2 = accum + 22;
            33 + accum2
        } else {
            let accum2 = accum + 33;
            54 + accum2
        };
        if_else
    }

    let a = 42_u8;
    let result = if_test(a);
    assert_eq!(result, 54);

    let a = 32_u8;
    let result = if_test(a);
    assert_eq!(result, 55);

    let a = 0_u8;
    let result = if_test(a);
    assert_eq!(result, 87);
}

#[test]
fn macro_test_if_with_consts1() {
    #[encrypted(execute)]
    fn if_test(a: u8) -> u8 {
        let ACCUM = 5;

        // Test if the if-else statement is correctly generated
        let if_else = if a == 42 {
            let accum2 = ACCUM + 1;
            43 + accum2
        } else if a == 32 {
            let accum2 = ACCUM + 2;
            33 + accum2
        } else {
            let accum2 = ACCUM + 3;
            2 + accum2
        };
        if_else
    }

    let a = 42_u8;
    let result = if_test(a);
    assert_eq!(result, 49);

    let a = 32_u8;
    let result = if_test(a);
    assert_eq!(result, 40);

    let a = 12_u8;
    let result = if_test(a);
    assert_eq!(result, 10);

    let a = 0_u8;
    let result = if_test(a);
    assert_eq!(result, 10);
}

#[test]
fn macro_test_match_with_consts() {
    #[encrypted(execute)]
    fn if_test(a: u8) -> u8 {
        let FIRST_ARM = 49;
        let SECOND_ARM = 40;

        // Test if the match statement is correctly generated
        let match_out = match a {
            FIRST_ARM => {
                let MULTIPLIER = 2;
                a * MULTIPLIER
            }
            SECOND_ARM => {
                let MULTIPLIER = 3;
                a * MULTIPLIER
            }
            _ => {
                let MULTIPLIER = 1;
                a * MULTIPLIER
            }
        };

        match_out
    }

    let a = 49_u8;
    let result = if_test(a);
    assert_eq!(result, 98);

    let a = 40_u8;
    let result = if_test(a);
    assert_eq!(result, 120);

    let a = 12_u8;
    let result = if_test(a);
    assert_eq!(result, 12);
}

#[test]
fn macro_test_if_and_match_with_consts() {
    #[encrypted(execute)]
    fn if_test(a: u8) -> u8 {
        let ACCUM = 5;

        // Test if the if-else statement is correctly generated
        let if_else = if a == 42 {
            let accum2 = ACCUM + 1;
            43 + accum2
        } else if a == 32 {
            let accum2 = ACCUM + 2;
            33 + accum2
        } else {
            let accum2 = ACCUM + 3;
            2 + accum2
        };

        let FIRST_ARM = 49;
        let SECOND_ARM = 40;

        // Test if the match statement is correctly generated
        let match_out = match &if_else {
            FIRST_ARM => {
                let MULTIPLIER = 2;
                a * MULTIPLIER
            }
            SECOND_ARM => {
                let MULTIPLIER = 3;
                a * MULTIPLIER
            }
            _ => {
                let MULTIPLIER = 1;
                a * MULTIPLIER
            }
        };

        match_out + if_else
    }

    let a = 42_u8;
    let result = if_test(a);
    assert_eq!(result, 133);

    let a = 32_u8;
    let result = if_test(a);
    assert_eq!(result, 136);

    let a = 12_u8;
    let result = if_test(a);
    assert_eq!(result, 22);
}

#[test]
fn test_macro_if_bool() {
    #[encrypted(execute)]
    fn if_test(a: bool) -> bool {
        if a {
            true
        } else {
            false
        }
        // defaults to false
    }

    let a = true;
    let result = if_test(a);
    assert!(result);

    let a = false;
    let result = if_test(a);
    assert!(!result);
}

#[test]
fn test_macro_if() {
    #[encrypted(execute)]
    fn if_test(a: u8) -> u8 {
        if a <= 50 {
            25
        } else {
            100
        }
    }

    let a = 42_u8;
    let result = if_test(a);
    assert_eq!(result, 25);

    let a = 132_u8;
    let result = if_test(a);
    assert_eq!(result, 100);
}
