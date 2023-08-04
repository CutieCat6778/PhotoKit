# PhotoKit
It is a file manager cli software, that helps photographers manage their files.

# How to use it?
### Arguments
```
  -from -f <PATH> 

  The original path of the files that you want to copy

  -to -t <PATH>

  The target path you want to copy the files

  -ext -e <EXTENSION>

  The extension name of the files you want to copy (currently it supports only files from Nikon and SONY, with filename format DSC_0001.RAW, DSC000001.ARW)

  -list -l <PATH>

  The path of the filename list you want to copy
```

### Copy files

1. Create a .txt file

2. Select and your favourite files and get the name. For example: DSC_0001.RAW -> 1

3. Write it in the .txt file, it will look like this.

```
  1
  3
  4
  10
  19
  20
  30
  44
```

4. Run the command

`./photo_kit -f ORIGIN_PATH -t TARGET_PATH -l LIST_PATH -e .RAW`

It will copy all the files you want.

# Contact

If there is a bug or feature request, just create a new issue. I will try to reply to it as soon as possible.

For futher contacts: [thinh@thinh.tech](mailto:thinh@thinh.tech)