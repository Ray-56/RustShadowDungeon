# Phase 0 研究：玩家移动系统技术决策

**Created**: 2025-11-24
**Feature**: 001-player-movement
**Purpose**: 解决技术实施中的未知问题，为 Phase 1 设计提供依据

---

## 研究概述

本文档记录玩家移动系统的技术研究结果，涵盖 bevy_tnua 集成、像素完美渲染、输入系统设计和性能基准测试方案。所有决策均基于对备选方案的充分评估，并符合 Constitution v1.0.1 原则。

---

## 研究 1: bevy_tnua 集成可行性

### Decision（决策）

**采用 bevy_tnua 作为字符控制器**，配合 **Avian2d** 使用（而非原计划的 bevy_rapier2d）。

**重要技术变更** (2025-11-24):
- ✅ 物理引擎从 bevy_rapier2d 变更为 **Avian2d 0.4**
- ✅ 使用 bevy-tnua-avian2d 0.8 作为集成桥梁
- ✅ bevy-tnua 版本升级到 0.26（支持 Bevy 0.17）

### Rationale（理由）

1. **专为字符控制设计**: bevy_tnua 提供高级字符控制器 API，封装了地面检测、斜坡行走、阶梯处理等常见功能，减少手写代码量。

2. **Avian2d 集成优势**: 
   - bevy-tnua-avian2d 0.8 官方支持
   - 完美兼容 Bevy 0.17.0（bevy_rapier2d 有版本冲突）
   - MIT/Apache-2.0 双重许可证（Constitution 合规）

3. **2D 侧视图支持**: bevy_tnua 设计支持 2D 物理，只需配置 2D 物理后端（Avian2d 而非 bevy_rapier2d）。

4. **社区验证**: GitHub 上有多个使用 bevy_tnua 开发 2D 平台游戏的示例项目，Avian2d 是 bevy_xpbd 的官方继任者，社区活跃。

5. **技术稳定性**: Avian2d 使用 XPBD 算法，比传统 impulse-based 物理更稳定，更适合 60 FPS 游戏。

### Alternatives Considered（考虑的备选方案）

| 方案 | 优点 | 缺点 | 拒绝原因 |
|------|------|------|---------|
| **bevy_rapier2d + bevy-tnua** | 原计划方案，文档丰富 | Bevy 0.17 版本冲突，无法编译 | 技术不可行，已验证失败 |
| **手写字符控制器** | 完全控制、无依赖 | 需实现地面检测、斜坡行走等复杂逻辑，开发时间长 | 与项目计划 M1 的 2 周工期不符，优先使用成熟库 |
| **bevy_platformer** | 专为 2D 平台游戏设计 | 文档较少、社区活跃度低、最后更新时间较旧 | 维护风险高，bevy_tnua 社区更活跃 |
| **直接使用 Avian2d** | 底层控制、灵活性高 | 需手动实现所有字符控制逻辑（速度控制、跳跃、地面检测） | 重复造轮子，bevy_tnua 已提供良好抽象 |

### Implementation Notes（实施注意事项）

1. **配置 2D 物理后端**:
   ```rust
   // Cargo.toml
   bevy-tnua = "0.26"
   avian2d = { version = "0.4", features = ["debug-plugin", "simd", "parallel"] }
   bevy-tnua-avian2d = "0.8"
   ```

2. **地面检测配置**:
   - 设置地面检测射线长度为 2 像素（角色底部稍微下方）
   - 配置地面层（CollisionGroups）以避免与其他物体误判

3. **跳跃参数调优**:
   - 初始跳跃速度：设置为达到 32 像素高度（2 个格子）
   - 重力：调整为符合 0.4 秒跳跃持续时间
   - 终端速度：限制坠落速度为 128 像素/秒

4. **回退计划**:
   如果 bevy_tnua + Avian2d 在实施过程中出现无法解决的问题（如性能不达标、与像素完美渲染冲突），将回退到手写字符控制器。预计回退增加 3 天开发时间（已在项目计划风险部分识别）。
   
   **技术决策变更记录**: 2025-11-24 成功从 bevy_rapier2d 迁移到 Avian2d，编译验证通过，无回退需求。详见 `.specify/specs/00-project-overview/tech-decisions/physics-engine-research.md`。

---

## 研究 2: 像素完美渲染方案

### Decision（决策）

**采用相机锁定到像素网格 + 整数缩放方案**，使用 Bevy 的 OrthographicProjection 配置像素完美渲染。

### Rationale（理由）

1. **Bevy 官方支持**: Bevy 提供 `OrthographicProjection` 和 `Transform`，可轻松实现像素完美渲染。

