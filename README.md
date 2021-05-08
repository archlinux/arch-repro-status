# arch-repro-status

A CLI tool for querying the [reproducibility](https://reproducible-builds.org/) status of the Arch Linux packages using the data from [archlinux.org](https://archlinux.org/packages) and a [rebuilderd](https://wiki.archlinux.org/index.php/Rebuilderd) instance such as [reproducible.archlinux.org](https://reproducible.archlinux.org/).

It only works with [official repositories](https://wiki.archlinux.org/index.php/Official_repositories) and it is designed for querying the packages of an individual [package maintainer](https://wiki.archlinux.org/index.php/Arch_terminology#Package_maintainer).

## Usage

```
arch-repro-status [FLAGS] [OPTIONS] --maintainer <MAINTAINER>
```

```
FLAGS:
    -i, --inspect    Views the build log or diffoscope of the interactively selected package
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --maintainer <MAINTAINER>    Sets the username of the maintainer [env: MAINTAINER=]
    -r, --rebuilderd <URL>           Sets the address of the rebuilderd instance [env: REBUILDERD=]
                                     [default: https://reproducible.archlinux.org]
    -f, --filter <STATUS>            Sets the filter for package status [env: FILTER=]
                                     [possible values: GOOD, BAD, UNKWN]
    -p, --pager <PAGER>              Sets the pager for viewing files [env: PAGER=]
                                     [default: less]
    -c, --cache-dir <DIR>            Sets the cache directory for log files [env: CACHE_DIR=]
```

### Listing packages

```sh
arch-repro-status -m orhun
```

![Listing packages](./demo/listing_packages.gif)


### Inspecting packages

```
arch-repro-status -i -m orhun -f BAD
```

![Inspecting packages](./demo/inspecting_packages.gif)

## Examples

```sh
export MAINTAINER=<username>
# print out BAD results
arch-repro-status -f BAD
# enable interactive mode
arch-repro-status -i -f BAD
# use `bat` to view files
arch-repro-status -i --pager bat
# specify rebuilderd
arch-repro-status --rebuilderd https://wolfpit.net/rebuild/
```

## License

[The MIT License](https://opensource.org/licenses/MIT)
