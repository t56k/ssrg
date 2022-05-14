# Double commit and push bash script

I wrote a quick bash script to publish to the two relevant repos (the [codebase](https://github.com/t56k/ssrg) and [this](https://github.com/t56k/t56k.github.io) site) at once.

```
#!/usr/bin/env bash

git add .
git commit -m "$1"
git push origin main

cp public/* ../t56k.github.io
cd ../t56k.github.io
git add .
git commit -m "$1"
git push origin main

cd ../ssrg
```

This is fine for a first-pass but it doesn't build the HTML first, meaning I still need to `cargo run` before running it. The best option here is to add some kind of build flag to write the HTML files.
