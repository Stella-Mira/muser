# Changelog

Each release's notes on GitHub are the section for its tag below (the release workflow copies it).
Write the next version's section here before tagging; the heading must be `## vX.Y.Z`.

## v0.4.0

### New

- **One-line install**: `curl -fsSL https://raw.githubusercontent.com/claw-works/muser/main/scripts/install.sh | sh` (Windows: `irm …/scripts/install.ps1 | iex`) downloads the right release, checks it against `SHA256SUMS.txt`, installs it under `~/.local/share/muser/<version>` (`%LOCALAPPDATA%\muser\<version>` on Windows) and puts `muser` and `muser-mcp` on `PATH`; Linux without ALSA gets the headless build.
- **`muser self-update`** updates to the latest release through the same script (`--check` only looks, `--version` picks one).
- **Record a voice in `muser view`**: a "Record a voice" panel records from the playhead (or just the loop range) over the backing in the browser, with a count-in and latency compensation (plus a manual nudge). Every take is saved at once as `recordings/<track>-take-NN.wav` and listed in `recordings/takes.json`; the take is laid on an audio track (created if needed, replacing what it overlaps) as a pending change — the project file only changes when you press Save. Takes can be swapped in or deleted from the panel. `muser takes <project>` shows each take's length, level, where the singing actually starts and ends, and whether the project uses it, so an agent can trim and align them (`--clean` removes unused takes). A project's `lyrics` text is shown next to the recorder, and as karaoke at the bottom of the screen: each line highlights character by character in time with the sung syllables (Chinese characters map one-to-one onto the `lyric` syllables, English by word), with the next line below; a Lyrics switch in the transport bar turns it off.
- **`muser render -o song.mp3`** writes MP3 directly (320 kbit/s, with title and tempo tags) with manguebyte's encoder.
- **`muser clip split <project> --track vocal --at <beat>` / `muser clip move … --clip N --to <beat> | --by-ms -70`**: the view's scissors and nudges for agents, editing only that track's clips in the project file.
- **Cut and move audio clips in `muser view`**: the ✂ button (or C) turns on scissors — click an audio clip to cut it there into two clips with short fades; drag an audio clip to move it (Shift snaps to 1/16 notes); click one to select it and nudge it with ← / → by 10 ms (Shift: a 1/16 note). Edits are pending changes until Save.
- **Humming lyrics**: notes on `choir.*` instruments can carry a `lyric` (pinyin or English spelling), or an ABC `w:` lyric line maps syllables onto the notes in order. Each note sings the syllable's vowel (diphthongs glide within the note, -n / -ng close to a hum), so a line hums with the vowels of its words; consonants are not sung in 0.4. Lyrics go into MIDI exports as lyric events. Sample: `samples/v0.4/moon-rising`.
- **Loops that run across sections**: a part with `"align": "global"` counts its loop from the start of the song instead of restarting at every section, so polymeter layers (7, 13, 17 steps …) keep drifting through the whole form without rewriting them as absolute clips. Consecutive sections that use the same global part become one clip, so long notes sound across section lines, and `muser view` shows it as one continuous block.
- `muser view` draws parts that are used more than once as rounded blocks, the same shade and the part's name for the same part — like patterns in FL Studio — and its bar lines follow the real bar lengths (they were wrong for meters like 6/8).
- **A time signature per section**: `"sections": { "bridge": { "bars": 4, "time_signature": { "numerator": 7, "denominator": 8 } } }`. Chords follow the section's bar length; `analyze` bar numbers, warnings, MIDI export (a time-signature event at each change), `export-data` and the bar lines in `muser view` follow the meter changes. The drummer still writes 4-beat grooves, and ABC export lays bars out in the project meter (with a note).
- **Export for a DAW**: `muser export-daw <project> -o <folder>` (MCP `export_daw`) writes everything Ableton Live, Cubase or Logic needs to carry on: a MIDI file of every instrument track (tempo, meter, section markers), 24-bit stems aligned at bar 1, the mix, and IMPORT.md, a guide with the steps for Live, Cubase, Logic, FL Studio, Studio One, Bitwig, REAPER, GarageBand and Pro Tools.
- **Stems**: `muser render <project> --stems <dir>` (MCP `export_stems`) writes one WAV per unmuted track — with its effects, fader, pan and what it sends to buses (reverb follows the track), sidechains intact, without the master effects — all the same length, for mixing on in a DAW.
- **Data for visuals**: `muser export-data <project> [-o data.json] [--fps 30]` (MCP `export_data`) writes one JSON with every track's notes in seconds (as rendered), the sections, the automation curves of tracks and master, and with `--fps` a per-frame level and low-band envelope — everything the *Electric Sheep* videos had to scrape by hand.
- **Section markers in WAV files**: `muser render` (and stems) write a cue point at the start of every section of the `form` (`verse`, `chorus`, `chorus#2`…), plus the title and tempo, so DAWs and players can jump by section.
- **`muser migrate <project> --from 0.3.0`** converts a project written for an older muser so it sounds the same now: for 0.3.0 projects it shifts the `gain_db` (and `gain_db` automation) of built-in instrument tracks by their calibration. It lists the changes; `--write` writes them back.

