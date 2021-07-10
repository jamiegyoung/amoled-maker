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