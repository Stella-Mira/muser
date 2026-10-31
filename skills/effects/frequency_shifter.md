<!-- 由 scripts/gen_skills.py 生成；参数表来自代码描述表，请改代码或脚本后重新生成 -->

# 效果器：frequency_shifter

移频：把所有频率平移固定的赫兹数（不同于变调，泛音关系被打破）。

**放在哪里**：小移频（1-10 Hz）做缓慢的相位旋转、加宽；大移频做不和谐的特效；加反馈做螺旋上升/下降的音。

查看最新参数：`muser effects frequency_shifter`（MCP：`describe_effect`）

## 参数

| 参数 | 默认 | 范围 | 说明 |
|---|---|---|---|
| `shift_hz` | 10 | -2000 – 2000 Hz | 平移量。±1-10 细微旋转；±50 以上明显失谐 |
| `feedback` | 0 | 0 – 0.95 | 反馈。越大越像无尽的上升/下降螺旋（巴伯音） |
| `mix` | 1 | 0 – 1 | 干湿比。0.5 时小移频有相位器般的效果 |

所有参数都可省略，省略时取默认值；超出范围时 `muser validate` 会报出具体位置。

## 最小示例

```json
{ "type": "frequency_shifter", "shift_hz": 4, "mix": 0.5 }
```

## 场景预设

- 细微旋转、加宽（类似相位器）

  ```json
  {"type": "frequency_shifter", "shift_hz": 3, "mix": 0.5}
  ```

- 不和谐的失谐特效

  ```json
  {"type": "frequency_shifter", "shift_hz": 120, "mix": 0.7}
  ```

- 无尽上升的螺旋（巴伯音）

  ```json
  {"type": "frequency_shifter", "shift_hz": 8, "feedback": 0.8, "mix": 0.6}
  ```

## 经验与常见错误

- 和变调不同：移频后泛音不再是整数倍，所以大移频总是不和谐的。
- 小移频（1-10 Hz）+ mix 0.5 最常用：慢慢旋转、不跑调。
- feedback 很高时会长时间堆积，记得给尾音留时间（渲染的 `--tail`）。
