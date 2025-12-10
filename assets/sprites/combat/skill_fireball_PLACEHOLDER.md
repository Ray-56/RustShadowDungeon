# Fireball Sprite Placeholder

## T090: Fireball sprite asset

**Path**: `assets/sprites/combat/skill_fireball.png`

**Specifications**:
- Size: 16×16 pixels (1 tile)
- Format: PNG with transparency
- Animation: Flight (1 frame) + Explosion (3-4 frames)
- Color: Orange/red fireball with yellow core

## Placeholder Instructions

For development, create a simple 16×16 orange circle:

```bash
# Using ImageMagick (if available)
convert -size 16x16 xc:orange -draw "circle 8,8 8,2" assets/sprites/combat/skill_fireball.png

# Or use any pixel art editor (Aseprite, Piskel, etc.)
```

## Art Style Guidelines
- 16×16 基础网格
- 固定调色板（橙色/红色/黄色）
- 无旋转、缩放（仅平移）
- DNF 风格的火球特效

## Future Enhancements
- Multi-frame animation for explosion
- Particle trail during flight
- Glow effect














