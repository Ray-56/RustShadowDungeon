# Combat Sprite Assets Placeholder

本目录将包含战斗系统所需的精灵资产。

## Required Sprites (16×16 pixel grid aligned)

### Hit Effect Particles (8×8 pixels)
- `hit_effect.png` - 打击特效粒子（白色/橙色/金色火花）
  - 用于轻击、重击、暴击的粒子特效
  - 8×8 像素，透明背景
  - 可以是简单的火花/星形图案

## Creating Placeholder Sprite

For development, create a simple 8×8 white square:

```bash
# Using ImageMagick (if available)
convert -size 8x8 xc:white assets/sprites/combat/hit_effect.png

# Or use any pixel art editor (Aseprite, Piskel, etc.)
```

## Art Style Guidelines
- 8×8 基础网格（粒子特效）
- 固定调色板
- 无旋转、缩放（仅平移）
- DNF 风格的打击特效

