# amoled-maker
A simple program to check the black pixel percentage of an image and make it true black.

## Usage
| Parameters | Description | Optional |
| ---------- | ----------- | -------- |
| \<path\>   | Path to the image to check/edit | No |
| -p | Check true black pixel percentage | Yes |
| -c \<black point\> | Creates a new image at the path `./amoled_image.png` replacing pixels under the black point threshold with true black pixels | Yes |

### Example Usage
`amoled-maker.exe ./path/to/image.png -p -c 20`

### Examples

This is an extreme example

Command used `.\amoled-maker.exe image.jpeg -c 80`

|Initial image|Final image|
|-------------|-----------|
|[![initial image](https://i.imgur.com/5FqISKo.jpg)](https://i.imgur.com/OakTA77.jpg)|[![processed image](https://i.imgur.com/3DOZGvT.jpg)](https://i.imgur.com/3xwpEje.png)|
|0% true black|71.52% true black|
> Click either image for the full resolution!

A more realistic scenario.

Command used `.\amoled-maker.exe image.jpeg -c 40`

|Initial image|Final image|
|-------------|-----------|
|[![initial image](https://i.imgur.com/cGi2B6X.jpg)](https://i.imgur.com/t8syKBD.jpg)|[![processed image](https://i.imgur.com/jxfJ2ev.jpg)](https://i.imgur.com/yBSM45E.png)|
|0.03% true black|20.57% true black|
> Click either image for the full resolution!

