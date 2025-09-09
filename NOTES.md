
### What is an instrument?
An instrument is just a buffer.
```rs
fn(usize) -> Option<Sample>
```
For an audio instrument, [0] is the first sample and [1] is the second sample
For a midi instrument, [0] is the first sample of C0, [1] is the second sample of C0, [X] is the first sample of D0

### What is a parser?
A parser converts a fancy string to a list of buffer indexes to be used on instruments.
```rs
fn(&str) -> Vec<Option<u32>>
```

# Contexts

Like traits. Contexts should be implemented even for intermediate steps

- Stream { play, pause }

# Parser

Takes string,
returns ??




# Instruments

## OfWav
```rs
type OfWav impl LuaUserData, Instrument<1>, Instrument<2>;
Lua_fn load: (path: String) -> LuaResult<OfWav>;
```

## P1

```rs
// Sheet
type Sheet impl LuaUserData;
Rust_fn create_sheet: (s: String) -> LuaResult<Sheet>;

// Instrument Map
type InstrumentMap impl LuaUserData;
Rust_fn create_instrument_map: (s: LuaTable) -> LuaResult<Option<InstrumentMap>>;

// 
type P1 impl LuaUserData, Instrument<1>, Instrument<2>;
Lua_fn create_p1: (config: Option<LuaValue>, sheet: String, instrument_map: LuaTable) -> LuaResult<P1>;
```

# Watch

```python
def on_file_eval(self, status):
  if status.code == SyntaxError:
    console.log(status.message)
  else:
    self.loop.set(status.loop)
```
