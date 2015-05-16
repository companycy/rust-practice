
fn main() {
    let mut var1 = 100i;
    let mut var2 = 200i;

    let mut p1 = &mut var1;  //左边的mut表示这个指针本身可以改变，
    //右边的mut表示指针指向的内容可以修改
    *p1 = 10;
    {
        p1 = &mut var2;
        *p1 = 20;
    }
    let p2 = &var2;
    println!("{} {}", *p1, *p2);
}
