<!-- 由 scripts/gen_skills.py 生成；参数表来自代码描述表，请改代码或脚本后重新生成 -->

# 效果器：auto_pan

自动声像：按 LFO 让声音在左右之间来回移动。

**放在哪里**：适合 pad、琶音、打击乐点缀；不要用在贝斯和底鼓上。按节拍：rate_hz = tempo / 60 / 拍数。

查看最新参数：`muser effects auto_pan`（MCP：`describe_effect`）

## 参数

| 参数 | 默认 | 范围 | 说明 |
|---|---|---|---|
| `shape` | `sine` | `sine` / `triangle` / `square` / `saw_up` / `saw_down` | LFO 波形：sine 平滑；square 左右跳 |
| `rate_hz` | 0.5 | 0 – 40 Hz | 移动速度。0.1-0.5 缓慢漂移；2 以上快速晃动 |
| `width` | 1 | 0 – 1 | 摆动幅度。1 从最左到最右；0.3 只在中间附近晃 |

所有参数都可省略，省略时取默认值；超出范围时 `muser validate` 会报出具体位置。

## 最小示例

```json
{ "type": "auto_pan", "rate_hz": 0.25, "width": 0.8 }
```

## 场景预设

- 缓慢漂移的 pad

  ```json
  {"type": "auto_pan", "rate_hz": 0.15, "width": 0.7}
  ```

- 按小节左右摆 @120bpm（每 2 拍一次 = tempo/120）

  ```json
  {"type": "auto_pan", "rate_hz": 1, "width": 1}
  ```

- 左右跳的打击乐点缀

  ```json
  {"type": "auto_pan", "rate_hz": 2, "width": 0.8, "shape": "square"}
  ```

## 经验与常见错误

- 贝斯、底鼓、主唱保持居中，不要自动声像。
- 多条轨都要动时，速度错开（0.13、0.21…）比同速更自然。