2. **简单有效**: 通过将相机和实体位置取整到像素网格，避免亚像素渲染导致的模糊。

3. **性能友好**: 整数位置计算不增加性能开销，仍可使用浮点数进行物理计算，仅在渲染时取整。

4. **社区最佳实践**: Bevy 社区广泛使用此方案，有大量成功案例（如 Kataster、bevy_pixel_camera 插件）。

### Alternatives Considered（考虑的备选方案）

| 方案 | 优点 | 缺点 | 拒绝原因 |
|------|------|------|---------|
| **bevy_pixel_camera 插件** | 开箱即用、配置简单 | 增加额外依赖、可能与 bevy_tnua 冲突 | 功能简单，可自行实现，避免依赖膨胀 |
| **固定像素分辨率渲染** | 渲染到固定尺寸纹理，缩放到窗口 | 实现复杂、性能开销（额外渲染通道） | 不符合"简单有效"原则，过度工程 |
| **子像素渲染（不对齐）** | 移动更平滑 | 违反 Constitution 原则 IV（像素完美），精灵模糊 | 明确违反宪法要求，不可接受 |

### Implementation Notes（实施注意事项）

1. **相机配置**:
   ```rust
   // 配置正交相机，像素与屏幕像素 1:1 对应
   commands.spawn(Camera2dBundle {
       projection: OrthographicProjection {
           scaling_mode: ScalingMode::WindowSize(1.0),
           ..default()
       },
       ..default()
   });
   ```

2. **实体位置取整**:
   ```rust
   // 在渲染前将 Transform 位置取整
   fn pixel_snap_system(mut query: Query<&mut Transform, With<PixelSnap>>) {
       for mut transform in query.iter_mut() {
           transform.translation.x = transform.translation.x.round();
           transform.translation.y = transform.translation.y.round();
       }
   }
   ```

3. **整数缩放**:
   - 窗口尺寸应为 16 的倍数（如 1280×720 = 80×45 格子）
   - 相机缩放级别使用整数（1×, 2×, 3×）

4. **精灵配置**:
   - 所有精灵使用 `Nearest` 采样（不使用线性插值）
   - 精灵尺寸为 16 的倍数（16×16, 32×32, 48×48）

5. **测试验证**:
   - 使用放大镜检查精灵边缘是否清晰
   - 在不同窗口尺寸下测试，确保无模糊

---

## 研究 3: 输入系统设计

### Decision（决策）

**采用 leafwing-input-manager 统一键盘和手柄输入**，自行实现 150ms 输入缓冲机制。

### Rationale（理由）

1. **输入抽象层**: leafwing-input-manager 提供高级输入抽象，将键盘、手柄、触摸统一为"动作"（Action），简化输入处理。

2. **跨平台一致性**: 自动处理不同平台的输入差异（如 Windows 的 XInput、Linux 的 evdev），确保跨平台一致体验。

3. **输入组合支持**: 支持多键组合（如 Ctrl+A）、输入序列，为后续复杂操作（如技能组合键）准备。

4. **社区推荐**: Bevy 社区广泛推荐的输入库，文档完善，示例丰富。

5. **MIT 许可证**: 符合项目许可证要求。

### Alternatives Considered（考虑的备选方案）

| 方案 | 优点 | 缺点 | 拒绝原因 |
|------|------|------|---------|
| **Bevy 内置输入（Input<KeyCode>）** | 无额外依赖 | 需手动处理键盘+手柄映射、输入缓冲 | 代码冗余，leafwing-input-manager 提供更好抽象 |
| **gilrs 库（直接手柄支持）** | 底层控制 | 仅支持手柄，需额外处理键盘输入 | 不统一，需维护两套输入代码 |
| **自行实现输入管理器** | 完全控制 | 开发时间长、需处理跨平台差异 | 重复造轮子，与项目 2 周工期不符 |

### Implementation Notes（实施注意事项）

1. **动作定义**:
   ```rust
   #[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash)]
   enum PlayerAction {
       MoveLeft,
       MoveRight,
       Jump,
   }
   ```

2. **输入映射配置**:
   ```rust
   // 键盘映射
   input_map.insert(KeyCode::A, PlayerAction::MoveLeft);
   input_map.insert(KeyCode::D, PlayerAction::MoveRight);
   input_map.insert(KeyCode::Space, PlayerAction::Jump);
   
   // 手柄映射
   input_map.insert(GamepadButtonType::DPadLeft, PlayerAction::MoveLeft);
   input_map.insert(GamepadButtonType::DPadRight, PlayerAction::MoveRight);
   input_map.insert(GamepadButtonType::South, PlayerAction::Jump); // A/Cross
   ```

