<!-- 由 scripts/gen_skills.py 生成；参数表来自代码描述表，请改代码或脚本后重新生成 -->

# 乐器：人声 / 合唱

音符 `pitch` 使用 **MIDI 音高（60 = 中央 C）**。查看最新列表：`muser instruments --family choir`。

| instrument | 推荐音域 | 复音数 | 说明 |
|---|---|---|---|
| `choir.aah` | 48–79 | 8 | 合唱 'aah'：开阔明亮，适合和弦铺底 |
| `choir.ooh` | 48–79 | 8 | 合唱 'ooh'：柔和暗淡 |
| `choir.solo` | 48–79 | 2 | 独唱人声（元音） |

## 可调参数（instrument_params）

在所选乐器/预设的音色上覆盖，只写想改的项；其余保持原样。`muser instruments <名称>` 可看到该预设的当前值。

| 参数 | 范围 | 自动化 | 说明 |
|---|---|---|---|
| `vowel` | 0 – 4，或 `"a"` / `"e"` / `"i"` / `"o"` / `"u"` | ✓ | 元音："a" "e" "i" "o" "u"，或 0-4 之间的数（如 1.5 = e 与 i 之间）。适合做自动化 |
| `attack` | 0.005 – 2 s | ✓ | 起音时间 |
| `release` | 0.01 – 4 s | ✓ | 松开后淡出时间 |
| `vibrato_rate` | 0 – 12 Hz | ✓ | 颤音速度 |
| `vibrato_depth` | 0 – 200 cent | ✓ | 颤音深度（音分） |
| `brightness` | 500 – 12000 Hz | ✓ | 音色亮度 |
| `breathiness` | 0 – 1 | ✓ | 气声 |
| `singers` | 1 – 6（整数） | — | 每个音由几个人唱：1 独唱；4-6 合唱 |
| `formant_shift` | 0.7 – 1.4 | ✓ | 共振峰缩放：<1 更深更大的声音；>1 更细更小 |

自动化列：✓ = 正在发声的音也立即跟随；下个音 = 只影响之后开始的音；— = 不能自动化。见 [automation.md](../automation.md)。

各乐器的预设取值：

| instrument | `vowel` | `attack` | `release` | `vibrato_rate` | `vibrato_depth` | `brightness` | `breathiness` | `singers` | `formant_shift` |
|---|---|---|---|---|---|---|---|---|---|
| `choir.aah` | 0 | 0.18 | 0.35 | 5.2 | 22 | 4000 | 0.2 | 6 | 1 |
| `choir.ooh` | 4 | 0.18 | 0.35 | 5.2 | 22 | 2500 | 0.3 | 6 | 1 |
| `choir.solo` | 0 | 0.06 | 0.2 | 5.6 | 40 | 6000 | 0.12 | 1 | 1 |

### 调音示例

- 'ooh' 元音、6 人、偏暗

  ```json
  { "instrument": "choir.aah", "instrument_params": {"vowel": "o", "singers": 6, "brightness": 2500} }
  ```


## 经验与常见错误

- 合唱适合长音和弦铺底（2-4 拍）；短促的音听起来不自然。
- `choir.solo` 复音数 2，只适合单旋律。
- **哼唱歌词**：音符写 `"lyric": "la"`，或在 ABC 里加 `w:` 歌词行按顺序对上音符（`w: yue liang sheng qi`；`hel-lo` 两个音节、`*` 跳过一个音、`_` 延长上一个音节）。拼音和英文拼法都认，声调数字忽略。**只唱元音**：每个音唱音节里的元音（a e i o u；拼音的 e 唱成"鹅"、ü 唱成 i），ai / ao / ia 这类复元音在音符里滑过去，-n / -ng 结尾合上嘴哼一下。辅音不唱（0.4 的辅音模拟听起来像另一个声音，默认关闭），所以听起来是带元音变化的哼唱，不是能听出字的演唱；要真的唱词，导出带歌词的 MIDI 交给歌声合成软件，或让用户自己录人声。歌词也写进 MIDI 导出（lyric 事件）。
