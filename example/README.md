# Intro

There is a **photo_kit.exe** dummy file. If you read the `/target/list.txt` file, you can see, that I have 3 numbers written in the file. They are the filename sequence. The programm will only copy the files that has the same sequence.

# Action

`./photo_kit.exe -f ./origin/ -t ./target/ -e .ARW -l ./target/list.txt`