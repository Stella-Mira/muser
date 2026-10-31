<!-- 由 scripts/gen_skills.py 生成；参数表来自代码描述表，请改代码或脚本后重新生成 -->

# 效果器：formant_filter

元音滤波：按人声元音的共振峰滤波，让合成器"说话"（a-e-i-o-u）。

**放在哪里**：适合合成器 lead、贝斯、pad。vowel 用自动化来回移动就是"哇哇"的说话感。

查看最新参数：`muser effects formant_filter`（MCP：`describe_effect`）

## 参数

| 参数 | 默认 | 范围 | 说明 |
|---|---|---|---|
| `vowel` | 0 | 0 – 4 | 元音位置：0 a、1 e、2 i、3 o、4 u，中间平滑过渡 |
| `shift` | 0 | -12 – 12 半音 | 共振峰平移。+ 更小的嗓子；- 更大的嗓子 |
| `gain_db` | 0 | -24 – 24 dB | 输出增益 |
| `mix` | 1 | 0 – 1 | 干湿比 |

所有参数都可省略，省略时取默认值；超出范围时 `muser validate` 会报出具体位置。

## 最小示例

```json
{ "type": "formant_filter", "vowel": 1.5, "mix": 0.8 }
```

## 场景预设

- "哇"——合成器 lead 说话（配合 vowel 自动化）

  ```json
  {"type": "formant_filter", "vowel": 0, "mix": 0.9}
  ```

- 贝斯的 "哦"

  ```json
  {"type": "formant_filter", "vowel": 3, "shift": -3, "mix": 0.7}
  ```

- 细小的嗓子

  ```json
  {"type": "formant_filter", "vowel": 2, "shift": 5, "mix": 0.8}
  ```

## 经验与常见错误

- vowel 用自动化来回移动（0 ↔ 4，lfo 或 points）才是"说话"的感觉；固定一个值只是一种音色。
- 泛音丰富的音色（锯齿、方波）效果最明显；正弦波几乎没变化。
- 和合成器自带的 vowel 滤波不同：它是效果器，可以挂在任何轨（包括插件乐器、音频）上。