### Better feedback for agents

- `analyze --audio` warns about sections that are almost silent (average level below −45 dB, or no sound at all) and says what it can see in the project: no notes there, the master volume curve low there, the playing tracks' volume low there, or both stacked (the master curve and track fades together, as in the dreamer songs). Intentional silence can be ignored.

### Fixes

- Plugins that print their own log lines (for example Kontakt 8's `[…] [error] PresetSlotManager::selectSlot: slot not found …`, once per instance) no longer flood the terminal during `render`, `view` and the other commands. Lines that start with a log timestamp go to `muser-plugin.log` in the temp folder, with one note saying where; muser's own output is unchanged. Set `MUSER_PLUGIN_LOG=show` to see them again (macOS and Linux).

## v0.3.1

### Upgrade notes

- **Built-in instrument levels are calibrated.** `gain_db 0` now means about −20 LUFS on a reference phrase for every built-in instrument, so instruments that used to be much louder or quieter than the rest now sit together. A project written for 0.3.0 may come out with a different balance; check it with `muser mix-report` and adjust `gain_db`. Custom `synth.poly` patches are not trimmed.

### New

- **12 new effects**: `bitcrusher`, `tremolo`, `auto_pan`, `ring_mod`, `frequency_shifter`, `widener`, `transient_shaper`, `multiband_compressor`, `tape` (saturation with wow / flutter), `vocoder` (modulated by another track through `sidechain`), `granular_delay`, `formant_filter`. All have ranges, automation and skill pages (`muser effects <name>`).
- **Tuning**: a project-level `tuning` for pitched built-in instruments and the SFZ sampler — just intonation, Pythagorean, meantone, Werckmeister III, Kirnberger III, Vallotti, Young, custom deviations, N-EDO, Scala `.scl` / `.kbm`, reference pitch, detune. Drum kits and plugins keep standard pitch. Samples: `tuning/just-chorale`, `tuning/maqam-rast`.
- **Plugin multi-out**: `@outputs` entries can take a plugin's auxiliary output bus (`aux`) and send a channel or bus straight to a muser bus (`bus`), so one plugin instance (e.g. a drum sampler) feeds several mixer buses. `muser routing` lists these taps.
- **Multichannel plugin instruments**: `@outputs` mixes multichannel outputs down to stereo; `@warmup` waits for plugins that load samples in the background (e.g. DrumGizmo).
- **Large SFZ libraries** (over 1 GB) stream samples from disk instead of loading everything into memory; the rendered output is identical.
- `muser sounds describe` reports a sound's volume against the built-ins.

### Better feedback for agents

- `validate` and `render` (CLI and MCP) list warnings without failing: ABC bars that don't match the meter, parts that cover only part of their section, notes far outside an instrument's range, drum notes on keys the kit doesn't have (they make no sound), and clipping on render.
- `analyze` adds an ungated `rms_db` per section (near-silent sections no longer hide behind LUFS gating), names the section, bars and a `form` mute snippet in its hints, and merges masking hints per track.
- A `grid` with `step` in the wrong place points to `grid.step`.

### Fixes

- `muser view`, `loop edit` and `poly edit` move to the next free port when the default one is taken.
- Updated the vendored manguebyte to 38eb44b.
