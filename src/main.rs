fn main() {
  let mut line = String::new();
  println!("请输入你的名字:");
  let b1 = std::io::stdin().read_line(&mut line).unwrap();
  println!("你好 , {}", line);
  println!("读取的字节数为：{}", b1);
}
