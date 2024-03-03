pub type RustFn = fn(&dyn LuaState) -> usize;

pub trait LuaState {
    /* 基本栈操作 */
    /// 返回栈顶元素的索引。
    ///
    /// 返回值：栈顶元素的索引。
    fn get_top(&self) -> isize;

    /// 将一个相对索引转换为绝对索引。
    ///
    /// 参数：
    /// * `idx` - 要转换的相对索引。
    ///
    /// 返回值：转换后的绝对索引。
    fn abs_index(&self, idx: isize) -> isize;

    /// 检查栈是否还有足够的空间来容纳 `n` 个元素，如果没有，则尝试扩展栈。
    ///
    /// 参数：
    /// * `n` - 需要的元素数量。
    ///
    /// 返回值：如果栈有足够的空间或扩展成功，则返回 `true`，否则返回 `false`。
    fn check_stack(&mut self, n: usize) -> bool;

    /// 从栈顶弹出 `n` 个元素。
    ///
    /// 参数：
    /// * `n` - 要弹出的元素数量。
    fn pop(&mut self, n: usize);

    /// 复制栈上的一个元素到另一个位置。
    ///
    /// 参数：
    /// * `from_idx` - 要复制的元素的索引。
    /// * `to_idx` - 复制的目标位置的索引。
    fn copy(&mut self, from_idx: isize, to_idx: isize);

    /// 将指定索引处的元素推送到栈顶。
    ///
    /// 参数：
    /// * `idx` - 要推送的元素的索引。
    fn push_value(&mut self, idx: isize);

    /// 用栈顶元素替换指定索引处的元素。
    ///
    /// 参数：
    /// * `idx` - 要替换的元素的索引。
    fn replace(&mut self, idx: isize);

    /// 将栈顶元素插入到指定的位置，并移动该位置及其上方的元素。
    ///
    /// 参数：
    /// * `idx` - 插入的目标位置的索引。
    fn insert(&mut self, idx: isize);

    /// 移除指定索引处的元素，并移动该位置上方的元素。
    ///
    /// 参数：
    /// * `idx` - 要移除的元素的索引。
    fn remove(&mut self, idx: isize);

    /// 旋转栈中的元素。将 `idx` 到栈顶的元素向上移动 `n` 个位置，使得 `idx+n` 处的元素移到 `idx` 处，原来 `idx` 到 `idx+n-1` 处的元素向上移动，原来 `idx+n` 到栈顶的元素向下移动。
    ///
    /// 参数：
    /// * `idx` - 要旋转的元素的起始索引。
    /// * `n` - 要移动的位置数量。
    fn rotate(&mut self, idx: isize, n: isize);

    /// 设置栈顶元素的索引，如果新的索引大于旧的索引，那么新的元素将被初始化为 nil，如果小于旧的索引，那么超出的元素将被弃用。
    ///
    /// 参数：
    /// * `idx` - 新的栈顶元素的索引。
    fn set_top(&mut self, idx: isize);

    /* 访问函数 (stack -> rust) */
    /// 返回给定类型的名称。
    ///
    /// 参数：
    /// * `tp` - 要获取名称的类型的 ID。
    ///
    /// 返回值：给定类型的名称。
    fn type_name(&self, tp: i8) -> &str;

    /// 返回指定索引处的值的类型 ID。
    ///
    /// 参数：
    /// * `idx` - 要获取类型的值的索引。
    ///
    /// 返回值：指定索引处的值的类型 ID。
    fn type_id(&self, idx: isize) -> i8;

    /// 检查指定索引处的值是否为 `None`。
    ///
    /// 参数：
    /// * `idx` - 要检查的值的索引。
    ///
    /// 返回值：如果指定索引处的值为 `None`，则返回 `true`，否则返回 `false`。
    fn is_none(&self, idx: isize) -> bool;

    /// 检查指定索引处的值是否为 `nil`。
    ///
    /// 参数：
    /// * `idx` - 要检查的值的索引。
    ///
    /// 返回值：如果指定索引处的值为 `nil`，则返回 `true`，否则返回 `false`。
    fn is_nil(&self, idx: isize) -> bool;

