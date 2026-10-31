<!-- 由 scripts/gen_skills.py 生成；参数表来自代码描述表，请改代码或脚本后重新生成 -->

# 效果器：tape

磁带：磁带饱和的温暖失真，加上走带不稳的音高晃动（wow / flutter）。

**放在哪里**：lo-fi、复古、温暖：整条总线或主总线轻用（drive 3-6），单轨可以更重。wow_cents 5-15 是老卡带的跑调感。

查看最新参数：`muser effects tape`（MCP：`describe_effect`）

## 参数

| 参数 | 默认 | 范围 | 说明 |
|---|---|---|---|
| `drive_db` | 6 | 0 – 30 dB | 驱动。3-6 温暖；12 以上明显失真 |
| `asymmetry` | 0.15 | 0 – 0.5 | 不对称，加偶次谐波（更暖） |
| `wow_cents` | 3 | 0 – 30 cent | 慢速音高晃动的深度。0 关；5-15 老卡带 |
| `flutter_cents` | 1 | 0 – 10 cent | 快速音高抖动的深度 |
| `mix` | 1 | 0 – 1 | 干湿比 |

所有参数都可省略，省略时取默认值；超出范围时 `muser validate` 会报出具体位置。

## 最小示例

```json
{ "type": "tape", "drive_db": 5, "wow_cents": 8 }
```

## 场景预设

- 温暖（主总线、鼓总线）

  ```json
  {"type": "tape", "drive_db": 4, "wow_cents": 0, "flutter_cents": 0}
  ```

- 老卡带 lo-fi（跑调感）

  ```json
  {"type": "tape", "drive_db": 6, "wow_cents": 12, "flutter_cents": 3}
  ```

- 重度饱和

  ```json
  {"type": "tape", "drive_db": 15, "asymmetry": 0.3, "mix": 0.8}
  ```

## 经验与常见错误

- wow / flutter 会让音高轻微晃动：和弦乐器、钢琴最明显；只想要温暖时把它们设成 0。
- lo-fi 链：tape → bitcrusher（轻）→ eq lowpass 8000 → reverb。
- drive 会让声音变响，用 `muser mix-report` 看推子后的响度再调 gain_db。
