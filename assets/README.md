# GUI assets

`github-mark-32.alpha` is a 32 × 32, row-major, 8-bit alpha mask of the GitHub mark.
The GUI tints it to match the current text color, so it works in light and dark themes.

Source: https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png

Generated with Pillow by converting the source to grayscale, inverting it, resizing
to 32 × 32 with Lanczos resampling, and saving the raw bytes. No image decoder or
network access is needed at runtime.