    /// 检查指定索引处的值是否为 `None` 或 `nil`。
    ///
    /// 参数：
    /// * `idx` - 要检查的值的索引。
    ///
    /// 返回值：如果指定索引处的值为 `None` 或 `nil`，则返回 `true`，否则返回 `false`。
    fn is_none_or_nil(&self, idx: isize) -> bool;

    /// 检查指定索引处的值是否为布尔值。
    ///
    /// 参数：
    /// * `idx` - 要检查的值的索引。
    ///
    /// 返回值：如果指定索引处的值为布尔值，则返回 `true`，否则返回 `false`。
    fn is_boolean(&self, idx: isize) -> bool;

    /// 检查指定索引处的值是否为整数。
    ///
    /// 参数：
    /// * `idx` - 要检查的值的索引。
    ///
    /// 返回值：如果指定索引处的值为整数，则返回 `true`，否则返回 `false`。
    fn is_integer(&self, idx: isize) -> bool;

    /// 检查指定索引处的值是否为数字。
    ///
    /// 参数：
    /// * `idx` - 要检查的值的索引。
    ///
    /// 返回值：如果指定索引处的值为数字，则返回 `true`，否则返回 `false`。
    fn is_number(&self, idx: isize) -> bool;

    /// 检查指定索引处的值是否为字符串。
    ///
    /// 参数：
    /// * `idx` - 要检查的值的索引。
    ///
    /// 返回值：如果指定索引处的值为字符串，则返回 `true`，否则返回 `false`。
    fn is_string(&self, idx: isize) -> bool;

    /// 检查指定索引处的值是否为表。
    ///
    /// 参数：
    /// * `idx` - 要检查的值的索引。
    ///
    /// 返回值：如果指定索引处的值为表，则返回 `true`，否则返回 `false`。
    fn is_table(&self, idx: isize) -> bool;

    /// 检查指定索引处的值是否为线程。
    ///
    /// 参数：
    /// * `idx` - 要检查的值的索引。
    ///
    /// 返回值：如果指定索引处的值为线程，则返回 `true`，否则返回 `false`。
    fn is_thread(&self, idx: isize) -> bool;

    /// 检查指定索引处的值是否为函数。
    ///
    /// 参数：
    /// * `idx` - 要检查的值的索引。
    ///
    /// 返回值：如果指定索引处的值为函数，则返回 `true`，否则返回 `false`。
    fn is_function(&self, idx: isize) -> bool;

    /// 检查指定索引处的值是否为 Rust 函数。
    ///
    /// 参数：
    /// * `idx` - 要检查的值的索引。
    ///
    /// 返回值：如果指定索引处的值为 Rust 函数，则返回 `true`，否则返回 `false`。
    fn is_rust_function(&self, idx: isize) -> bool;

    /// 将指定索引处的值转换为布尔值。
    ///
    /// 参数：
    /// * `idx` - 要转换的值的索引。
    ///
    /// 返回值：转换后的布尔值。
    fn to_boolean(&self, idx: isize) -> bool;

    /// 将指定索引处的值转换为整数。
    ///
    /// 参数：
    /// * `idx` - 要转换的值的索引。
    ///
    /// 返回值：转换后的整数。
    fn to_integer(&self, idx: isize) -> i64;

    /// 将指定索引处的值尝试转换为整数。
    ///
    /// 参数：
    /// * `idx` - 要转换的值的索引。
    ///
    /// 返回值：如果转换成功，则返回 `Some(i64)`，否则返回 `None`。
    fn to_integerx(&self, idx: isize) -> Option<i64>;

    /// 将指定索引处的值转换为数字。
    ///
    /// 参数：
    /// * `idx` - 要转换的值的索引。
    ///
    /// 返回值：转换后的数字。
    fn to_number(&self, idx: isize) -> f64;

    /// 尝试将指定索引处的 Lua 值转换为浮点数。如果转换成功，返回 `Some(f64)`，否则返回 `None`。
    ///
    /// 参数：
    /// * `idx` - 要转换的值的索引。
    ///
    /// 返回值：如果转换成功，返回 `Some(f64)`，否则返回 `None`。
    fn to_numberx(&self, idx: isize) -> Option<f64>;