3. **输入缓冲实现**:
   - 创建 InputBuffer 资源，存储最近 150ms 的输入
   - 在移动系统中检查缓冲，即使当前帧无跳跃输入，如果缓冲中有，也执行跳跃
   - 清理过期输入（超过 150ms）

4. **手柄摇杆灵敏度**:
   - 设置死区（Dead Zone）为 0.1（避免摇杆漂移）
   - 摇杆推动比例映射到移动速度（0-100%）

---

## 研究 4: 性能基准测试方案

### Decision（决策）

**使用 criterion 编写性能基准测试**，模拟典型游戏场景，验证移动系统帧时间 <1ms。

### Rationale（理由）

1. **Rust 标准**: criterion 是 Rust 生态最流行的基准测试库，精确度高、统计分析完善。

2. **回归检测**: criterion 自动检测性能回归，可设置阈值（>10% 回归触发警告），符合 Constitution 原则 III。

3. **可视化报告**: 生成 HTML 报告，直观展示性能趋势。

4. **CI 集成**: 可集成到 CI 流程，自动运行基准测试。

### Alternatives Considered（考虑的备选方案）

| 方案 | 优点 | 缺点 | 拒绝原因 |
|------|------|------|---------|
| **手动计时（std::time::Instant）** | 简单直接 | 无统计分析、易受系统噪声影响 | 不准确，无法检测小幅度回归 |
| **bevy_diagnostic（帧时间监控）** | 集成在游戏中 | 仅显示实时数据，无历史对比、无自动化 | 适合开发调试，不适合自动化测试 |
| **cargo-flamegraph（性能分析）** | 详细的性能瓶颈分析 | 开销大、不适合快速回归测试 | 用于深度优化，不替代基准测试 |

### Implementation Notes（实施注意事项）

1. **基准测试场景**:
   ```rust
   // benches/movement_bench.rs
   fn movement_system_benchmark(c: &mut Criterion) {
       c.bench_function("single_player_movement", |b| {
           let mut app = App::new();
           // 设置典型场景：1 个玩家、输入、物理
           app.add_plugins(MinimalPlugins)
              .add_systems(Update, movement_system);
           
           b.iter(|| {
               app.update(); // 执行一帧
           });
       });
   }
   ```

2. **测试场景**:
   - **Baseline**: 单玩家站立（无输入）
   - **Walking**: 单玩家行走（持续 D 键输入）
   - **Jumping**: 单玩家连续跳跃（150ms 间隔空格输入）
   - **AirControl**: 单玩家空中移动（跳跃+左右输入）

3. **性能目标**:
   - 所有场景 <1ms（平均值）
   - p95（95 百分位）<1.5ms
   - 最大值 <2ms（允许偶尔的峰值）

4. **回归检测**:
   ```rust
   c.bench_function("movement_system")
       .with_measurement(WallTime)
       .sample_size(100) // 100 次采样
       .significance_level(0.1) // 10% 回归触发警告
   ```

5. **CI 集成**:
   ```bash
   # .github/workflows/benchmark.yml
   cargo bench --bench movement_bench
   ```

---

## 研究总结

所有技术决策已完成，主要结论：

| 技术领域 | 最终方案 | 风险级别 | 缓解措施 |
|----------|---------|---------|---------|
| **字符控制器** | bevy_tnua 0.26 + Avian2d 0.4 | 低 | 编译验证通过，官方集成稳定 |
| **物理引擎** | Avian2d 0.4 (XPBD) | 低 | 完美兼容 Bevy 0.17，回退到手写（+3 天） |
| **像素完美** | 相机网格锁定 + 整数缩放 | 低 | Bevy 官方支持，社区验证 |
| **输入系统** | leafwing-input-manager 0.17 + 自定义缓冲 | 低 | 成熟库 + 简单实现 |
| **性能测试** | criterion 基准测试 | 低 | Rust 标准，广泛使用 |

**重要技术变更**:
- ✅ 2025-11-24: 从 bevy_rapier2d 变更为 Avian2d（详见技术调研报告）
- ✅ 原因：bevy_rapier2d 0.29/0.31 与 Bevy 0.17 版本冲突
- ✅ 优势：Avian2d XPBD 算法更稳定，更适合 60 FPS 目标
- ✅ 许可证：MIT/Apache-2.0 双重许可，完全合规

**准备状态**: ✅ 所有技术未知问题已解决，编译验证通过，可进入 Phase 1 设计阶段。

---

**Next Phase**: 生成 `data-model.md`（ECS 组件设计）和 `quickstart.md`（开发者快速入门）

