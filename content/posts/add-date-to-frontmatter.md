---
title: "Add Date from File Name to Frontmatter"
date: 2019-10-05T23:11:14-04:00
---

I figured I should write a blog post since I haven't written anything in four years or so. I recently moved my blog from Jekyll to Hugo, and there are a few differences between the two when writing posts for them. One of which is that Hugo seems to only read the date for the post from the Front Matter in the Markdown file. Jekyll would read it from the filename. I've been messing around with Python again and decided to write a small Python script to add these dates to the Front Matter for me. I don't have that many posts, but I have enough for it to be annoying to do by hand. Below is the script in its entirety with an in-depth explanation below.

```python
#!/usr/bin/env python3
from os import listdir
from os.path import isfile, join
import sys

only_files = [f for f in listdir(sys.argv[1]) if isfile(join(sys.argv[1], f))]
file_dates = [f[:10] for f in only_files]

for (date, filename) in zip(file_dates, only_files):
  filepath = join(sys.argv[1], filename)
  contents = list()
  with open(filepath, 'r') as fd:
    contents = fd.readlines()

  contents = [contents[0]] + [f'date: {date}\n'] + contents[1:]

  with open(filepath, 'w') as fd:
    fd.writelines(contents)
```

Using the script is easy. Save it to a file called `apply_dates.py`, give it execute rights (`chmod +x apply_dates.py`), and run the file, passing it the path to the directory containing the posts. An example run would look like this:

```shell
./apply_dates.py /content/posts
```

## More explanation

```python
#!/usr/bin/env python3
from os import listdir
from os.path import isfile, join
import sys
```

The first few lines are just a shebang and the imports for modules we use in the script. The shebang allows us to execute the script without the `python3` binary. We can just execute the file itself.

```python
only_files = [f for f in listdir(sys.argv[1]) if isfile(join(sys.argv[1], f))]
file_dates = [f[:10] for f in only_files]
```

These lines use list comprehensions to get the file names in the directory we pass as the first argument to this script. We then iterate over those file names and get the dates for each file under the assumption the file names look like so: `2018-03-12-some-post-name.md`.

```python
for (date, filename) in zip(file_dates, only_files):
```

Now that we have the files and their dates in two lists, we `zip` these two lists together and iterate over the resulting list of tuples.

```python
  filepath = join(sys.argv[1], filename)
  contents = list()
  with open(filepath, 'r') as fd:
    contents = fd.readlines()
```

We create the path to each file, initialize a list to hold the file contents in, open the file, and then read in the file as a list into the `contents` variable.

```python
  contents = [contents[0]] + [f'date: {date}\n'] + contents[1:]
```

Here we split up the contents, again making an assumption that the first line is `---`. So we place a line below the first line with the word `date: ` and then the date we pulled off the file name.

```python
  with open(filepath, 'w') as fd:
    fd.writelines(contents)
```

Here we are just opening the file again in write mode, which truncates the file, and writes the contents out to the file again.

## Conclusion

I figured there'd be an easier way to append lines to the middle of a file, but I couldn't seem to find anything in the minutes of Googling I did. This worked well though for my use-case. Hopefully someone else finds it useful as well.

BTW, for anyone wondering, many of the migration steps needed to move from Jekyll to Hugo can be done using `sed`. I leave that for the fun of the reader to figure out.
