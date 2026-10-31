<!-- 由 scripts/gen_skills.py 生成；参数表来自代码描述表，请改代码或脚本后重新生成 -->

# 效果器：vocoder

声码器：用另一条轨（多是人声）的频谱去塑造这条轨（多是合成器和弦）——机器人唱歌。

**放在哪里**：挂在合成器/和弦轨（载波）上，"sidechain" 写人声那条轨或总线的名字（调制信号）。载波要持续、泛音丰富（锯齿 pad、string machine）。

查看最新参数：`muser effects vocoder`（MCP：`describe_effect`）

## 参数

| 参数 | 默认 | 范围 | 说明 |
|---|---|---|---|
| `bands` | 16 | 8 – 32 | 频带数。8 粗糙复古；16 常规；32 更清楚 |
| `low_hz` | 100 | 40 – 16000 Hz | 最低频带 |
| `high_hz` | 8000 | 40 – 16000 Hz | 最高频带 |
| `formant_shift` | 0 | -12 – 12 半音 | 共振峰平移。+ 更尖（小孩、卡通）；- 更粗 |
| `attack_ms` | 10 | 0.1 – 500 ms | 包络跟随的启动时间 |
| `release_ms` | 10 | 0.1 – 2000 ms | 包络跟随的释放时间。长一点更连贯 |
| `gain_db` | 0 | -24 – 24 dB | 输出增益 |
| `mix` | 1 | 0 – 1 | 干湿比 |

所有参数都可省略，省略时取默认值；超出范围时 `muser validate` 会报出具体位置。

## 最小示例

```json
{ "type": "vocoder", "sidechain": "voice", "bands": 16 }
```

## 场景预设

- 经典机器人合唱（载波：锯齿 pad）

  ```json
  {"type": "vocoder", "sidechain": "voice", "bands": 16}
  ```

- 复古粗糙

  ```json
  {"type": "vocoder", "sidechain": "voice", "bands": 8, "release_ms": 30}
  ```

- 清楚、可懂

  ```json
  {"type": "vocoder", "sidechain": "voice", "bands": 32, "high_hz": 10000}
  ```

## 经验与常见错误

- 挂在**载波**轨（合成器和弦、string machine、锯齿 pad）上；`sidechain` 写**调制**信号——人声（audio 音轨）或任何有节奏的轨。
- 调制信号那条轨常设 `"muted": true`：只用来塑造声音，自己不出声（侧链照样拿得到信号）。
- 载波要一直在响、泛音丰富；载波停了声码器就没声音。和弦跟着人声唱的音换。
- 调制信号是校准过的内置乐器时 gain_db 0 大致和原声一样响；人声录音偏轻时把 gain_db 往上调。
