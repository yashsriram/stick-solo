# Motion planning
- Code is written in java, should work with JRE 8+
- src/ contains all source code
- jars/ contain all libraries bundled as jars
    - processing is used as a library

# Compilation (on linux)
- Open a terminal with current directory as the one containing this file
- Use `javac -Xlint:unchecked -cp "jars/processing/*:jars/ejml-v0.39-libs/*:jars/minim/*" -d build/ $(find -name "*.java")` to compile and put all output class files under build/
- Use `java -cp "build/:jars/processing/*:jars/ejml-v0.39-libs/*:jars/minim/*" <package>.<to>.<path>.<class>` to run any simulation
