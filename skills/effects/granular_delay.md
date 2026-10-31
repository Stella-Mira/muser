<!-- 由 scripts/gen_skills.py 生成；参数表来自代码描述表，请改代码或脚本后重新生成 -->

# 效果器：granular_delay

颗粒延迟：把延迟线切成小颗粒随机重放，能变调、散开，做出云雾、闪烁的氛围。

**放在哪里**：适合 pad、钢琴、人声的氛围段，常放在发送总线上。pitch_semitones 12 做"闪光"（shimmer）。

查看最新参数：`muser effects granular_delay`（MCP：`describe_effect`）

## 参数

| 参数 | 默认 | 范围 | 说明 |
|---|---|---|---|
| `position_ms` | 250 | 0 – 1000 ms | 颗粒从多久之前取 |
| `spray_ms` | 50 | 0 – 500 ms | 取样位置的随机范围 |
| `grain_ms` | 60 | 5 – 500 ms | 颗粒长度。短（10-30）颗粒感强；长（100+）更平滑 |
| `density_hz` | 30 | 0.5 – 200 Hz | 每秒多少颗粒 |
| `pitch_semitones` | 0 | -24 – 24 半音 | 颗粒变调。12 高八度闪光；-12 低沉 |
| `pitch_spray` | 0 | 0 – 24 半音 | 变调的随机范围 |
| `stereo_spread` | 0.5 | 0 – 1 | 颗粒在左右的散开程度 |
| `feedback` | 0 | 0 – 0.95 | 反馈。越大云雾越厚越长 |
| `mix` | 0.5 | 0 – 1 | 干湿比。在发送总线上用 1 |

所有参数都可省略，省略时取默认值；超出范围时 `muser validate` 会报出具体位置。

## 最小示例

```json
{ "type": "granular_delay", "pitch_semitones": 12, "feedback": 0.4, "mix": 0.4 }
```

## 场景预设

- 闪光 shimmer（高八度的云）

  ```json
  {"type": "granular_delay", "pitch_semitones": 12, "feedback": 0.5, "mix": 0.4}
  ```

- 颗粒氛围（钢琴、pad 的发送总线）

  ```json
  {"type": "granular_delay", "grain_ms": 120, "spray_ms": 200, "density_hz": 20, "feedback": 0.4, "mix": 1.0}
  ```

- 故障感的碎片

  ```json
  {"type": "granular_delay", "grain_ms": 15, "density_hz": 80, "pitch_spray": 7, "mix": 0.5}
  ```

## 经验与常见错误

- 最常用在发送总线上（mix 1），再跟一个 reverb：氛围、电影感。
- pitch_semitones 12 + 一点 feedback 是 shimmer 混响的核心声音。
- 颗粒的随机是固定种子，同一份工程每次渲染结果一样。
