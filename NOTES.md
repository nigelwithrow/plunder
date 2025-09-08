
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

# Watch

```python
def on_file_eval(self, status):
  if status.code == SyntaxError:
    console.log(status.message)
  else:
    self.loop.set(status.loop)
```
