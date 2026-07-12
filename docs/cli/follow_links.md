---
layout: docs
---

# Following Links

The `--follow-links` flag makes `specdown run` follow local Markdown links
found in the given spec files, and run every linked file too - recursively.
This lets you run a whole documentation set (e.g. an `index.md` that links
out to other pages) by pointing `specdown` at just the entry file.

## Enabling the feature

```shell
specdown run --help 2>&1 | grep -A 2 -- '--follow-links'
```

```text
      --follow-links
          Follow local Markdown links found in spec files and run every linked file too, recursively. Files are deduplicated by canonical path, so link cycles are handled safely and each file only runs once.
          
```

## Default Behaviour: Links Are Not Followed

Given `link_index.md`, which links to `link_target.md`:

````markdown
# Link Index

See [target](link_target.md) for more.

```shell,script(name="index_script")
echo "index ran"
```

```text,verify(script_name="index_script")
index ran
```
````

````markdown
# Link Target

```shell,script(name="target_script")
echo "target ran"
```

```text,verify(script_name="target_script")
target ran
```
````

Running `specdown run link_index.md` without `--follow-links` only runs the
file given on the command line:

```shell
specdown run link_index.md
```

```text
Running tests for link_index.md:

  ✓ running script 'index_script' succeeded
  ✓ verifying stdout from 'index_script' succeeded

  2 functions run (2 succeeded / 0 failed)

```

## Following Links with `--follow-links`

Running the same command with `--follow-links` also runs `link_target.md`:

```shell
specdown run --follow-links link_index.md
```

```text
Running tests for link_index.md:

  ✓ running script 'index_script' succeeded
  ✓ verifying stdout from 'index_script' succeeded

  2 functions run (2 succeeded / 0 failed)

Running tests for link_target.md:

  ✓ running script 'target_script' succeeded
  ✓ verifying stdout from 'target_script' succeeded

  2 functions run (2 succeeded / 0 failed)

```

## Following Links Recursively

Links are followed to any depth. Given `recursive_a.md`, which links to
`recursive_b.md`, which in turn links to `recursive_c.md`:

````markdown
# Recursive A

See [b](recursive_b.md).

```shell,script(name="recursive_a")
echo "a ran"
```

```text,verify(script_name="recursive_a")
a ran
```
````

````markdown
# Recursive B

See [c](recursive_c.md).

```shell,script(name="recursive_b")
echo "b ran"
```

```text,verify(script_name="recursive_b")
b ran
```
````

````markdown
# Recursive C

```shell,script(name="recursive_c")
echo "c ran"
```

```text,verify(script_name="recursive_c")
c ran
```
````

Running `specdown run --follow-links recursive_a.md` runs all three files:

```shell
specdown run --follow-links recursive_a.md
```

```text
Running tests for recursive_a.md:

  ✓ running script 'recursive_a' succeeded
  ✓ verifying stdout from 'recursive_a' succeeded

  2 functions run (2 succeeded / 0 failed)

Running tests for recursive_b.md:

  ✓ running script 'recursive_b' succeeded
  ✓ verifying stdout from 'recursive_b' succeeded

  2 functions run (2 succeeded / 0 failed)

Running tests for recursive_c.md:

  ✓ running script 'recursive_c' succeeded
  ✓ verifying stdout from 'recursive_c' succeeded

  2 functions run (2 succeeded / 0 failed)

```

## Link Cycles Are Handled Safely

Files are tracked by their canonical path as they're discovered, so a link
cycle doesn't cause an infinite loop, and each file still only runs once.
Given `cycle_a.md`, which links to `cycle_b.md`, which links back to
`cycle_a.md`:

````markdown
# Cycle A

See [b](cycle_b.md).

```shell,script(name="cycle_a")
echo "a ran"
```

```text,verify(script_name="cycle_a")
a ran
```
````

````markdown
# Cycle B

See [a](cycle_a.md).

```shell,script(name="cycle_b")
echo "b ran"
```

```text,verify(script_name="cycle_b")
b ran
```
````

```shell
specdown run --follow-links cycle_a.md
```

```text
Running tests for cycle_a.md:

  ✓ running script 'cycle_a' succeeded
  ✓ verifying stdout from 'cycle_a' succeeded

  2 functions run (2 succeeded / 0 failed)

Running tests for cycle_b.md:

  ✓ running script 'cycle_b' succeeded
  ✓ verifying stdout from 'cycle_b' succeeded

  2 functions run (2 succeeded / 0 failed)

```

## Enabling via `specdown.toml`

`--follow-links` can also be turned on for a directory by adding a
`specdown.toml` file with `follow_links = true` under the `[run]` table,
instead of passing the flag on every invocation.

Given the same `link_index.md` / `link_target.md` pair from earlier, plus a
`specdown.toml`:

```toml
[run]
follow_links = true
```

Running `specdown run link_index.md`, with **no** `--follow-links` flag,
still follows the link because of the config file:

```shell
specdown run link_index.md
```

```text
Running tests for link_index.md:

  ✓ running script 'index_script' succeeded
  ✓ verifying stdout from 'index_script' succeeded

  2 functions run (2 succeeded / 0 failed)

Running tests for link_target.md:

  ✓ running script 'target_script' succeeded
  ✓ verifying stdout from 'target_script' succeeded

  2 functions run (2 succeeded / 0 failed)

```

## Non-Local Links Are Ignored

Only local Markdown files (ending in `.md`, and not an absolute URL or
in-page anchor) are followed. Given `external_links.md`, which links to an
external URL, a `mailto:` address, an in-page anchor, and an image:

````markdown
# External Links

[External site](https://example.com/docs.md)

[Email us](mailto:someone@example.com)

[Jump to a section](#somewhere)

![An image](image.png)

```shell,script(name="external_links")
echo "ran"
```

```text,verify(script_name="external_links")
ran
```
````

None of those links point at a local spec file, so only `external_links.md`
itself is run - no error occurs, and nothing else is fetched or read:

```shell
specdown run --follow-links external_links.md
```

```text
Running tests for external_links.md:

  ✓ running script 'external_links' succeeded
  ✓ verifying stdout from 'external_links' succeeded

  2 functions run (2 succeeded / 0 failed)

```

## Broken Links Are Reported as an Error

If a linked file doesn't exist, following links fails with a clear error
rather than being silently ignored. Given `broken_link.md`, which links to a
file that doesn't exist:

```markdown
# Broken Link

See [missing](does_not_exist.md).
```

```shell
specdown run --follow-links broken_link.md
```

```text
  ✗ Failed to follow link to 'does_not_exist.md': No such file or directory (os error 2)
```

