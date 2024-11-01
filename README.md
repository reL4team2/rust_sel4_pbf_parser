# 简介
关于这个工具，目标是用于解析sel4的pbf文件生成对应的rust代码
关于pbf文件的内容，他原本是一个bf文件，但是这个bf文件会在编译过程中根据条件编译的情况，按照给定的条件，生成对应的只包含相关bitfield定义的pbf文件
对于这部分的bitfield的定义方法，见sel4如下的文档：
https://github.com/seL4/seL4/blob/master/tools/bitfield_gen.md#bit-field-generator-by-example

在原本的rel4的代码中，这部分解析的功能在plus_define_bitfield宏定义中。然而，在内核代码升级过程中发现了一个错误，见
https://docs.qq.com/doc/DY0pabGhkUm53aE10
这个错误的来源是这样的，原本某个48位的地址，该地址需要截断低20位，得到高28位
然后将该28位，放置在[47:20]的位置。
但是由于原本的plus_define_bitfield宏的解析的数值，都是手工填入的，这部分的填入无法保证完全正确的解析。
所以当初内核代码升级的时候发现，该48位的地址，直接放在[47:20]的位置，没有所谓的截断低20位的事情。
在原本的代码中没有出错的原因是测例本身不够全面，只有一处出现了高于28位的地址，这个地方，显然就直接被截断了，最后写入和读取的值就不对应了。该处虽然被截断，但是后续代码的判断逻辑，让他虽然出了错，但是后续仍然正确工作了
但是当我升级了代码之后，这个原本后续代码可以正常工作的逻辑，现在不work了，就导致了报错。

为此，我们不得不使用代码生成的方式，直接从pbf文件生成对应的rust代码。并在rel4中，使用新的方式去工作
# 做的改动和提升
在最开始，我以为我直接使用rust-sel4的解析库，稍加改造便可正常工作

该解析库的位置在https://github.com/seL4/rust-sel4/tree/main/crates/sel4/bitfield-parser

但是，我不确定其在用户态的使用场景，但是在内核态的解析，这是并不成功的。
# 在rel4中的使用方法