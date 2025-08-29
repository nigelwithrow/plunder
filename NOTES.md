
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
