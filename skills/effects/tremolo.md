<!-- 由 scripts/gen_skills.py 生成；参数表来自代码描述表，请改代码或脚本后重新生成 -->

# 效果器：tremolo

颤音（音量）：按 LFO 周期性地起伏音量。

**放在哪里**：适合电钢、吉他、pad。按节拍：rate_hz = tempo / 60 / 拍数（每拍一次 = tempo/60）。

查看最新参数：`muser effects tremolo`（MCP：`describe_effect`）

## 参数

| 参数 | 默认 | 范围 | 说明 |
|---|---|---|---|
| `shape` | `sine` | `sine` / `triangle` / `square` / `saw_up` / `saw_down` | LFO 波形：sine 柔和；triangle；square 斩波（chopper）效果；saw_up / saw_down 锯齿 |
| `rate_hz` | 5 | 0 – 40 Hz | 起伏速度。3-6 经典电钢颤音；按节拍取值更合拍 |
| `depth` | 0.5 | 0 – 1 | 深度。0.3 轻微；1 起伏到无声 |
| `stereo_phase` | 0 | 0 – 1 | 左右声道的相位差（周）。0.5 左右交替，带一点自动声像感 |

所有参数都可省略，省略时取默认值；超出范围时 `muser validate` 会报出具体位置。

## 最小示例

```json
{ "type": "tremolo", "rate_hz": 4, "depth": 0.6 }
```

## 场景预设

- 电钢经典颤音

  ```json
  {"type": "tremolo", "rate_hz": 4.5, "depth": 0.5}
  ```

- 按八分音符斩波 @120bpm（rate = tempo/30）

  ```json
  {"type": "tremolo", "rate_hz": 4, "depth": 1, "shape": "square"}
  ```

- 左右交替的颤音（加宽）

  ```json
  {"type": "tremolo", "rate_hz": 3, "depth": 0.6, "stereo_phase": 0.5}
  ```

## 经验与常见错误

- rate_hz 按节拍算才合拍：每拍一次 = tempo/60，八分音符 = tempo/30，十六分 = tempo/15。
- `shape: square` + depth 1 是 trance gate / 斩波；sine 更柔和。
- depth 自动化（从 0 推到 0.8）能做出逐渐"颤起来"的效果。
