# D2fps

D2fps is a multi-version framerate unlocker for Diablo II.

## Features

* Unlocks The framerate both in-game and in-menu.
* Builtin frame limiter. No need for v-sync to be enabled.
* Auto-detects the refresh rate of the monitor the game is currently displayed on.
* CPU-use fix both in-menu and in-game. Diablo II will no longer run a single core at 100% use.
* Motion smoothing when running at framerates other than Diablo II's native 25fps.

## Compatibility

Supports almost all Diablo II versions and all video modes. The following versions are currently *not* supported: `1.04`, `1.09c`, `1.10b`, `1.10s`, `1.13a`, `1.13b`, `1.14a`, `1.14b`

This is *not* compatible with other framerate and CPU-use patches. Trying to use both at the same time may result in d2fps not loading, features not working correctly, or crashes. Most other code edits should be compatible, but there's no guarantee.

## Use

### Installation

Extract `d2fps.dll` into the Diablo II folder and have dll loaded via some external method. PlugY can be used by adding it to either `DllToLoad` or `DllToLoad2` in `PlugY.ini`. The D2ModSystem can bu used by adding `d2fps = d2fps.dll` to `D2Mod.ini`.

### Configuration

D2fps can be configured via `d2fps.ini` or the command line. This allows configuring the frame limiter and controlling which features are enabled. See `d2fps.ini` for more details.

## License

D2fps is licensed under the GNU General Public License, Version 3.0 <https://www.gnu.org/licenses/gpl-3.0.html>.
