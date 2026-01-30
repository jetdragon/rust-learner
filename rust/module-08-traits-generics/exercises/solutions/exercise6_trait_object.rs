//! # 练习 6: Trait 对象 - 解答
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: trait, Box
//! 学习目标:
//!   - 使用 trait 对象实现动态分发
//!   - 理解 &dyn Trait 和 Box<dyn Trait>

/// Draw trait - 可绘制对象的接口
///
/// 这是"对象安全"（object-safe）的 trait:
/// - 不需要泛型参数
/// - 不有 Self 类型（除了 &self 和 &mut self）
/// - 没有 return Self 类型的方法
pub trait Draw {
    /// 绘制对象
    fn draw(&self);
}

/// Button 结构体 - 按钮组件
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Drawing button: {} ({}x{})",
            self.label, self.width, self.height
        );
    }
}

/// TextField 结构体 - 文本输入框
pub struct TextField {
    pub width: u32,
    pub height: u32,
    pub placeholder: String,
}

// ============= 解决方案 =============

/// 为 TextField 实现 Draw trait
///
/// 实现说明:
/// - 与 Button 实现相同的 trait
/// - 可以在同一个集合中使用
/// - 这展示了多态性
impl Draw for TextField {
    fn draw(&self) {
        println!(
            "Drawing text field: {} ({}x{}) with placeholder '{}'",
            self.width, self.height, self.placeholder
        );
    }
}

/// 创建包含不同类型组件的函数
///
/// 使用 &dyn Draw trait 对象:
/// - &dyn Draw 是 trait 对象的引用
/// - dyn 关键字表示动态分发
/// - 可以存储不同类型的组件
///
/// # 内存布局
///
/// Trait 对象是"胖指针"（fat pointer）:
/// - 包含数据指针
/// - 包含 vtable（虚函数表）指针
/// - 大小为两个 usize（16 字节在 64 位系统）
pub fn run_drawables(components: &[&dyn Draw]) {
    println!("=== Drawing {} components ===", components.len());
    for component in components {
        component.draw();
    }
}

// ============= 扩展示例 =============

/// 使用 Box<dyn Draw> 拥有 trait 对象
///
/// Box<dyn Draw> 的优势:
/// - 拥有数据，不依赖引用生命周期
/// - 可以存储在集合中
/// - 可以传递所有权
pub struct ComponentList {
    components: Vec<Box<dyn Draw>>,
}

impl ComponentList {
    pub fn new() -> Self {
        ComponentList {
            components: Vec::new(),
        }
    }

    pub fn add(&mut self, component: Box<dyn Draw>) {
        self.components.push(component);
    }

    pub fn draw_all(&self) {
        println!("=== Drawing {} owned components ===", self.components.len());
        for component in &self.components {
            component.draw();
        }
    }
}

/// SelectBox 组件 - 选择框
pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Drawing select box: {}x{} with {} options",
            self.width,
            self.height,
            self.options.len()
        );
    }
}

// ============= 测试代码 =============

fn main() {
    println!("=== Trait 对象示例 ===\n");

    // 创建不同类型的组件
    let button = Button {
        width: 50,
        height: 10,
        label: String::from("Click"),
    };

    let text_field = TextField {
        width: 100,
        height: 20,
        placeholder: String::from("Enter text..."),
    };

    let select_box = SelectBox {
        width: 80,
        height: 30,
        options: vec!["Option 1".to_string(), "Option 2".to_string()],
    };

    // 使用 &dyn Draw trait 对象
    println!("--- 使用引用 (&dyn Draw) ---");
    let components: Vec<&dyn Draw> = vec![&button, &text_field, &select_box];
    run_drawables(&components);

    // 使用 Box<dyn Draw> trait 对象
    println!("\n--- 使用拥有所有权 (Box<dyn Draw>) ---");
    let mut list = ComponentList::new();
    list.add(Box::new(Button {
        width: 60,
        height: 15,
        label: String::from("Submit"),
    }));
    list.add(Box::new(TextField {
        width: 120,
        height: 25,
        placeholder: String::from("Email"),
    }));
    list.add(Box::new(SelectBox {
        width: 100,
        height: 40,
        options: vec!["A".to_string(), "B".to_string(), "C".to_string()],
    }));
    list.draw_all();

    // 对比泛型和 trait 对象
    println!("\n=== 泛型 vs Trait 对象 ===");
    compare_generic_and_trait_object();
}

/// 对比泛型（静态分发）和 trait 对象（动态分发）
fn compare_generic_and_trait_object() {
    // 泛型版本 - 静态分发
    pub fn notify_draw_generic<T: Draw>(item: &T) {
        item.draw();
    }

    // trait 对象版本 - 动态分发
    pub fn notify_draw_object(item: &dyn Draw) {
        item.draw();
    }

    let button = Button {
        width: 50,
        height: 10,
        label: String::from("Test"),
    };

    println!("泛型版本:");
    notify_draw_generic(&button);

    println!("Trait 对象版本:");
    notify_draw_object(&button);

    // 泛型可以为每个类型生成专用代码（更快）
    // trait 对象使用运行时查找（更灵活）
}

// ============= 扩展知识 =============

/*
动态分发 vs 静态分发:

泛型（静态分发）:
- 编译时确定具体类型
- 为每个类型生成独立代码
- 内联优化
- 更快的执行速度
- 二进制大小增加

Trait 对象（动态分发）:
- 运行时查找方法
- 单一代码路径
- 无法内联
- 略慢的执行速度
- 更小的二进制大小

何时使用 Trait 对象:

✅ 使用 trait 对象:
1. 需要在集合中存储不同类型
2. 类型在编译时未知
3. 需要运行时多态
4. 库的用户定义类型

❌ 使用泛型:
1. 类型在编译时已知
2. 性能很重要
3. 需要内联优化

对象安全规则:

要创建 trait 对象，trait 必须:
1. 不有泛型类型参数
2. 不返回 Self
3. 没有 Self: Sized 约束

✅ 对象安全:
fn draw(&self)
fn as_any(&self) -> &dyn Any

❌ 非对象安全:
fn clone(&self) -> Self  // 返回 Self
fn new() -> Self  // 返回 Self
fn compare<T>(&self, other: &T)  // 泛型

常见 trait 对象:
- dyn Display
- dyn Debug
- dyn Any
- dyn Error
- dyn Iterator (需要 Box 才能对象安全)
*/
