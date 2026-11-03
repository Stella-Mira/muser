//! 录人声的"遍"（take）：`muser view` 里录的每一遍都立刻存成工程目录下 `recordings/<轨名>-take-NN.wav`，
//! 并记进 `recordings/takes.json`（录在第几拍、多长、什么时候录的）。工程 JSON 只在点"保存"时才写进选定的那几遍。
//! `muser takes` 给 agent 看每一遍（长度、响度、实际开始唱 / 唱完的位置、工程用了没有），帮用户挑、剪、对齐。

use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use muser_core::Project;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// 一遍录音。`file` 相对工程文件所在的目录（就是音频片段里写的路径）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Take {
    pub file: String,
    pub track: String,
    /// 录在第几拍开始（片段的起点）。
    pub start: f64,
    pub seconds: f64,
    /// 录制时间（Unix 秒）。
    pub recorded: u64,
}

/// 一遍录音最大多少字节（48 kHz、16 位、单声道约 40 分钟）。
const MAX_BYTES: usize = 256 << 20;

fn dir(project: &Path) -> PathBuf {
    project.parent().unwrap_or(Path::new(".")).join("recordings")
}

/// 清单里的每一遍（没有清单时为空）。
pub fn list(project: &Path) -> Vec<Take> {
    std::fs::read_to_string(dir(project).join("takes.json")).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default()
}

fn write_list(project: &Path, takes: &[Take]) -> Result<()> {
    std::fs::write(dir(project).join("takes.json"), serde_json::to_string_pretty(takes)? + "\n")?;
    Ok(())
}

/// 存下一遍：`wav` 是浏览器传来的 WAV 文件内容。文件名由这里生成（轨名只留字母数字），只写进 `recordings/`。
pub fn store(project: &Path, track: &str, start: f64, wav: &[u8]) -> Result<Take> {
    if wav.len() > MAX_BYTES {
        bail!("录音太大（{} MB，最多 {} MB）", wav.len() >> 20, MAX_BYTES >> 20);
    }
    if !(start.is_finite() && start >= 0.0) {
        bail!("起点不对：{start}");
    }
    let parsed = muser_engine::wav::read(wav).map_err(|e| anyhow!("不是能读的 WAV：{e:?}"))?;
    let seconds = parsed.buffer.num_frames() as f64 / f64::from(parsed.sample_rate.max(1));
    if seconds < 0.05 {
        bail!("录音太短");
    }
    let stem: String = track.chars().filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_').take(40).collect();
    let stem = if stem.is_empty() { "vocal".to_string() } else { stem };
    let d = dir(project);
    std::fs::create_dir_all(&d).with_context(|| format!("建不了 {}", d.display()))?;
    let mut takes = list(project);
    let n = (1..).find(|k| !d.join(format!("{stem}-take-{k:02}.wav")).exists()).unwrap_or(1);
    let name = format!("{stem}-take-{n:02}.wav");
    std::fs::write(d.join(&name), wav)?;
    let take = Take {
        file: format!("recordings/{name}"),
        track: track.to_string(),
        start,
        seconds,
        recorded: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs()),
    };
    takes.push(take.clone());
    write_list(project, &takes)?;
    Ok(take)
}

/// 删掉一遍（文件与清单项）。`in_use` 为真时不删。
pub fn delete(project: &Path, file: &str, in_use: bool) -> Result<()> {
    let mut takes = list(project);
    let i = takes.iter().position(|t| t.file == file).ok_or_else(|| anyhow!("清单里没有 {file}"))?;
    if in_use {
        bail!("{file} 正在被工程使用：先换成别的一遍或删掉那个片段");
    }
    let path = dir(project).join(Path::new(file).file_name().ok_or_else(|| anyhow!("文件名不对"))?);
    let _ = std::fs::remove_file(path);
    takes.remove(i);
    write_list(project, &takes)
}

/// 工程里有没有片段在用这个文件。
pub fn referenced(p: &Project, file: &str) -> bool {
    p.tracks.iter().flat_map(|t| &t.clips).any(|c| c.audio.as_ref().is_some_and(|a| a.file == file))
}

