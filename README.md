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
    -p, --pager <PAGER>              Sets the pager for viewing files [env: PAGER=]  [default:
                                     less]
```

### Listing packages

```sh
arch-repro-status -m orhun
```

![Listing packages](https://user-images.githubusercontent.com/24392180/115127741-928b6900-9fe1-11eb-9bad-f4589f2943f9.gif)


### Inspecting packages

```
arch-repro-status -i -m orhun -f BAD
```

![Inspecting packages](https://user-images.githubusercontent.com/24392180/115127748-a1721b80-9fe1-11eb-90cb-973a750515d7.gif)

## License

[The MIT License](https://opensource.org/licenses/MIT)
