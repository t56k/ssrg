# Date and time formatting

Another quality-of-life improvement is the date and time formatting issue. No one really needs all those microseconds for a blog post.

> 2022-05-13 10:56:56.733270322 +10:00 Case conversion

Given our datetime is collected roughly like so:

```
use chrono::{DateTime, Local};

let modified_systemtime = e.metadata()
    .expect("failed to get metadata")
    .modified()
    .expect("failed to get modified");

let modified: DateTime<Local> = modified_systemtime.into();

modified.to_string()
```

We have a couple of options. Astute readers will notice we're using [chrono](https://docs.rs/chrono/0.4.19/chrono/index.html#formatting-and-parsing) which handily comes with a couple of formatting options. The work's been done for us:

```
modified.to_rfc2822()
```

Returns this:

>  Fri, 13 May 2022 10:56:56 +1000 Case conversion

Much better.
