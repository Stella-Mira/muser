<!-- 由 scripts/gen_skills.py 生成；参数表来自代码描述表，请改代码或脚本后重新生成 -->

# 效果器：transient_shaper

瞬态塑形：单独增强/减弱起音和延音，不管音量多大。

**放在哪里**：鼓最常用：attack_db +4 到 +8 让鼓更脆更冲；sustain_db -6 收紧余音（去房间声）。放在压缩器之前。

查看最新参数：`muser effects transient_shaper`（MCP：`describe_effect`）

## 参数

| 参数 | 默认 | 范围 | 说明 |
|---|---|---|---|
| `attack_db` | 0 | -24 – 24 dB | 起音增益。+ 更冲、更脆；- 更软、更远 |
| `sustain_db` | 0 | -24 – 24 dB | 延音增益。+ 更多尾音和房间感；- 更干更紧 |
| `attack_ms` | 20 | 2 – 200 ms | 起音段的长度。鼓 5-20 |
| `sustain_ms` | 400 | 60 – 3000 ms | 延音段的反应时间 |

所有参数都可省略，省略时取默认值；超出范围时 `muser validate` 会报出具体位置。

## 最小示例

```json
{ "type": "transient_shaper", "attack_db": 6, "sustain_db": -4 }
```

## 场景预设

- 鼓更冲、更脆

  ```json
  {"type": "transient_shaper", "attack_db": 6, "sustain_db": 0}
  ```

- 鼓更干更紧（去房间声）

  ```json
  {"type": "transient_shaper", "attack_db": 2, "sustain_db": -8}
  ```

- 鼓更软、更远

  ```json
  {"type": "transient_shaper", "attack_db": -6, "sustain_db": 2}
  ```

## 经验与常见错误

- 和压缩器不同，它只看声音的形状、不看音量：轻敲和重敲得到一样比例的处理。
- 放在压缩器之前：先定形状，再控动态。
- 也能用在原声吉他（拨弦更清楚）、贝斯（attack 让音头更清楚）。
