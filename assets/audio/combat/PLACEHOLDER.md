# Combat Audio Assets Placeholder

本目录将包含战斗系统所需的音效资产。

## Required Audio Files

### Hit Sound Effects
- `hit_light.ogg` - 轻击音效
  - 用于普通攻击命中
  - 格式：OGG Vorbis
  - 采样率：44.1 kHz 或 22.05 kHz
  - 时长：<1 秒
  - 音量：适中

- `hit_heavy.ogg` - 重击音效
  - 用于第三击（重击）命中
  - 格式：OGG Vorbis
  - 采样率：44.1 kHz 或 22.05 kHz
  - 时长：<1 秒
  - 音量：较大，更有冲击感

- `hit_critical.ogg` - 暴击音效
  - 用于暴击命中
  - 格式：OGG Vorbis
  - 采样率：44.1 kHz 或 22.05 kHz
  - 时长：<1 秒
  - 音量：最大，最强烈的冲击感

## Audio Requirements (Constitution v1.0.1 Compliance)
- 所有音效必须对齐到 16×16 像素网格（视觉同步）
- 使用 OGG Vorbis 格式（压缩率高，加载快）
- 16-bit 深度
- 最大时长 <2 秒
- 音效与动画同步（误差 <2 帧）

## Creating Placeholder Audio

For development, create silent OGG files:

```bash
# Using ffmpeg (if available)
ffmpeg -f lavfi -i anullsrc=r=44100:cl=mono -t 0.1 -c:a libvorbis assets/audio/combat/hit_light.ogg
ffmpeg -f lavfi -i anullsrc=r=44100:cl=mono -t 0.1 -c:a libvorbis assets/audio/combat/hit_heavy.ogg
ffmpeg -f lavfi -i anullsrc=r=44100:cl=mono -t 0.1 -c:a libvorbis assets/audio/combat/hit_critical.ogg
```

## Art Style Guidelines
- DNF 风格的打击音效
- 清晰、有冲击力
- 与视觉反馈同步

