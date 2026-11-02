# Changelog

Each release's notes on GitHub are the section for its tag below (the release workflow copies it).
Write the next version's section here before tagging; the heading must be `## vX.Y.Z`.

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
