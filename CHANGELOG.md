<a name="0.3.0"></a>
## 0.3.0 (2021-05-11)

#### Bug Fixes

*   Browse through pages to collect packages from archweb ([30a123d2](30a123d2))

#### Features

*   Add option for showing package information ([f9033b8a](f9033b8a))

<a name="0.2.1"></a>
## 0.2.1 (2021-05-08)

#### Bug Fixes

*   Enable paging for the package selection prompt ([d7d0a547](d7d0a547))

<a name="0.2.0"></a>
## 0.2.0 (2021-05-08)

#### Bug Fixes

*   Append project name for default cache directory only ([712f1689](712f1689))

#### Features

*   Use application-specific user agent for requests ([2b05187b](2b05187b))
*   Add `--debug` flag ([c1b97dda](c1b97dda))
*   Show debug log for cached files ([256e6c8a](256e6c8a))
*   Use cache directory for log files ([54fd05d9](54fd05d9))

<a name="0.1.0"></a>
## 0.1.0 (2021-04-21)

#### Bug Fixes

*   Disable colors while testing ([e261d09c](e261d09c))
*   Install pkg-config dependency for CI ([aaf86fe5](aaf86fe5))
*   Install openssl dependency for CI ([e5f3dae1](e5f3dae1))
*   Specify the default binary to run in Cargo.toml ([0f11d521](0f11d521))
*   Show warning message if no packages found ([ca0493af](ca0493af))
*   Check for the presence of filter before attempting to filter ([30ef2679](30ef2679))

#### Features

*   Add possible values for the '--filter' argument ([f46edb3e](f46edb3e))
*   Add shell completion generation script ([cf6dc81e](cf6dc81e))
*   Show epoch value of the package ([d1499e30](d1499e30))
*   Add interactive mode for inspecting packages ([f00d0481](f00d0481))
*   Support filtering the result via '--filter' ([1cac347f](1cac347f))
*   Initialize logging ([20a34e94](20a34e94))
*   Use values from command line arguments ([d0dad2e9](d0dad2e9))
*   Add initial implementation ([66c82d5b](66c82d5b))
