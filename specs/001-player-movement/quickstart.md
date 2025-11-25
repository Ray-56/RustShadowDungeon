# 快速入门指南：玩家移动系统开发

**Created**: 2025-11-24
**Feature**: 001-player-movement
**Purpose**: 帮助开发者快速设置环境、运行测试、开始开发

---

## 1. 环境准备

### 1.1 必需工具

确保已安装以下工具：

```bash
# Rust 工具链（1.91.1 或更高）
rustc --version   # 应显示 rustc 1.91.1 或更高
cargo --version

# 如果未安装，使用 rustup 安装
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
```

### 1.2 IDE 推荐

- **VSCode** + rust-analyzer 扩展（推荐）
- **RustRover**（JetBrains）
- **Vim/Neovim** + rust.vim + coc-rust-analyzer

### 1.3 可选工具

```bash
# 代码格式化检查
cargo install rustfmt

# 代码检查（linter）
cargo install clippy

# 测试覆盖率
cargo install cargo-tarpaulin

# 性能分析
cargo install flamegraph
cargo install cargo-flamegraph
```

---

## 2. 项目设置

### 2.1 克隆仓库

```bash
git clone https://github.com/Ray-56/RustShadowDungeon.git
cd RustShadowDungeon

# 切换到玩家移动功能分支
git checkout 001-player-movement
```

### 2.2 初始化项目（如果是新项目）

```bash
# 创建 Cargo 项目
cargo new --lib rust_shadow_dungeon
cd rust_shadow_dungeon

# 编辑 Cargo.toml，添加依赖
```

### 2.3 配置 Cargo.toml

在 `Cargo.toml` 中添加以下依赖：

```toml
[package]
name = "rust_shadow_dungeon"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy = { version = "0.17.0", features = ["dynamic_linking"] }  # 开发时启用动态链接加速编译
bevy_rapier2d = "0.29"
bevy_tnua = { version = "0.19", features = ["bevy_rapier2d"] }
leafwing-input-manager = "0.15"
serde = { version = "1.0", features = ["derive"] }
ron = "0.8"

[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "movement_bench"
harness = false

[profile.dev]
opt-level = 1  # 开发时启用最小优化，提升性能

[profile.dev.package."*"]
opt-level = 3  # 依赖项完全优化

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

### 2.4 创建项目结构

```bash
# 创建目录结构（参考 plan.md）
mkdir -p src/{domain/movement,infrastructure/{plugins,components,systems,resources,events}}
mkdir -p tests/{unit,integration}
mkdir -p assets/{sprites,data,levels}
mkdir -p benches

# 创建模块文件
touch src/domain/mod.rs
touch src/domain/movement/mod.rs
touch src/infrastructure/mod.rs
touch src/infrastructure/plugins/mod.rs
# ... 其他模块文件
```

---

## 3. 运行与测试

### 3.1 编译项目

```bash
# 开发构建（快速，带 dynamic_linking）
cargo build

# 发布构建（优化，无 dynamic_linking）
cargo build --release
```

**预期**: 首次编译需要 5-10 分钟（下载并编译所有依赖）。

### 3.2 运行游戏

```bash
# 开发模式运行
cargo run

# 发布模式运行（性能测试）
cargo run --release

# 启用日志输出
RUST_LOG=debug cargo run
RUST_LOG=info,wgpu=warn cargo run  # 过滤 wgpu 日志
```

**预期**: 窗口打开，显示玩家角色，可使用 WASD 移动。

### 3.3 运行测试

```bash
# 运行所有测试（单元测试 + 集成测试）
cargo test

# 运行单元测试
cargo test --lib

# 运行集成测试
cargo test --test movement_pipeline_test

# 运行特定测试
cargo test velocity_calculation

# 显示测试输出
cargo test -- --nocapture

# 测试覆盖率（需要 cargo-tarpaulin）
cargo tarpaulin --out Html
# 在 tarpaulin-report.html 查看覆盖率报告
```

**预期**: 所有测试通过，覆盖率 ≥85%。

### 3.4 运行基准测试

```bash
# 运行性能基准测试
cargo bench --bench movement_bench

# 查看基准测试报告
open target/criterion/report/index.html  # macOS
xdg-open target/criterion/report/index.html  # Linux
start target/criterion/report/index.html  # Windows
```

**预期**: 移动系统平均帧时间 <1ms。

### 3.5 代码检查与格式化

```bash
# 代码格式化
cargo fmt

# 格式化检查（不修改文件）
cargo fmt --check

# Clippy 检查（linter）
cargo clippy

# Clippy 严格模式（零警告）
cargo clippy -- -D warnings
```

**预期**: 零警告，代码格式符合 Rust 标准。

---

## 4. 开发工作流

### 4.1 TDD 工作流（测试驱动开发）

1. **编写失败的测试**:
   ```bash
   # 在 tests/unit/velocity_test.rs 中编写测试
   cargo test velocity_calculation
   # 预期：测试失败（红）
   ```

2. **实现功能**:
   ```bash
   # 在 src/domain/movement/velocity.rs 中实现
   cargo test velocity_calculation
   # 预期：测试通过（绿）
   ```

3. **重构**:
   ```bash
   # 优化代码，保持测试通过
   cargo test
   cargo clippy
   ```

### 4.2 调试技巧

#### 使用 Bevy Inspector

添加依赖（开发时）：

```toml
[dependencies]
bevy-inspector-egui = "0.25"
```

在 main.rs 中：

```rust
use bevy_inspector_egui::quick::WorldInspectorPlugin;