/// 把一遍贴到工作副本：名为 `take.track` 的音频轨（没有就新建），和这一遍重叠的旧片段换掉。
pub fn place(p: &mut Project, take: &Take) -> Result<()> {
    let beats = take.seconds * p.tempo / 60.0;
    let i = match p.tracks.iter().position(|t| t.name == take.track) {
        Some(i) => i,
        None => {
            p.tracks.push(serde_json::from_value(json!({ "name": take.track, "instrument": "audio", "clips": [] }))?);
            p.tracks.len() - 1
        }
    };
    let t = &mut p.tracks[i];
    if t.instrument.trim() != muser_core::audio::INSTRUMENT {
        bail!("音轨 '{}' 不是音频轨（{}），换个轨名录", t.name, t.instrument);
    }
    let (a, z) = (take.start, take.start + beats);
    t.clips.retain(|c| c.start.0 + c.length.0 <= a + 1e-9 || c.start.0 >= z - 1e-9);
    t.clips.push(serde_json::from_value(clip_json(take, beats))?);
    t.clips.sort_by(|x, y| x.start.0.total_cmp(&y.start.0));
    Ok(())
}

fn clip_json(take: &Take, beats: f64) -> Value {
    json!({ "start": take.start, "length": (beats * 1000.0).round() / 1000.0, "audio": { "file": take.file } })
}

/// 保存时把一遍写进工程 JSON（与 [`place`] 一样：找轨或新建、换掉重叠的片段）。
pub fn place_json(v: &mut Value, take: &Take, tempo: f64) -> Result<()> {
    let beats = take.seconds * tempo / 60.0;
    let tracks = v.get_mut("tracks").and_then(Value::as_array_mut).ok_or_else(|| anyhow!("工程文件里没有 tracks"))?;
    let i = match tracks.iter().position(|t| t.get("name").and_then(Value::as_str) == Some(take.track.as_str())) {
        Some(i) => i,
        None => {
            tracks.push(json!({ "name": take.track, "instrument": "audio", "clips": [] }));
            tracks.len() - 1
        }
    };
    let t = tracks[i].as_object_mut().ok_or_else(|| anyhow!("音轨格式不对"))?;
    let clips = t.entry("clips").or_insert_with(|| json!([])).as_array_mut().ok_or_else(|| anyhow!("clips 不是数组"))?;
    let (a, z) = (take.start, take.start + beats);
    clips.retain(|c| {
        let s = c.get("start").and_then(Value::as_f64).unwrap_or(0.0);
        let l = c.get("length").and_then(Value::as_f64).unwrap_or(0.0);
        s + l <= a + 1e-9 || s >= z - 1e-9
    });
    clips.push(clip_json(take, beats));
    clips.sort_by(|x, y| x["start"].as_f64().unwrap_or(0.0).total_cmp(&y["start"].as_f64().unwrap_or(0.0)));
    Ok(())
}

/// `muser takes`：列出每一遍，或 `--clean` 删掉工程没用到的。
pub fn run(project_path: &str, clean: bool) -> Result<()> {
    let path = Path::new(project_path);
    let project = Project::from_json(&std::fs::read_to_string(path)?).map_err(|e| anyhow!("{e}"))?;
    let takes = list(path);
    if takes.is_empty() {
        println!("{project_path}：还没有录音（在 muser view 里点人声轨的 🎤 录）");
        return Ok(());
    }
    if clean {
        let unused: Vec<&Take> = takes.iter().filter(|t| !referenced(&project, &t.file)).collect();
        for t in &unused {
            delete(path, &t.file, false)?;
            println!("已删除 {}", t.file);
        }
        println!("删掉 {} 遍没用到的，留下 {} 遍", unused.len(), takes.len() - unused.len());
        return Ok(());
    }
    let map = project.bar_map();
    println!("{} 遍录音（recordings/takes.json）：", takes.len());
    for t in &takes {
        let (bar, beat) = map.bar_at(t.start);
        let file = dir(path).join(Path::new(&t.file).file_name().unwrap_or_default());
        let (voice, rms) = voice_span(&file).unwrap_or((None, f64::NEG_INFINITY));
        let span = voice.map_or("（几乎没声音）".to_string(), |(a, z)| format!("唱的部分 {a:.2}–{z:.2} 秒"));
        println!(
            "  {}  轨 {}  第 {} 小节第 {} 拍起  {:.2} 秒  平均 {:.0} dB  {}{}",
            t.file,
            t.track,
            bar + 1,
            (beat * 100.0).round() / 100.0 + 1.0,
            t.seconds,
            rms,
            span,
            if referenced(&project, &t.file) { "  ✓ 工程在用" } else { "" }
        );
    }
    println!("裁掉开头结尾的空白：改片段的 audio.offset（秒）、start / length（拍）；接缝用 fade_in / fade_out。");
    Ok(())
}

