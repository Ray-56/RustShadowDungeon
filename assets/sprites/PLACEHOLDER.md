# Sprite Assets Placeholder

本目录将包含玩家移动系统所需的精灵资产。

## Required Sprites (16×16 pixel grid aligned)

### Player Sprites (32×32 pixels)
- `player_idle.png` - 待机动画帧
- `player_walk.png` - 行走动画帧  
- `player_jump.png` - 跳跃动画帧
- `player_fall.png` - 坠落动画帧

### Temporary Development Assets
- `player_placeholder.png` - 开发期间使用的占位符精灵（纯色方块）

## Sprite Requirements (Constitution v1.0.1 Compliance)
- 所有精灵必须对齐到 16×16 像素网格
- 使用 Nearest Neighbor 采样（无线性插值）
- PNG 格式，透明背景
- 像素完美渲染（无亚像素模糊）

## Creating Placeholder Sprite

For development, create a simple 32×32 red square:

```bash
# Using ImageMagick (if available)
convert -size 32x32 xc:red assets/sprites/player_placeholder.png

# Or use any pixel art editor (Aseprite, Piskel, etc.)
```

## Art Style Guidelines
- 16×16 基础网格
- 固定调色板
- 无旋转、缩放（仅平移和翻转）
- DNF 风格的动作游戏美术


