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

举个例子，我试图用生成的pbf文件直接调用原版的代码，他会因为各种问题产生assert
在用户态，他的地址基本高16位绝对是全0，但是在内核态则不是，他的高16位地址不会是全0，因此需要做位扩展。
但是原本的pbf解析器不知道为什么没有，等等问题，因此，我最终决定在原有的基础上进行修补。

以下是我做的全部修补改动说明。

 - 1/对于field_high的支持
对于这部分，也就是我上面说的那个问题，他需要在特定情况下计算出需要进行位扩展的位数并进行扩展
但是需要注意到底扩展多少位的问题。这个时候也跟在文件中声明的base有关
```
base 64(39,1)   #riscv
base 64(48,1)   #aarch64
```
这是因为riscv使用sv39，而aarch64使用48位的原因，后面那个1表明是否需要位扩展，如果设为0，后面就算有field_high，也不需要扩展了。
而这个base的使用，在原版本的代码中simplified.rs的simplify_base函数中还有相关的assert，期望是后面那个需要位扩展的设为0，并且期望base后面的64和括号里的39/48应该是等同的。
显然这在pbf解析过程中就是忽略了这部分。
因此，注释掉这部分，并在field结构体中增加相应的字段描述这种行为，并在bf/mod.rs文件中进行判定时，额外增加相应代码。

 - 2/
在structure.pbf文件中存在这么一段
```
tagged_union pte pte_type(pte_hw_type, pte_sw_type) {
    tag pte_table (3, 0)
    tag pte_page (1, 0)
    tag pte_4k_page (3, 1)
    tag pte_invalid (0, 0)
}
```
在解析的时候同样会触发问题，这段含义是pte_type里面其实包含hw和sw两种type，对应的值分别为3，1，3，0和0，0，1，0，原因是原先的不太支持这种语法格式。
对这部分的操作我选择了去除，理由是，这两个字段在上面的定义中，两个字段都存在
例如
```
block pte_invalid {
    padding 5
    field pte_sw_type 1
    padding 56
    field pte_hw_type 2
}
```
而由于Rust本身的类型系统，如果按照现有的代码框架，我势必为同一个pte_invalid的生成两种类型，还要互相转换，非常讨厌，干脆直接特判去掉算了。
 - 3/
```
block thread_state(blockingIPCBadge, blockingIPCCanGrant,
                   blockingIPCCanGrantReply, blockingIPCIsCall,
                   tcbQueued, blockingObject,
                   tsType) {
    field blockingIPCBadge 64
    padding 60
    field blockingIPCCanGrant 1
    field blockingIPCCanGrantReply 1
    field blockingIPCIsCall 1
    field tcbQueued 1
    padding 16
    field_high blockingObject 44
    field tsType 4
}
```
还是拿这个上面例子进行举例。
在后面增加了一个括号，括号内的内容，其实和下面的一样
而这个解析器本身并不期望有这个括号里的内容。我同样忽略掉了这个问题，让他可以运行
 - 4/
修改type
```
block asid_map_none {
    padding 63
    field type 1
}
--- hw_vmids are required in hyp mode
block asid_map_vspace {
    padding 16
    field_high vspace_root 36
    padding 11
    field type 1
}
tagged_union asid_map type {
    tag asid_map_none 0
    tag asid_map_vspace 1
}
```
在rust中，type是一个关键字，但C中不是，我想说到这里大家应该能明白过来，解析的时候，直接使用type这个字段就是会出错。我在解析的时候忽略了这个。
 - 5/
增加某个tag到具体的类型的转换
这是我在实践中需要的，原先的splay和unsplay，他实际上会存在一个复制的问题，如果不使用clone或者添加copy trait，就无法过编译器，如果添加了，也是不对的（写入的时候存在问题）
```
to_unsplay_type.push(quote! {
	pub fn #tag_value_ident(capability: &Self) -> &mut #tag_value_ident{
	unsafe { (capability as *const _ as *mut #tag_value_ident).as_mut().unwrap() }
	}
});
```
反正不会写入到正确的slot里面
因此我直接指针强转。
 - 6/
对于copy trait
在最开始原始的rust-sel4中不存在生成的类型包含copy trait。（而比如cap类型在没有使用这个工具生成的代码中，使用plus_defined_bitfield宏生成的代码，里面包含copy trait）
但是没有copy trait很多地方必须重新考虑，而不是简单写个替换，最开始我很傻得直接增加copy trait，但是后来发现写入位置不对。
所以又去除了，这一点请务必留意。
 - 7/
对于缺少的工具引用的问题。
由于这个工具生成的是一个Bitfield相关的类型，这个Bitfield也是定义在rust-sel4中的一个crate。
因此，必须在这里添加对于这个Bitfield的引用，相关的代码我放在parser.rs中。

以上就是我在代码中实际改动的方面。具体代码请结合实际分析在此不一一标注。
# 测试及在rel4中的使用方法
以下我描述我是如何在rel4中测试并调用这些代码的
首先是测试，这部分测试指的是能够正确的解析并生成rust的代码
这无需集成到rel4内核中再去测试
这部分的代码其实在main.rs中，只需要
将rel4已经生成的riscv64或者aarch64版本的所有pbf文件都放到当前目录下的pbf文件夹下即可自动解析
（当然，解析完之后会在pbf文件夹下生成对应的rs文件，每次重新生成前需要手动清理，反正只是验证，我就不多折腾了，能跑）
然后直接运行
```
cargo run
```
即可
在pbf文件夹下看到对应的rs文件
如果解析器需要debug，也需要使用这个方式。
## debug和使用的分离
虽然目前这个工具可以正常在rel4中使用， 但是，将来某个时刻，可能仍然免不了需要debug和修改。
目前的设计方案是，在main分支里面包含了main.rs，可以直接按照上述的方法进行debug，同时新开了一个master分支，该分支用于rel4中使用，如果需要和rel4进行联调，那么只需要在代码中指定分支为main即可进行debug

只有在main分支按照需要debug过了，再同步到master分支，作为正式使用的版本，以防其他人在使用rel4的时候出现该解析器的错误。

## 关于在rel4下使用该pbf解析器
在我的修改下，src/parser.rs中有个pbf_parser函数
该函数是整个工具的入口函数，期望的内容是，给出一个输入文件夹的路径，和一个输出文件夹的路径
目标是生成在输出文件夹下的对应rs文件。
因此在rel4中，首先我确定生成的代码的位置应该放在common中，同时他期望在正式编译之前就生成对应的rs文件，从而在该文件夹中增加了一个build.rs。

这里面我做的事情主要就是指定对应的文件夹，parser的目标文件夹实际上是在对应的生成的文件夹下的，随后我根据需要，再去将他们复制到sel4_common文件夹下。
## 其他
最后一个注意点，我们使用解析器生成接口的函数调用，函数参数顺序可能发生变化。
（也因此坑了我很久）