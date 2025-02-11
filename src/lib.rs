#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod bindings;

#[cfg(test)]
mod tests {
    use crate::bindings::root::cparse::{self, TokenMap, calculator};
    use std::ffi::CString;
    use std::mem;
    use std::ptr;

    #[test]
    fn test_calculator() {
        unsafe {
            // 创建一个表达式的C字符串
            let expr = CString::new("2 + 3 * 4").unwrap();

            // 使用空的TokenMap
            let vars = mem::zeroed::<TokenMap>();

            // 调用计算函数 - 使用正确的TokenMap类型
            let result = calculator::calculate(
                expr.as_ptr(),
                vars,            // 传入TokenMap实例
                ptr::null(),     // 无分隔符
                ptr::null_mut(), // 不需要rest指针
            );

            // 验证结果是否正确 (2 + 3 * 4 = 14)
            assert_eq!(result.asDouble(), 14.0);
        }
    }

    #[test]
    fn test_calculator_with_vars() {
        unsafe {
            // 创建一个表达式
            let expr = CString::new("x + y").unwrap();

            // 创建一个空的TokenMap
            let vars = mem::zeroed::<TokenMap>();

            // 创建一个计算器实例
            let cal = calculator::new1(
                expr.as_ptr(),
                vars,            // 使用TokenMap实例
                ptr::null(),     // 无分隔符
                ptr::null_mut(), // 不需要rest指针
                calculator::Default(),
            );

            // 创建一个空的TokenMap作为作用域
            let scope = mem::zeroed::<TokenMap>();

            // 计算结果，设置 keep_refs 为 false
            let result = cal.eval(scope, false);

            // 验证结果
            assert_eq!(result.asDouble(), 0.0); // x + y = 0 因为变量未定义
        }
    }
}
