---
layout: page
title: Usage
permalink: /usage/
---

# Basic Usage
In order to control an LED matrix with Matricks, the program must be provided with some basic information about the matrix.
At a minimum, you must specify the dimensions of the matrix and the plugin(s) to run on the matrix. 
This configuration can be provided to Matricks either "manually" via the command line, or "automatically" via a configuration file.

## Manual configuration
You may manually provide a configuration to Matricks using `matricks manual`.
To run a plugin (or a set of plugins in a directory), Matricks can be invoked as follows:

```
matricks manual [OPTIONS] --path <PLUGIN_PATH> --width <WIDTH> --height <HEIGHT>
```

This will run the plugin at the given path (or, if the path is a directory, all plugins in that directory) on the connected matrix.
For a complete list of available configuration options, run `matricks help manual`.

## Saving a configuration
Once you have confirmed that everything is working with `matricks manual`, you can save your configuration to a file using the `matricks save` command.
To save your configuration, Matricks can be invoked as follows:

```
matricks save <NEW_CONFIG_PATH> [OPTIONS] --path <PLUGIN_PATH> --width <WIDTH> --height <HEIGHT>
```

This is similar to `matricks manual`, but instead of running the plugin, Matricks will save the configuration information to a new TOML file at the given path.
`matricks save` has the same matrix and plugin configuration options as `matricks manual`.
See `matricks help save` for more information.

## Automatic configuration
If you have a TOML configuration file (created either by hand or by running `matricks save`), you can use it using `matricks auto`.
To run Matricks with a configuration file, Matricks can be invoked as follows:

```
matricks auto <CONFIG_PATH>
```

This command will use the configuration information in the given file to drive the matrix.
See `matricks help auto` for more information.

## Clearing the matrix
If for any reason you need to clear all LEDs on the matrix, Matricks can be invoked as follows:

```
matricks clear --width <WIDTH> --height <HEIGHT>
```

See `matricks help clear` for more information.
