---
# Feel free to add content and custom Front Matter to this file.
# To modify the layout, see https://jekyllrb.com/docs/themes/#overriding-theme-defaults

layout: home
---

> "Teach an old matrix new tricks..."

**Matricks** is a WASM-based extensible LED matrix control tool intended for use on Raspberry Pi devices. 
LED matrix functionality is defined by user-developed plugins, or "tricks", which can be developed in any language that is supported by the Extism PDK. 
On non-Raspberry Pi devices, Matricks will simulate a LED matrix and display the simulated matrix state in real time.

In general, Matricks aims to be:
1. **Powerful** - Anything you would want an LED matrix to do, you should be able to do with Matricks.
2. **Transparent** - If something goes wrong in the host or a plugin, the problem is clearly and transparently communicated to the user.
3. **Easily sharable** - Sharing plugins should be as simple as sharing a single file.
