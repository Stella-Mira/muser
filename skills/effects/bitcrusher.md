<!-- 由 scripts/gen_skills.py 生成；参数表来自代码描述表，请改代码或脚本后重新生成 -->

# 效果器：bitcrusher

降位降采样：变粗、变脏的数码失真（8-bit、老采样器、lo-fi 质感）。

**放在哪里**：适合鼓、lead、整条 lo-fi 总线；bits 8-12 是老采样器，4-6 很脏。放在混响之前。

查看最新参数：`muser effects bitcrusher`（MCP：`describe_effect`）

## 参数

| 参数 | 默认 | 范围 | 说明 |
|---|---|---|---|
| `bits` | 8 | 1 – 24 bit | 位深。12 轻微颗粒；8 老游戏机；4 以下非常脏 |
| `downsample` | 1 | 1 – 64 × | 降采样倍数（保持每 N 个样本）。1 不降；4-8 明显的金属锯齿感 |
| `mix` | 1 | 0 – 1 | 干湿比。0.3-0.5 只加一点颗粒 |

所有参数都可省略，省略时取默认值；超出范围时 `muser validate` 会报出具体位置。

## 最小示例

```json
{ "type": "bitcrusher", "bits": 10, "downsample": 2, "mix": 0.6 }
```

## 场景预设

- 老采样器的颗粒（lo-fi 鼓、键盘）

  ```json
  {"type": "bitcrusher", "bits": 12, "downsample": 2, "mix": 0.5}
  ```

- 8-bit 游戏机

  ```json
  {"type": "bitcrusher", "bits": 8, "downsample": 4, "mix": 1.0}
  ```

- 很脏的破碎感（特效段落）

  ```json
  {"type": "bitcrusher", "bits": 4, "downsample": 8, "mix": 0.7}
  ```

## 经验与常见错误

- 降位会抬高底噪和高频锯齿，整首都开时 mix 0.3-0.5 更耐听；全开适合短段落和过门。
- 放在混响之前：先变脏再进空间；放在混响之后会把尾音也打碎。
- 和 `tape` 一起用是常见的 lo-fi 链：tape → bitcrusher（轻）→ eq lowpass。
