<!-- 由 scripts/gen_skills.py 生成；参数表来自代码描述表，请改代码或脚本后重新生成 -->

# 效果器：multiband_compressor

三段压缩：分成低、中、高三段各自压缩，低频的重音不会压到高频。

**放在哪里**：主总线做母带、鼓总线、贝斯；low_hz / high_hz 是分频点。low_db / mid_db / high_db 是每段的补偿增益，也能当粗调 EQ。放在 limiter 之前。

查看最新参数：`muser effects multiband_compressor`（MCP：`describe_effect`）

## 参数

| 参数 | 默认 | 范围 | 说明 |
|---|---|---|---|
| `low_hz` | 200 | 20 – 2000 Hz | 低频段与中频段的分频点 |
| `high_hz` | 2000 | 200 – 16000 Hz | 中频段与高频段的分频点（要高于 low_hz） |
| `threshold_db` | -20 | -60 – 0 dB | 三段共用的阈值 |
| `ratio` | 3 | 1 – 100 :1 | 三段共用的压缩比。2-3 母带；4+ 更紧 |
| `attack_ms` | 10 | 0.01 – 1000 ms | 启动时间 |
| `release_ms` | 150 | 1 – 5000 ms | 释放时间 |
| `low_db` | 0 | -24 – 24 dB | 低频段补偿增益 |
| `mid_db` | 0 | -24 – 24 dB | 中频段补偿增益 |
| `high_db` | 0 | -24 – 24 dB | 高频段补偿增益 |

所有参数都可省略，省略时取默认值；超出范围时 `muser validate` 会报出具体位置。

## 最小示例

```json
{ "type": "multiband_compressor", "threshold_db": -18, "ratio": 2.5, "high_db": 1 }
```

## 场景预设

- 母带轻压（在 limiter 之前）

  ```json
  {"type": "multiband_compressor", "threshold_db": -18, "ratio": 2, "attack_ms": 20, "release_ms": 200}
  ```

- 鼓总线：压住低频、提亮高频

  ```json
  {"type": "multiband_compressor", "low_hz": 150, "high_hz": 4000, "threshold_db": -24, "ratio": 4, "high_db": 2}
  ```

- 贝斯：只压低频的轰鸣

  ```json
  {"type": "multiband_compressor", "low_hz": 120, "high_hz": 1500, "threshold_db": -22, "ratio": 4}
  ```

## 经验与常见错误

- 比普通压缩器的好处：底鼓一响不会把镲片也压下去。
- low_db / mid_db / high_db 是每段的补偿增益，也可以当一个很宽的三段 EQ 用。
- 主总线上顺序：eq → multiband_compressor → limiter。
