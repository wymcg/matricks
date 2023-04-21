---
layout: page
title: Usage
permalink: /usage/
---

# Basic Usage
To run one or more plugins with Matricks on an LED matrix, run the following command.
```
matricks --width <WIDTH> --height <HEIGHT> --plugins <PLUGIN/DIRECTORY>
```
If the path provided to the `--plugins` option is a direct path to a plugin, Matricks will only run that plugin.
If the path provided is a directory, Matricks will attempt to run every plugin in that directory in sequence.

--- 

For a full list of options, run ``matricks --help``.

