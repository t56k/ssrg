# Shell and vim reference 101

I've had a file live on my two machines for years now that has various commands written in it that either refuse to stick in my head, or their syntax is regrettable, or they're used just infrequently enough to forget. I'm putting them here so I can access them in other places, and maybe they'll help you, too.

## neovim

- `/-.*-/` searches for at least two hyphens
- `:%s/^/whatever` prepend to every line
- `:'<,'>!bc` do the math
- `:cfdo s/what/ever/g` find and replace in quickfix
- `:g/whatever/d` delete lines containing whatever
- `:gr whatever` fill quickfix
- `:r !ls -la ~/whatever` copies output of ! into buffer
- `:r !sed -n 10,25p whatever` read lines 10-25 from whatever into buffer
- `:verbose map <leader>` get those mapping conflicts
- `:w %:p:h/whatever` write to path
- `:wa` write all buffers
- `<leader>rg whatever` grep exactly, see [init.vim](https://github.com/t56k/dotfiles/blob/main/init.vim)
- `vap` visually select whole paragraph
- `:n *.md` open all
- `:sav %:h/new-whatever.rs`
- `Ctrl-t` push telescope to trouble's quickfix

## zsh

- `for i in $(cat whatever); do mv ${i}* new-whatever; done` move partial matches
- `fd -H 'whatever' -tf -X rm -i` remove interactively with `fd`

## tmux

- `Ctrl-a [` enter scrolling mode
- `Ctrl-a :swap-window -s 1 -t 3` swap tabs in tmux

Plenty more are detailed in my [dotfiles](https://github.com/t56k/dotfiles) too.