    /// 将指定索引处的 Lua 值转换为字符串。如果值不是字符串或数字，将引发错误。
    ///
    /// 参数：
    /// * `idx` - 要转换的值的索引。
    ///
    /// 返回值：转换后的字符串。
    fn to_string(&self, idx: isize) -> String;

    /// 尝试将指定索引处的 Lua 值转换为字符串。如果值是字符串或数字，返回 `Some(String)`，否则返回 `None`。
    ///
    /// 参数：
    /// * `idx` - 要转换的值的索引。
    ///
    /// 返回值：如果转换成功，返回 `Some(String)`，否则返回 `None`。
    fn to_stringx(&self, idx: isize) -> Option<String>;

    /// 尝试将指定索引处的 Lua 值转换为 Rust 函数。如果值是 Rust 函数，返回 `Some(RustFn)`，否则返回 `None`。
    ///
    /// 参数：
    /// * `idx` - 要转换的值的索引。
    ///
    /// 返回值：如果转换成功，返回 `Some(RustFn)`，否则返回 `None`。
    fn to_rust_function(&self, idx: isize) -> Option<RustFn>;

    /* 推送函数 (rust -> stack) */
    /// 将 nil 值推送到栈顶。
    fn push_nil(&mut self);

    /// 将布尔值推送到栈顶。
    ///
    /// 参数：
    /// * `b` - 要推送的布尔值。
    fn push_boolean(&mut self, b: bool);

    /// 将整数值推送到栈顶。
    ///
    /// 参数：
    /// * `n` - 要推送的整数值。
    fn push_integer(&mut self, n: i64);

    /// 将浮点数值推送到栈顶。
    ///
    /// 参数：
    /// * `n` - 要推送的浮点数值。
    fn push_number(&mut self, n: f64);

    /// 将字符串值推送到栈顶。
    ///
    /// 参数：
    /// * `s` - 要推送的字符串值。
    fn push_string(&mut self, s: String);

    /// 将 Rust 函数推送到栈顶。
    ///
    /// 参数：
    /// * `func` - 要推送的 Rust 函数。
    fn push_rust_function(&mut self, func: RustFn);

    /// 将全局表推送到栈顶。
    fn push_global_table(&mut self);

    /* 算数和比较运算函数 */
    /// 对栈顶的两个元素执行算术运算，并将结果推送到栈顶。运算类型由 `op` 参数指定。
    ///
    /// 参数：
    /// * `op` - 指定算术运算的类型。可能的值包括 `ADD`、`SUB`、`MUL`、`MOD`、`POW`、`DIV`、`IDIV`、`BAND`、`BOR`、`BXOR`、`SHL`、`SHR`、`UNM`、`BNOT`。
    fn arith(&mut self, op: u8);

    /// 比较栈上的两个元素。比较类型由 `op` 参数指定。
    ///
    /// 参数：
    /// * `idx1` - 第一个要比较的元素的索引。
    /// * `idx2` - 第二个要比较的元素的索引。
    /// * `op` - 指定比较的类型。可能的值包括 `EQ`、`LT`、`LE`。
    ///
    /// 返回值：如果比较结果为真，返回 `true`，否则返回 `false`。
    fn compare(&mut self, idx1: isize, idx2: isize, op: u8) -> bool;

    /* 其他函数 */
    /// 计算指定索引处的 Lua 值的长度，并将结果推送到栈顶。对于字符串，这是其长度；对于表，这是适合数组部分的最大索引。
    ///
    /// 参数：
    /// * `idx` - 要计算长度的值的索引。
    fn len(&mut self, idx: isize);

    /// 连接栈顶的 `n` 个字符串值，并将结果推送到栈顶。如果 `n` 是 1，结果是一个字符串；如果 `n` 是 0，结果是一个空字符串。
    ///
    /// 参数：
    /// * `n` - 要连接的字符串值的数量。
    fn concat(&mut self, n: isize);

    /* 获取函数 (Lua -> stack) */
    /// 创建一个新的空表并将其推送到栈顶。
    fn new_table(&mut self);

