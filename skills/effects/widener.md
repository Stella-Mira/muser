<!-- 由 scripts/gen_skills.py 生成；参数表来自代码描述表，请改代码或脚本后重新生成 -->

# 效果器：widener

立体声扩展：加大左右差异，让声音更宽；低频可保持单声道。

**放在哪里**：适合 pad、和弦、混响总线、主总线（轻用）。贝斯、底鼓、主唱保持居中。

查看最新参数：`muser effects widener`（MCP：`describe_effect`）

## 参数

| 参数 | 默认 | 范围 | 说明 |
|---|---|---|---|
| `width` | 1.5 | 0 – 2 | 宽度。1 不变；1.2-1.6 常用；0 变单声道 |
| `bass_mono_hz` | 120 | 0 – 1000 Hz | 低于这个频率保持单声道（低频不散）。0 关闭 |

所有参数都可省略，省略时取默认值；超出范围时 `muser validate` 会报出具体位置。

## 最小示例

```json
{ "type": "widener", "width": 1.4, "bass_mono_hz": 150 }
```

## 场景预设

- pad / 和弦加宽

  ```json
  {"type": "widener", "width": 1.5, "bass_mono_hz": 150}
  ```

- 混响总线加宽

  ```json
  {"type": "widener", "width": 1.3, "bass_mono_hz": 200}
  ```

- 变窄（让一件乐器集中）

  ```json
  {"type": "widener", "width": 0.5, "bass_mono_hz": 0}
  ```

## 经验与常见错误

- 只对立体声的声音有用：单声道的乐器（左右一样）先加 chorus 或 auto_pan 才有"宽"可扩。
- width 超过 1.6 时合成单声道（手机外放）会明显变薄；主总线上不超过 1.2。
- bass_mono_hz 保持低频居中，低频才稳、才有力；0 关闭。
