<!-- 由 scripts/gen_skills.py 生成；参数表来自代码描述表，请改代码或脚本后重新生成 -->

# 效果器：ring_mod

环形调制：和一个正弦相乘，产生不和谐的金属、钟、机器人音色。

**放在哪里**：适合合成器、鼓的特效段落、科幻音效。频率低（<30 Hz）时是颤动，高时是金属感。

查看最新参数：`muser effects ring_mod`（MCP：`describe_effect`）

## 参数

| 参数 | 默认 | 范围 | 说明 |
|---|---|---|---|
| `frequency_hz` | 440 | 0 – 5000 Hz | 载波频率。20-30 颤动；100-1000 金属/钟声 |
| `mix` | 0.5 | 0 – 1 | 干湿比。1 只剩调制后的声音 |

所有参数都可省略，省略时取默认值；超出范围时 `muser validate` 会报出具体位置。

## 最小示例

```json
{ "type": "ring_mod", "frequency_hz": 300, "mix": 0.4 }
```

## 场景预设

- 钟声 / 金属感

  ```json
  {"type": "ring_mod", "frequency_hz": 600, "mix": 0.5}
  ```

- 机器人

  ```json
  {"type": "ring_mod", "frequency_hz": 90, "mix": 0.8}
  ```

- 慢速颤动（低频载波）

  ```json
  {"type": "ring_mod", "frequency_hz": 18, "mix": 0.6}
  ```

## 经验与常见错误

- 结果和音高无关、几乎总是不和谐的：用在特效、过门、科幻音效；旋律轨慎用。
- frequency_hz 自动化扫动会产生很夸张的金属滑音。
