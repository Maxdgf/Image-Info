# Image-Info
A CLI application for getting information about image, images🖼️

| Demo screen |
| :---------------------|
| <img width="400" height="250" src=".github/screen.png"> |
| <a href=".github/demo.mp4">Demo video</a> |

## How to use?
App is easy to use, all you need to do is enter required parameterized commands, where the parameter is the path to your image🖼️ or other. Commands are listed before:

### GII - Get Image Info🖼️

##
    gii=(path_to_your_image)
- This command returns a simple data about your image (size, filename, file extension, file size, pixels count, rgb colors percent in every pixel)

### FEM - Fetch Exif Metadata📃

##
    fem=(path_to_your_image)
- This command returns Exif metadata from image and writes exif entries to output txt file (Exif supported in **.JPEG, .TIFF, .HEIF, .WEBP** images)

### IS = Images Size💿

##
    is=(target_image_extension)
- This command returns all images count, size with specific extension (specific extension entering without dot!) by scanning dirs🗃 where usually store images

### Help = Get Help❔️

##
    help
- This command returns help informatiin for user

### Exit = Exit App🟥

##
    exit
- Exit app
