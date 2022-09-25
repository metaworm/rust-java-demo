
Rust和Java交互的Demo工程

## 构建 & 运行

1. `cargo build` 编译Rust的JNI模块
2. `mvn compile` 编译Java代码
3. `java -Djava.library.path=target/debug -classpath target/classes pers.metaworm.RustJNI` 运行demo

## 文章

[Rust与Java交互-JNI模块编写-实践总结](https://zhuanlan.zhihu.com/p/568062165)