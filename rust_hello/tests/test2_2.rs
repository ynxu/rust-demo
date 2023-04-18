
/// 基本类型

/// 数值类型

/// 整型溢出
#[test]
fn test01(){
    let a:u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}",b);
}
/// 浮点类型
#[test]
fn test02(){
    let a = 2.0;
    let b :f32 = 3.0;
    println!("{}",b);
}