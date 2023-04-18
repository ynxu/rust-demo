/// 变量绑定与解构
/// 变量赋值, 可变/不可变
#[test]
fn test01(){
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6;
    println!("The value of x is: {}", x);

    let mut y:i32 = 6;
    println!("The value of y is: {}", y);
    y = 7;
    println!("The value of y is: {}", y);
}

/// 变量解构
#[test]
fn test02(){
    let (a,mut b):(bool,bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a={},b={}",a,b);
    b = true;
    println!("a={},b={}",a,b);
    assert_eq!(a,b)
}
/// 结构式赋值

struct Struct{
    e: i32
}
#[test]
fn test03(){
    let (a,b,c,d,e);
    (a,b) = (1,2);
    [..,c,d,_] = [1,2,3,4,5];
    Struct{e,..} = Struct{e:5};
    assert_eq!([1,2,3,4,5],[a,b,c,d,e]);
}
/// 常量
const _MAX_POINTS: u32 = 100_000;

/// 变量遮蔽
#[test]
fn test04(){
    let x: i32 =1;
    let x: i32 = x + 1;
    {
        let x: i32 = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    let spaces = "     ";
    println!("echo str: [{}]",spaces);
    let spaces  = spaces.len();
    println!("echo str length: {}",spaces);
    
    // let mut space = "     ";
    // println!("echo str: [{}]",space);
    // let space  = space.len();
    // println!("echo str length: {}",space);
}