app.add_plugins(WorldInspectorPlugin::new());
```

运行后按 `F1` 打开 Inspector，查看所有实体和组件。

#### 日志调试

```rust
use bevy::log::*;

fn movement_system(...) {
    info!("Player velocity: {:?}", velocity);
    debug!("State transition: {:?} -> {:?}", from, to);
}
```

运行时启用日志：

```bash
RUST_LOG=debug cargo run
```

#### 性能分析（Flamegraph）

```bash
# Linux/macOS
cargo flamegraph --bin rust_shadow_dungeon

# 打开 flamegraph.svg 查看性能热点
```

---

## 5. 常见任务

### 5.1 添加新组件

1. 在 `src/infrastructure/components/player.rs` 中定义：
   ```rust
   #[derive(Component, Debug, Clone)]
   pub struct MyNewComponent {
       pub field: f32,
   }
   ```

2. 在玩家实体蓝图中添加：
   ```rust
   commands.spawn((
       Player,
       MyNewComponent { field: 1.0 },
       // ... 其他组件
   ));
   ```

### 5.2 添加新系统

1. 在 `src/infrastructure/systems/my_system.rs` 中定义：
   ```rust
   pub fn my_system(query: Query<&MyNewComponent>) {
       for component in query.iter() {
           // 系统逻辑
       }
   }
   ```

2. 在 PlayerPlugin 中注册：
   ```rust
   impl Plugin for PlayerPlugin {
       fn build(&self, app: &mut App) {
           app.add_systems(Update, my_system);
       }
   }
   ```

### 5.3 修改移动参数

编辑 `assets/data/movement_config.ron`：

```ron
(
    ground_speed: 60.0,  // 修改地面速度
    air_speed_factor: 0.7,
    // ...
)
```

重新运行游戏，参数立即生效（热重载）。

### 5.4 添加新输入映射

编辑 `assets/data/input_config.ron`：

```ron
(
    keyboard: (
        move_left: "A",
        move_right: "D",
        jump: "Space",
        dash: "Shift",  // 新增冲刺键
    ),
    // ...
)
```

---

## 6. 故障排除

### 问题 1: 编译错误 "could not find `bevy`"

**原因**: 依赖未正确安装。

**解决**:
```bash
cargo clean
cargo update
cargo build
```

### 问题 2: 游戏窗口黑屏，无内容

**原因**: 相机未正确设置或玩家实体未生成。

**解决**:
1. 检查是否在 setup 系统中生成了相机和玩家
2. 启用日志查看错误信息：`RUST_LOG=debug cargo run`
3. 使用 Bevy Inspector 查看实体列表

### 问题 3: 测试失败 "thread panicked"

**原因**: 测试中的断言失败或代码 panic。

**解决**:
```bash
# 显示详细错误信息
cargo test -- --nocapture

# 运行单个测试
cargo test 失败的测试名称 -- --nocapture
```

### 问题 4: 性能不达标（帧率 <60 FPS）

**原因**: 可能是开发构建性能较低。

**解决**:
1. 使用发布构建：`cargo run --release`
2. 启用性能分析：`cargo flamegraph`
3. 检查是否有无限循环或过度分配

### 问题 5: 像素艺术模糊

**原因**: 相机未锁定到像素网格或精灵采样设置错误。

**解决**:
1. 确认 PixelSnap 系统已注册
2. 检查精灵纹理采样模式为 `Nearest`
3. 验证相机缩放级别为整数（1×, 2×, 3×）

---

## 7. 有用的命令速查

```bash
# 开发常用
cargo run                       # 运行游戏（开发模式）
cargo test                      # 运行所有测试
cargo fmt && cargo clippy       # 格式化 + 检查
cargo bench                     # 运行基准测试

# 调试
RUST_LOG=debug cargo run        # 启用调试日志
cargo run --features bevy/trace # 启用 Bevy 跟踪

# 优化
cargo build --release           # 发布构建
cargo flamegraph                # 性能分析

# 清理
cargo clean                     # 清理构建缓存
rm -rf target/                  # 完全删除 target 目录

# 依赖管理
cargo update                    # 更新依赖
cargo tree                      # 查看依赖树
cargo outdated                  # 检查过期依赖（需安装 cargo-outdated）
```

---

## 8. 推荐学习资源

### Bevy 官方文档
- **Bevy Book**: https://bevyengine.org/learn/book/introduction/
- **Bevy Examples**: https://bevyengine.org/examples/
- **Bevy Cheat Book**: https://bevy-cheatbook.github.io/

### 物理引擎文档
- **bevy_rapier2d**: https://rapier.rs/docs/user_guides/bevy_plugin/getting_started_bevy
- **bevy_tnua**: https://github.com/idanarye/bevy-tnua

### Rust 学习
- **The Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/

---

## 9. 获取帮助

### 社区支持
- **Bevy Discord**: https://discord.gg/bevy
- **Rust Discord**: https://discord.gg/rust-lang
- **项目 GitHub Issues**: https://github.com/Ray-56/RustShadowDungeon/issues

### 报告 Bug
在 GitHub Issues 中报告 bug 时，请提供：
1. 完整的错误信息或日志
2. 复现步骤
3. 系统信息（操作系统、Rust 版本）
4. 预期行为 vs 实际行为

---

**Document Status**: ✅ 快速入门指南已完成
**Ready for**: 开发者可立即开始开发玩家移动系统



