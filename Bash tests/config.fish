source /usr/share/cachyos-fish-config/cachyos-config.fish

# overwrite greeting
# potentially disabling fastfetch
#function fish_greeting
#    # smth smth
#end

# pnpm
set -gx PNPM_HOME "/home/blahaj/.local/share/pnpm"
if not string match -q -- $PNPM_HOME $PATH
  set -gx PATH "$PNPM_HOME" $PATH
end
# pnpm end

# Added by LM Studio CLI (lms)
set -gx PATH $PATH /home/blahaj/.lmstudio/bin
# End of LM Studio CLI section

  function codex-sessions
      find ~/.codex/sessions -type f -name '*.jsonl' | sort
  end

  function codex-recent
      set -l count 20
      if test (count $argv) -ge 1
          set count $argv[1]
      end
      find ~/.codex/sessions -type f -name '*.jsonl' | sort | tail -n $count
  end

  function codex-last
      codex resume --last
  end

  function codex-fork-last
      codex fork --last
  end

  function codex-open-last
      set -l f (find ~/.codex/sessions -type f -name '*.jsonl' | sort | tail -n 1)
      if test -n "$f"
          if set -q EDITOR
              $EDITOR $f
          else
              kate $f
          end
      end
  end

eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv fish)"
fish_add_path --move --append /home/linuxbrew/.linuxbrew/bin /home/linuxbrew/.linuxbrew/sbin

# Restart KDE Plasma shell
alias replasma='systemctl --user restart plasma-plasmashell.service'
alias "pacman-update"="sudo pacman -Syy"



# Added by Antigravity CLI installer
set -gx PATH "/home/blahaj/.local/bin" $PATH