/// 实际唱的部分（秒，按 −45 dB 判断有没有声音）与整遍的平均电平（dB）。
fn voice_span(file: &Path) -> Option<(Option<(f64, f64)>, f64)> {
    let w = muser_engine::wav::read_file(file).ok()?;
    let x = w.buffer.channel(0);
    let sr = f64::from(w.sample_rate);
    let hop = (sr * 0.02) as usize;
    let mut on = Vec::new();
    let mut total = 0.0f64;
    for (k, c) in x.chunks(hop.max(1)).enumerate() {
        let ms = c.iter().map(|v| f64::from(*v).powi(2)).sum::<f64>() / c.len() as f64;
        total += ms * c.len() as f64;
        if 10.0 * ms.max(1e-12).log10() > -45.0 {
            on.push(k);
        }
    }
    let rms = 10.0 * (total / x.len().max(1) as f64).max(1e-12).log10();
    let span = on.first().zip(on.last()).map(|(a, z)| (*a as f64 * hop as f64 / sr, (*z + 1) as f64 * hop as f64 / sr));
    Some((span, rms))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wav(seconds: f64) -> Vec<u8> {
        let n = (48_000.0 * seconds) as usize;
        let mut b = muser_engine::AudioBuffer::<f32>::new(1, n);
        for (i, x) in b.channel_mut(0).iter_mut().enumerate() {
            *x = if i > n / 4 { (i as f32 * 0.05).sin() * 0.3 } else { 0.0 };
        }
        let mut out = Vec::new();
        muser_engine::wav::write(&mut out, &b, 48_000, muser_engine::WavFormat::Int16).unwrap();
        out
    }

    #[test]
    fn store_place_and_save() {
        let d = std::env::temp_dir().join(format!("muser-takes-{}", std::process::id()));
        std::fs::create_dir_all(&d).unwrap();
        let path = d.join("song.muser.json");
        std::fs::write(&path, r#"{ "tempo": 120, "tracks": [{ "name": "keys", "instrument": "poly.juno_keys", "clips": [] }] }"#).unwrap();
        let a = store(&path, "vocal", 4.0, &wav(2.0)).unwrap();
        let b = store(&path, "vo/../cal", 4.0, &wav(1.0)).unwrap();
        assert_eq!(a.file, "recordings/vocal-take-01.wav");
        assert_eq!(b.file, "recordings/vocal-take-02.wav", "轨名里的 / 和 .. 去掉，文件只会在 recordings/ 里");
        assert_eq!(list(&path).len(), 2);
        // 贴到工作副本：新建音频轨；第二遍和第一遍重叠，换掉它。
        let mut p = Project::from_json(&std::fs::read_to_string(&path).unwrap()).unwrap();
        place(&mut p, &a).unwrap();
        place(&mut p, &Take { track: "vocal".into(), ..b.clone() }).unwrap();
        let v = &p.tracks[1];
        assert_eq!((v.name.as_str(), v.instrument.as_str(), v.clips.len()), ("vocal", "audio", 1));
        assert_eq!(v.clips[0].audio.as_ref().unwrap().file, "recordings/vocal-take-02.wav");
        assert_eq!(v.clips[0].length.0, 2.0, "1 秒 @ 120 BPM = 2 拍");
        // 写进 JSON 一样。
        let mut j: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        place_json(&mut j, &a, 120.0).unwrap();
        assert_eq!(j["tracks"][1]["clips"][0]["audio"]["file"], json!("recordings/vocal-take-01.wav"));
        assert!(delete(&path, &a.file, true).is_err(), "在用的不删");
        delete(&path, &a.file, false).unwrap();
        assert_eq!(list(&path).len(), 1);
        assert!(store(&path, "vocal", 0.0, b"not a wav").is_err());
        let _ = std::fs::remove_dir_all(&d);
    }
}