    /// 创建一个新的表并将其推送到栈顶。`narr` 是预期的数组部分的大小，`nrec` 是预期的哈希部分的大小。
    ///
    /// 参数：
    /// * `narr` - 预期的数组部分的大小。
    /// * `nrec` - 预期的哈希部分的大小。
    fn create_table(&mut self, narr: usize, nrec: usize);

    /// 将指定索引处的表的值推送到栈顶。返回值表示操作的成功与否。
    ///
    /// 参数：
    /// * `idx` - 表的索引。
    ///
    /// 返回值：如果操作成功，返回 1，否则返回 0。
    fn get_table(&mut self, idx: isize) -> i8;

    /// 获取指定索引处的表中的字段，并将其推送到栈顶。返回值表示操作的成功与否。
    ///
    /// 参数：
    /// * `idx` - 表的索引。
    /// * `k` - 字段的名称。
    ///
    /// 返回值：如果操作成功，返回 1，否则返回 0。
    fn get_field(&mut self, idx: isize, k: &str) -> i8;

    /// 获取指定索引处的表中的元素，并将其推送到栈顶。返回值表示操作的成功与否。
    ///
    /// 参数：
    /// * `idx` - 表的索引。
    /// * `i` - 元素的索引。
    ///
    /// 返回值：如果操作成功，返回 1，否则返回 0。
    fn get_i(&mut self, idx: isize, i: i64) -> i8;

    /// 获取全局变量的值，并将其推送到栈顶。返回值表示操作的成功与否。
    ///
    /// 参数：
    /// * `name` - 全局变量的名称。
    ///
    /// 返回值：如果操作成功，返回 1，否则返回 0。
    fn get_global(&mut self, name: &str) -> i8;

    /* 设置函数 (stack -> Lua) */
    /// 将栈顶的值设置为指定索引处的表的值，并弹出栈顶的值。
    ///
    /// 参数：
    /// * `idx` - 表的索引。
    fn set_table(&mut self, idx: isize);

    /// 将栈顶的值设置为指定索引处的表的字段，并弹出栈顶的值。
    ///
    /// 参数：
    /// * `idx` - 表的索引。
    /// * `k` - 字段的名称。
    fn set_field(&mut self, idx: isize, k: &str);

    /// 将栈顶的值设置为指定索引处的表的元素，并弹出栈顶的值。
    ///
    /// 参数：
    /// * `idx` - 表的索引。
    /// * `i` - 元素的索引。
    fn set_i(&mut self, idx: isize, i: i64);

    /// 将栈顶的值设置为全局变量的值，并弹出栈顶的值。
    ///
    /// 参数：
    /// * `name` - 全局变量的名称。
    fn set_global(&mut self, name: &str);

    /// 注册一个 Rust 函数作为 Lua 函数。这个函数将被添加到全局环境中，可以在 Lua 代码中通过 `name` 来调用。
    ///
    /// 参数：
    /// * `name` - 函数在 Lua 中的名称。
    /// * `f` - 要注册的 Rust 函数。
    fn register(&mut self, name: &str, f: RustFn);

    /* 加载和调用函数 (加载和运行 Lua 代码) */
    /// 加载一个 Lua 代码块。这个函数将代码块编译为字节码，然后将生成的函数推送到栈顶。
    ///
    /// 参数：
    /// * `chunk` - 包含 Lua 代码块的字节向量。
    /// * `chunk_name` - 代码块的名称，用于错误消息和调试信息。
    /// * `mode` - 控制编译器的模式，可以是 "b"（只接受二进制代码块）、"t"（只接受文本代码块）或 "bt"（接受二进制或文本代码块）。
    ///
    /// 返回值：如果加载成功，返回 0；如果发生错误，返回一个非零错误码。
    fn load(&mut self, chunk: Vec<u8>, chunk_name: &str, mode: &str) -> u8;

    /// 调用一个 Lua 函数。这个函数应该在栈顶，其参数应该在其下面，参数的数量由 `nargs` 指定。函数的返回值将被推送到栈顶。
    ///
    /// 参数：
    /// * `nargs` - 函数的参数数量。
    /// * `nresults` - 期望的返回值数量。如果是 -1，那么将返回所有的结果。
    fn call(&mut self, nargs: usize, nresults: isize);
}
