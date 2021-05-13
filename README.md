# arch-repro-status

A CLI tool for querying the [reproducibility](https://reproducible-builds.org/) status of the Arch Linux packages using data from a [rebuilderd](https://wiki.archlinux.org/index.php/Rebuilderd) instance such as [reproducible.archlinux.org](https://reproducible.archlinux.org/).
It can show the reproducibility status of:
* packages that belong to an individual [package maintainer](https://wiki.archlinux.org/index.php/Arch_terminology#Package_maintainer) (uses the data from [archlinux.org/packages](https://archlinux.org/packages))
* currently installed packages on the system (uses the data from [pacman](https://wiki.archlinux.org/title/Pacman) local database)
You can inspect the build logs and [diffoscope](https://diffoscope.org/) of the packages by enabling the interactive mode via `-i`.

## Usage

```
arch-repro-status [FLAGS] [OPTIONS]
```

```
FLAGS:
    -d, --debug      Activates the debug mode
    -i, --inspect    Views the build log or diffoscope of the interactively selected package
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --maintainer <MAINTAINER>    Sets the username of the maintainer [env: MAINTAINER=]
    -r, --rebuilderd <URL>           Sets the address of the rebuilderd instance [env: REBUILDERD=]
                                     [default: https://reproducible.archlinux.org]
    -b, --dbpath <PATH>              Sets the path to the pacman database [env: DBPATH=]
                                     [default: /var/lib/pacman]
    -f, --filter <STATUS>            Sets the filter for package status [env: FILTER=]
                                     [possible values: GOOD, BAD, UNKWN]
    -p, --pager <PAGER>              Sets the pager for viewing files [env: PAGER=]
                                     [default: less]
    -c, --cache-dir <DIR>            Sets the cache directory for log files [env: CACHE_DIR=]
```

### Listing packages

```sh
arch-repro-status
```

![Listing user packages](./demo/list_user_pkgs.gif)

```sh
arch-repro-status -m orhun
```

![Listing maintainer packages](./demo/list_maintainer_pkgs.gif)

### Inspecting packages

```sh
arch-repro-status -i -f BAD
```

![Inspecting user packages](./demo/inspect_user_pkgs.gif)

```sh
arch-repro-status -i -m orhun -f BAD
```

![Inspecting maintainer packages](./demo/inspect_maintainer_pkgs.gif)

## Examples

```sh
# specify a maintainer (optional)
export MAINTAINER=<username>
# print out BAD results
arch-repro-status -f BAD
# enable interactive mode
arch-repro-status -i -d -f BAD
# use `bat` to view files
arch-repro-status -i -d --pager bat
# specify rebuilderd
arch-repro-status --rebuilderd https://wolfpit.net/rebuild/
```

## License

[The MIT License](https://opensource.org/licenses/MIT)
